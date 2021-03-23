#![no_std]

pub mod board;
pub mod can;
pub mod current_reference;
mod driver;
pub mod leds;
pub mod monitoring;
mod object_dictionary;
pub mod step_counter;
pub mod step_timer;
pub mod usb;

use core::sync::atomic::{AtomicUsize, Ordering};

use crate::board::{CurrentRef1Channel, CurrentRef2Channel, Dir1, Dir2, Step1, Step2, GPIO};
use crate::can::{CANOpen, CANOpenMessage};
use crate::current_reference::{initialize_current_ref, CurrentDACChannel};
use crate::leds::LEDs;
use crate::monitoring::Monitoring;
use crate::step_timer::StepGeneratorTimer;
use crate::usb::USBProtocol;
use core::convert::TryFrom;
use defmt_rtt as _; // global logger
use driver::{DriverState, DualAxisDriver};
use embedded_time::duration::Microseconds;
use hal::prelude::*;
use panic_probe as _;
use sm4_shared::canopen::*;
use sm4_shared::encoder::{Position, Speed};
use sm4_shared::tmc2100::TMC2100;
use sm4_shared::StepperDriver;
use step_counter::StepCounterEncoder;
use stm32f4xx_hal as hal;
use stm32f4xx_hal::dma::StreamsTuple; // memory layout

const ENCODER_RESOLUTION: u16 = 16 * 200;

const CAN_ID: u8 = 0x01;

type Motor1Driver =
    TMC2100<StepGeneratorTimer<hal::pac::TIM8>, Step1, Dir1, CurrentDACChannel<CurrentRef1Channel>>;
type Motor2Driver =
    TMC2100<StepGeneratorTimer<hal::pac::TIM1>, Step2, Dir2, CurrentDACChannel<CurrentRef2Channel>>;
type Motor1Encoder = StepCounterEncoder<hal::pac::TIM5>;
type Motor2Encoder = StepCounterEncoder<hal::pac::TIM2>;

const SECOND: u32 = 168_000_000;

pub struct SM4 {
    leds: LEDs,
    usb: USBProtocol,
    can: CANOpen,
    monitoring: Monitoring,
    state: DriverState,
    driver: DualAxisDriver<Motor1Driver, Motor2Driver, Motor1Encoder, Motor2Encoder>,
}

impl SM4 {
    pub fn init(device: hal::pac::Peripherals, mut core: rtic::Peripherals) -> Self {
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
        let timer1 = StepGeneratorTimer::init_tim8(device.TIM8, clocks);
        let timer2 = StepGeneratorTimer::init_tim1(device.TIM1, clocks);

        let mut driver1 = TMC2100::new(timer1, gpio.step1, gpio.dir1, ref1, board::SENSE_R);
        let driver2 = TMC2100::new(timer2, gpio.step2, gpio.dir2, ref2, board::SENSE_R);

        driver1.set_current(0.4);

        let sampling_period = Microseconds(10000);

        let counter_encoder1 =
            StepCounterEncoder::tim5(device.TIM5, sampling_period, ENCODER_RESOLUTION);
        let counter_encoder2 =
            StepCounterEncoder::tim2(device.TIM2, sampling_period, ENCODER_RESOLUTION);

        let driver = DualAxisDriver::new(
            driver1,
            driver2,
            counter_encoder1,
            counter_encoder2,
            sampling_period,
        );

        defmt::error!("init done");

        Self {
            can,
            leds,
            usb,
            monitoring,
            state: DriverState::new(16 * 200),
            driver,
        }
    }

    pub fn sample(&mut self) {
        self.driver.sample(&mut self.state);
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

    pub fn process_usb(&mut self) {
        self.usb.process_interrupt();
    }

    // TODO send NMT heartbeat
    pub fn process_can(&mut self) {
        if let Some((message, frame)) = self.can.process_incoming_frame() {
            match message {
                CANOpenMessage::NMTNodeControl => {
                    if frame.dlc() != 2 {
                        defmt::error!("Malformed NMT node control data received.");
                        return;
                    }
                    if frame.data().unwrap()[1] != CAN_ID {
                        return;
                    }
                    match NMTRequestedState::try_from(frame.data().unwrap()[0]) {
                        Ok(state) => match state {
                            NMTRequestedState::Operational => {
                                self.state.go_to_operational();
                            }
                            NMTRequestedState::Stopped => {
                                self.state.go_to_stopped();
                            }
                            NMTRequestedState::PreOperational => {
                                self.state.go_to_preoperational();
                            }
                            NMTRequestedState::ResetNode => {
                                unimplemented!()
                            }
                            NMTRequestedState::ResetCommunication => {
                                unimplemented!()
                            }
                        },
                        Err(_) => {
                            defmt::error!("Invalid NMT requested state received.");
                        }
                    }
                }
                CANOpenMessage::GlobalFailsafeCommand => {}
                CANOpenMessage::Sync => {
                    // TODO move monitoring values to object dictionary as well
                    let pdo = TxPDO1 {
                        battery_voltage: (self.monitoring.get_battery_voltage() * 1000.0) as u16,
                        temperature: (self.monitoring.get_temperature() * 10.0) as u16,
                    };
                    let mut buffer = [0u8; 8];
                    let size = pdo.to_raw(&mut buffer).unwrap();
                    self.can.send(CANOpenMessage::TxPDO1, &buffer[..size]);

                    let pdo = TxPDO2 {
                        axis1_velocity: self
                            .state
                            .object_dictionary()
                            .axis1_actual_velocity()
                            .get_rps(),
                        axis2_velocity: self
                            .state
                            .object_dictionary()
                            .axis2_actual_velocity()
                            .get_rps(),
                    };
                    let size = pdo.to_raw(&mut buffer).unwrap();
                    self.can.send(CANOpenMessage::TxPDO2, &buffer[..size]);

                    let pdo = TxPDO3 {
                        revolutions: self
                            .state
                            .object_dictionary()
                            .axis1_actual_position()
                            .get_revolutions(),
                        angle: self
                            .state
                            .object_dictionary()
                            .axis1_actual_position()
                            .get_angle() as u32,
                    };
                    let size = pdo.to_raw(&mut buffer).unwrap();
                    self.can.send(CANOpenMessage::TxPDO3, &buffer[..size]);

                    let pdo = TxPDO4 {
                        revolutions: self
                            .state
                            .object_dictionary()
                            .axis2_actual_position()
                            .get_revolutions(),
                        angle: self
                            .state
                            .object_dictionary()
                            .axis2_actual_position()
                            .get_angle() as u32,
                    };
                    let size = pdo.to_raw(&mut buffer).unwrap();
                    self.can.send(CANOpenMessage::TxPDO4, &buffer[..size]);

                    defmt::error!("sync received");
                }
                CANOpenMessage::Emergency => {}
                CANOpenMessage::TimeStamp => {}
                CANOpenMessage::RxPDO1 => {
                    if frame.data().is_none() {
                        defmt::warn!("Invalid RxPDO1 received.");
                        return;
                    }
                    if let Ok(pdo) = RxPDO1::try_from(frame.data().unwrap().as_ref()) {
                        self.state
                            .object_dictionary()
                            .set_axis1_mode(pdo.axis1_mode);
                        self.state
                            .object_dictionary()
                            .set_axis2_mode(pdo.axis2_mode);
                        self.state
                            .object_dictionary()
                            .set_axis2_enabled(pdo.axis1_enabled);
                        self.state
                            .object_dictionary()
                            .set_axis2_enabled(pdo.axis2_enabled);
                    } else {
                        defmt::warn!("Malformed RxPDO1 received.");
                    }
                }
                CANOpenMessage::RxPDO2 => {
                    if frame.data().is_none() {
                        defmt::warn!("Invalid RxPDO2 received.");
                        return;
                    }
                    if let Ok(pdo) = RxPDO2::try_from(frame.data().unwrap().as_ref()) {
                        self.state
                            .object_dictionary()
                            .set_axis1_target_velocity(Speed::new(pdo.axis1_velocity));
                        self.state
                            .object_dictionary()
                            .set_axis2_target_velocity(Speed::new(pdo.axis2_velocity));
                    } else {
                        defmt::warn!("Malformed RxPDO2 received.");
                    }
                }
                CANOpenMessage::RxPDO3 => {
                    if frame.data().is_none() {
                        defmt::warn!("Invalid RxPDO1 received.");
                        return;
                    }
                    if let Ok(pdo) = RxPDO3::try_from(frame.data().unwrap().as_ref()) {
                        self.state
                            .object_dictionary()
                            .set_axis1_target_position(Position::new(
                                ENCODER_RESOLUTION,
                                pdo.revolutions,
                                pdo.angle as u16,
                            ));
                    } else {
                        defmt::warn!("Malformed RxPDO3 received.");
                    }
                }
                CANOpenMessage::RxPDO4 => {
                    if frame.data().is_none() {
                        defmt::warn!("Invalid RxPDO4 received.");
                        return;
                    }
                    if let Ok(pdo) = RxPDO4::try_from(frame.data().unwrap().as_ref()) {
                        self.state
                            .object_dictionary()
                            .set_axis2_target_position(Position::new(
                                ENCODER_RESOLUTION,
                                pdo.revolutions,
                                pdo.angle as u16,
                            ));
                    } else {
                        defmt::warn!("Malformed RxPDO4 received.");
                    }
                }
                CANOpenMessage::RxSDO => {}
                _ => {}
            }
        }
    }

    pub const fn blink_period() -> u32 {
        SECOND / 100
    }

    pub const fn monitoring_period() -> u32 {
        SECOND / 10
    }

    pub const fn sampling_period() -> u32 {
        SECOND / 100
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
