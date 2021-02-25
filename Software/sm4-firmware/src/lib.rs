#![no_std]

pub mod board;
pub mod can;
pub mod current_reference;
pub mod direction;
pub mod leds;
pub mod monitoring;
pub mod ramp;
pub mod step_counter;
pub mod step_timer;
pub mod usb;

use core::sync::atomic::{AtomicUsize, Ordering};

use crate::board::{CurrentRef1Channel, CurrentRef2Channel, Dir1, Dir2, GPIO};
use crate::can::{CANOpen, CANOpenMessage, NMTState};
use crate::current_reference::{initialize_current_ref, Reference};
use crate::direction::DirectionPin;
use crate::leds::LEDs;
use crate::monitoring::Monitoring;
use crate::ramp::{DriverWithGen, TrapRampGen};
use crate::step_counter::Counter;
use crate::step_timer::ControlTimer;
use crate::usb::USBProtocol;
use core::convert::TryFrom;
use cortex_m::peripheral::DWT;
use defmt_rtt as _; // global logger
use panic_probe as _;
use sm4_shared::canopen::RxPDO1;
use sm4_shared::{Driver, Motor1, Motor2};
use stm32f4xx_hal as hal;
use stm32f4xx_hal::dma::StreamsTuple;
use stm32f4xx_hal::gpio::GpioExt;
use stm32f4xx_hal::rcc::RccExt;
use stm32f4xx_hal::time::U32Ext; // memory layout

const CAN_ID: u8 = 0x01;

type Motor1Driver = Driver<
    Motor1,
    ControlTimer<hal::pac::TIM8>,
    DirectionPin<Dir1>,
    Counter<hal::pac::TIM5>,
    Reference<CurrentRef1Channel>,
>;

type Motor2Driver = Driver<
    Motor2,
    ControlTimer<hal::pac::TIM1>,
    DirectionPin<Dir2>,
    Counter<hal::pac::TIM2>,
    Reference<CurrentRef2Channel>,
>;

const SECOND: u32 = 168_000_000;

/// The object dictionary struct represents the global state of the driver
#[derive(Copy, Clone, Default)]
pub struct ObjectDictionary {
    desired_speed1: f32,
    desired_speed2: f32,
}

impl ObjectDictionary {
    pub fn set_desired_speed1(&mut self, speed: f32) {
        self.desired_speed1 = speed;
    }

    pub fn set_desired_speed2(&mut self, speed: f32) {
        self.desired_speed2 = speed;
    }
}

pub struct SM4 {
    leds: LEDs,
    usb: USBProtocol,
    can: CANOpen,
    driver1: DriverWithGen<Motor1Driver>,
    driver2: DriverWithGen<Motor2Driver>,
    monitoring: Monitoring,
    object_dictionary: ObjectDictionary,
    state: DriverState,
}

const SPEED_COMMAND_RESET_INTERVAL: u8 = 10; // ticks of a failsafe timer

#[derive(Copy, Clone, Default)]
struct DriverState {
    nmt_state: NMTState,
    last_received_speed_command_down_counter: u8,
}

impl DriverState {
    pub fn is_movement_blocked(&self) -> bool {
        self.nmt_state != NMTState::Operational
            || self.last_received_speed_command_down_counter == 0
    }

    pub fn decrement_last_received_speed_command_counter(&mut self) {
        self.last_received_speed_command_down_counter -= 1;
    }

    pub fn invalidate_last_received_speed_command_counter(&mut self) {
        self.last_received_speed_command_down_counter = SPEED_COMMAND_RESET_INTERVAL;
    }
}

impl SM4 {
    pub fn init(device: hal::pac::Peripherals, mut core: rtic::Peripherals) -> Self {
        core.DCB.enable_trace();
        DWT::unlock();
        core.DWT.enable_cycle_counter();

        let clocks = device
            .RCC
            .constrain()
            .cfgr
            .use_hse(25.mhz())
            .sysclk(168.mhz())
            .hclk(168.mhz())
            .pclk1(42.mhz())
            .pclk2(84.mhz())
            .require_pll48clk()
            .freeze();

        let gpio = GPIO::configure(
            device.GPIOA.split(),
            device.GPIOB.split(),
            device.GPIOC.split(),
        );

        let usb = USBProtocol::new(
            device.OTG_FS_GLOBAL,
            device.OTG_FS_DEVICE,
            device.OTG_FS_PWRCLK,
            gpio.usb_minus,
            gpio.usb_plus,
            clocks,
        );

        let can = CANOpen::new(
            hal::can::Can::new(device.CAN1, (gpio.can_tx, gpio.can_rx)),
            CAN_ID,
        );

        let dma2 = StreamsTuple::new(device.DMA2);
        let mut leds = LEDs::new(gpio.status_led, gpio.error_led);
        leds.signalize_sync();

        let monitoring = Monitoring::new(device.ADC1, gpio.battery_voltage, dma2.0);

        let (ref1, ref2) = initialize_current_ref(device.DAC, gpio.ref1, gpio.ref2);
        let dir1 = DirectionPin::dir1(gpio.dir1);
        let dir2 = DirectionPin::dir2(gpio.dir2);
        let timer1 = ControlTimer::init_tim8(device.TIM8, clocks);
        let timer2 = ControlTimer::init_tim1(device.TIM1, clocks);
        let counter1 = Counter::tim5(device.TIM5);
        let counter2 = Counter::tim2(device.TIM2);

        let driver1 = Driver::new(timer1, dir1, counter1, ref1);
        let driver2 = Driver::new(timer2, dir2, counter2, ref2);

        let mut driver1 = DriverWithGen::new(driver1, 3.0, TrapRampGen::new(2.0, 10.0));
        let driver2 = DriverWithGen::new(driver2, 3.0, TrapRampGen::new(2.0, 10.0));

        driver1.set_speed(3.0);

        defmt::error!("init done");

        Self {
            leds,
            usb,
            driver1,
            driver2,
            can,
            monitoring,
            object_dictionary: ObjectDictionary::default(),
            state: DriverState::default(),
        }
    }

    pub fn run(&mut self) {
        if self.state.nmt_state == NMTState::BootUp {
            self.state.nmt_state = NMTState::PreOperational;
        }
        let (speed1, speed2) = if self.state.is_movement_blocked() {
            (0.0, 0.0)
        } else {
            (
                self.object_dictionary.desired_speed1,
                self.object_dictionary.desired_speed2,
            )
        };
        self.driver1.set_speed(speed1);
        self.driver2.set_speed(speed2);
    }

    pub fn blink_leds(&mut self) {
        self.leds.tick();
    }

    pub fn monitor(&mut self) {
        self.monitoring.poll();
    }

    pub fn monitoring_complete(&mut self) {
        self.monitoring.transfer_complete();
    }

    pub fn update_failsafe(&mut self) {
        self.state.decrement_last_received_speed_command_counter();
    }

    pub fn ramp(&mut self) {
        self.driver1.update();
        self.driver2.update();
    }

    pub fn process_usb(&mut self) {
        self.usb.process_interrupt();
    }

    pub fn process_can(&mut self) {
        if let Some((message, frame)) = self.can.process_incoming_frame() {
            match message {
                CANOpenMessage::NMTNodeControl => {}
                CANOpenMessage::GlobalFailsafeCommand => {}
                CANOpenMessage::Sync => {
                    defmt::error!("sync received");
                }
                CANOpenMessage::Emergency => {}
                CANOpenMessage::TimeStamp => {}
                CANOpenMessage::TxPDO1 => {}
                CANOpenMessage::RxPDO1 => {
                    if frame.data().is_none() {
                        defmt::warn!("Invalid RxPDO1 received.");
                        return;
                    }
                    if let Ok(pdo) = RxPDO1::try_from(frame.data().unwrap().as_ref()) {
                        defmt::error!("speed: {:?}", pdo.driver1_speed);
                        self.object_dictionary.set_desired_speed1(pdo.driver1_speed);
                        self.object_dictionary.set_desired_speed2(pdo.driver2_speed);
                        self.state.invalidate_last_received_speed_command_counter();
                    } else {
                        defmt::warn!("Malformed RxPDO1 received.");
                    }
                }
                CANOpenMessage::TxPDO2 => {}
                CANOpenMessage::RxPDO2 => {}
                CANOpenMessage::TxPDO3 => {}
                CANOpenMessage::RxPDO3 => {}
                CANOpenMessage::TxPDO4 => {}
                CANOpenMessage::RxPDO4 => {}
                CANOpenMessage::TxSDO => {}
                CANOpenMessage::RxSDO => {}
                CANOpenMessage::NMTNodeMonitoring => {}
            }
        }
    }

    pub const fn blink_period() -> u32 {
        SECOND / 10
    }

    pub const fn monitoring_period() -> u32 {
        SECOND / 10
    }

    pub const fn ramping_period() -> u32 {
        SECOND / 20
    }

    pub const fn failsafe_period() -> u32 {
        SECOND / 10
    }
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

#[defmt::timestamp]
fn timestamp() -> u64 {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    // NOTE(no-CAS) `timestamps` runs with interrupts disabled
    let n = COUNT.load(Ordering::Relaxed);
    COUNT.store(n + 1, Ordering::Relaxed);
    n as u64
}
