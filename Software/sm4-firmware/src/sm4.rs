use crate::can::{CANOpen, CANOpenMessage};
use crate::prelude::config::{CAN_ID, ENCODER_RESOLUTION, SENSE_R};
use crate::prelude::definitions::{Axis1, Axis2};
use crate::prelude::*;
use crate::state::DriverState;
use embedded_time::duration::Microseconds;
use hal::dma::StreamsTuple;
use hal::prelude::*;
use sm4_shared::prelude::*;
use stm32f4xx_hal as hal;

const SECOND: u32 = 168_000_000;

pub struct SM4 {
    leds: LEDs,
    usb: USBProtocol,
    can: CANOpen,
    monitoring: Monitoring,
    state: DriverState<{ ENCODER_RESOLUTION }>,
    axis1: Axis1,
    axis2: Axis2,
}

impl SM4 {
    pub fn init(device: hal::pac::Peripherals) -> Self {
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

        let (ref1, ref2) = initialize_current_ref(device.DAC, gpio.ref1, gpio.ref2);
        let timer1 = StepGeneratorTimer::init_tim8(device.TIM8, clocks);
        let timer2 = StepGeneratorTimer::init_tim1(device.TIM1, clocks);

        let sampling_period = Microseconds(1000);

        let axis1 = AxisMotionController::new(
            TMC2100::new(
                timer1,
                gpio.step1,
                gpio.dir1,
                ref1,
                SENSE_R,
                config::MICROSTEPS_PER_REV,
            ),
            StepCounterEncoder::tim5(device.TIM5, sampling_period),
            sampling_period,
        );
        let axis2 = AxisMotionController::new(
            TMC2100::new(
                timer2,
                gpio.step2,
                gpio.dir2,
                ref2,
                SENSE_R,
                config::MICROSTEPS_PER_REV,
            ),
            StepCounterEncoder::tim2(device.TIM2, sampling_period),
            sampling_period,
        );

        defmt::error!("init done");

        let mut state = DriverState::new();
        state.go_to_preoperational_if_needed();

        Self {
            can,
            leds,
            usb,
            monitoring: Monitoring::new(device.ADC1, gpio.battery_voltage, dma2.0),
            state,
            axis1,
            axis2,
        }
    }

    pub fn control(&mut self) {
        self.axis1.control(
            self.state.is_movement_blocked(),
            self.state.object_dictionary().axis1_mut(),
        );
        self.axis2.control(
            self.state.is_movement_blocked(),
            self.state.object_dictionary().axis2_mut(),
        );
    }

    pub fn ramp(&mut self) {
        self.axis1.ramp(
            self.state.is_movement_blocked(),
            self.state.object_dictionary().axis1_mut(),
        );
        self.axis2.ramp(
            self.state.is_movement_blocked(),
            self.state.object_dictionary().axis2_mut(),
        );
    }

    pub fn failsafe_tick(&mut self) {
        self.state.decrement_last_received_speed_command_counter();
    }

    pub fn heartbeat_tick(&mut self) {
        self.can.send(
            CANOpenMessage::NMTNodeMonitoring,
            &[u8::from(self.state.nmt_state())],
        )
    }

    pub fn blink_leds(&mut self) {
        self.leds.tick();
    }

    pub fn monitor(&mut self) {
        self.monitoring.poll();
    }

    pub fn monitoring_complete(&mut self) {
        self.monitoring.transfer_complete();
        self.state
            .object_dictionary()
            .set_battery_voltage(self.monitoring.get_battery_voltage());
        self.state
            .object_dictionary()
            .set_temperature(self.monitoring.get_temperature());
    }

    pub fn process_usb(&mut self) {
        self.usb.process_interrupt();
    }

    pub fn process_can(&mut self) {
        if let Some((message, frame)) = self.can.process_incoming_frame() {
            match message {
                CANOpenMessage::NMTNodeControl => {
                    crate::protocol::canopen::nmt_received(CAN_ID, &frame, &mut self.state);
                }
                CANOpenMessage::GlobalFailsafeCommand => {}
                CANOpenMessage::Sync => {
                    self.leds.signalize_sync();
                    crate::protocol::canopen::sync(&mut self.can, &mut self.state);
                }
                CANOpenMessage::Emergency => {}
                CANOpenMessage::TimeStamp => {}
                CANOpenMessage::RxPDO1 => {
                    crate::protocol::canopen::rx_pdo1(&frame, &mut self.state)
                }
                CANOpenMessage::RxPDO2 => {
                    crate::protocol::canopen::rx_pdo2(&frame, &mut self.state)
                }
                CANOpenMessage::RxPDO3 => {
                    crate::protocol::canopen::rx_pdo3(&frame, &mut self.state)
                }
                CANOpenMessage::RxPDO4 => {
                    crate::protocol::canopen::rx_pdo4(&frame, &mut self.state)
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

    pub const fn ramping_period() -> u32 {
        SECOND / 1000
    }

    pub const fn control_period() -> u32 {
        SECOND / 100
    }

    pub const fn failsafe_tick_period() -> u32 {
        SECOND / 10
    }

    pub const fn heartbeat_tick_period() -> u32 {
        SECOND / 2
    }
}
