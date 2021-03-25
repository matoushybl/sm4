use crate::board::*;
use crate::can::{CANOpen, CANOpenMessage};
use crate::current_reference::{initialize_current_ref, CurrentDACChannel};
use crate::leds::LEDs;
use crate::monitoring::Monitoring;
use crate::object_dictionary::ObjectDictionary;
use crate::step_counter::StepCounterEncoder;
use crate::step_timer::StepGeneratorTimer;
use crate::usb::USBProtocol;
use core::convert::TryFrom;
use embedded_time::duration::Microseconds;
use hal::dma::StreamsTuple;
use hal::prelude::*;
use sm4_shared::prelude::*;
use stm32f4xx_hal as hal;

const ENCODER_RESOLUTION: u16 = 16 * 200;

const CAN_ID: u8 = 0x01;

type Axis1Driver =
    TMC2100<StepGeneratorTimer<hal::pac::TIM8>, Step1, Dir1, CurrentDACChannel<CurrentRef1Channel>>;
type Axis2Driver =
    TMC2100<StepGeneratorTimer<hal::pac::TIM1>, Step2, Dir2, CurrentDACChannel<CurrentRef2Channel>>;
type Axis1Encoder = StepCounterEncoder<hal::pac::TIM5>;
type Axis2Encoder = StepCounterEncoder<hal::pac::TIM2>;

type Axis1 = AxisMotionController<Axis1Driver, Axis1Encoder>;
type Axis2 = AxisMotionController<Axis2Driver, Axis2Encoder>;

const SECOND: u32 = 168_000_000;

const SPEED_COMMAND_RESET_INTERVAL: u8 = 10; // ticks of a failsafe timer

#[derive(Copy, Clone)]
pub struct DriverState {
    nmt_state: NMTState,
    object_dictionary: ObjectDictionary,
    last_received_speed_command_down_counter: u8,
}

impl DriverState {
    pub fn new(encoder_resolution: u16) -> Self {
        Self {
            nmt_state: NMTState::default(),
            object_dictionary: ObjectDictionary::new(encoder_resolution),
            last_received_speed_command_down_counter: 0,
        }
    }

    pub fn go_to_preoperational_if_needed(&mut self) {
        if self.nmt_state == NMTState::BootUp {
            self.nmt_state = NMTState::PreOperational;
        }
    }

    pub fn go_to_operational(&mut self) {
        self.nmt_state = NMTState::Operational;
    }

    pub fn go_to_stopped(&mut self) {
        self.nmt_state = NMTState::Stopped;
    }

    pub fn go_to_preoperational(&mut self) {
        self.nmt_state = NMTState::PreOperational;
    }

    pub fn is_movement_blocked(&self) -> bool {
        self.nmt_state != NMTState::Operational
            || self.last_received_speed_command_down_counter == 0
    }

    pub fn decrement_last_received_speed_command_counter(&mut self) {
        if self.last_received_speed_command_down_counter != 0 {
            self.last_received_speed_command_down_counter -= 1;
        }
    }

    pub fn invalidate_last_received_speed_command_counter(&mut self) {
        self.last_received_speed_command_down_counter = SPEED_COMMAND_RESET_INTERVAL;
    }

    pub fn object_dictionary(&mut self) -> &mut ObjectDictionary {
        &mut self.object_dictionary
    }
}

pub struct SM4 {
    leds: LEDs,
    usb: USBProtocol,
    can: CANOpen,
    monitoring: Monitoring,
    state: DriverState,
    axis1: Axis1,
    axis2: Axis2,
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

        let (ref1, ref2) = initialize_current_ref(device.DAC, gpio.ref1, gpio.ref2);
        let timer1 = StepGeneratorTimer::init_tim8(device.TIM8, clocks);
        let timer2 = StepGeneratorTimer::init_tim1(device.TIM1, clocks);

        let sampling_period = Microseconds(10000);

        let axis1 = AxisMotionController::new(
            TMC2100::new(timer1, gpio.step1, gpio.dir1, ref1, SENSE_R),
            StepCounterEncoder::tim5(device.TIM5, sampling_period, ENCODER_RESOLUTION),
            sampling_period,
        );
        let axis2 = AxisMotionController::new(
            TMC2100::new(timer2, gpio.step2, gpio.dir2, ref2, SENSE_R),
            StepCounterEncoder::tim2(device.TIM2, sampling_period, ENCODER_RESOLUTION),
            sampling_period,
        );

        defmt::error!("init done");

        let mut state = DriverState::new(ENCODER_RESOLUTION);
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

    pub fn sample(&mut self) {
        self.axis1.control(
            self.state.is_movement_blocked(),
            self.state.object_dictionary.axis1_mut(),
        );
        self.axis2.control(
            self.state.is_movement_blocked(),
            self.state.object_dictionary.axis2_mut(),
        );
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
                    let pdo = TxPDO1 {
                        battery_voltage: (self.state.object_dictionary().battery_voltage() * 1000.0)
                            as u16,
                        temperature: (self.state.object_dictionary().temperature() * 10.0) as u16,
                    };
                    let mut buffer = [0u8; 8];
                    let size = pdo.to_raw(&mut buffer).unwrap();
                    self.can.send(CANOpenMessage::TxPDO1, &buffer[..size]);

                    let pdo = TxPDO2 {
                        axis1_velocity: self
                            .state
                            .object_dictionary()
                            .axis1()
                            .actual_velocity()
                            .get_rps(),
                        axis2_velocity: self
                            .state
                            .object_dictionary()
                            .axis2()
                            .actual_velocity()
                            .get_rps(),
                    };
                    let size = pdo.to_raw(&mut buffer).unwrap();
                    self.can.send(CANOpenMessage::TxPDO2, &buffer[..size]);

                    let pdo = TxPDO3 {
                        revolutions: self
                            .state
                            .object_dictionary()
                            .axis1()
                            .actual_position()
                            .get_revolutions(),
                        angle: self
                            .state
                            .object_dictionary()
                            .axis1()
                            .actual_position()
                            .get_angle() as u32,
                    };
                    let size = pdo.to_raw(&mut buffer).unwrap();
                    self.can.send(CANOpenMessage::TxPDO3, &buffer[..size]);

                    let pdo = TxPDO4 {
                        revolutions: self
                            .state
                            .object_dictionary()
                            .axis2()
                            .actual_position()
                            .get_revolutions(),
                        angle: self
                            .state
                            .object_dictionary()
                            .axis2()
                            .actual_position()
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
                            .axis1_mut()
                            .set_mode(pdo.axis1_mode);
                        self.state
                            .object_dictionary()
                            .axis2_mut()
                            .set_mode(pdo.axis2_mode);
                        self.state
                            .object_dictionary()
                            .axis1_mut()
                            .set_enabled(pdo.axis1_enabled);
                        self.state
                            .object_dictionary()
                            .axis2_mut()
                            .set_enabled(pdo.axis2_enabled);
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
                            .axis1_mut()
                            .set_target_velocity(Velocity::new(pdo.axis1_velocity));
                        self.state
                            .object_dictionary()
                            .axis2_mut()
                            .set_target_velocity(Velocity::new(pdo.axis2_velocity));
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
                            .axis1_mut()
                            .set_target_position(Position::new(
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
                            .axis2_mut()
                            .set_target_position(Position::new(
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
