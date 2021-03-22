use sm4_shared::{canopen::NMTState, encoder::Encoder, encoder::Speed, AxisMode, StepperDriver};

use crate::object_dictionary::ObjectDictionary;
use sm4_shared::ramp::TrapRampGen;

const SPEED_COMMAND_RESET_INTERVAL: u8 = 10; // ticks of a failsafe timer

pub trait Driver {
    type State;
    fn sample(&mut self, state: &mut Self::State);
}

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

    pub fn is_movement_blocked(&self) -> bool {
        self.nmt_state != NMTState::Operational
            || self.last_received_speed_command_down_counter == 0
    }

    pub fn axis1_target_velocity(&self) -> Speed {
        if self.is_movement_blocked() || !self.object_dictionary.axis1_enabled() {
            Speed::zero()
        } else {
            self.object_dictionary.axis1_target_velocity()
        }
    }

    pub fn axis2_target_velocity(&self) -> Speed {
        if self.is_movement_blocked() || !self.object_dictionary.axis2_enabled() {
            Speed::zero()
        } else {
            self.object_dictionary.axis2_target_velocity()
        }
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

pub struct DualAxisDriver<D1: StepperDriver, D2: StepperDriver, E1: Encoder, E2: Encoder> {
    axis1_driver: D1,
    axis2_driver: D2,
    axis1_encoder: E1,
    axis2_encoder: E2,
    axis1_ramp_generator: TrapRampGen,
    axis2_ramp_generator: TrapRampGen,
}

impl<D1: StepperDriver, D2: StepperDriver, E1: Encoder, E2: Encoder>
    DualAxisDriver<D1, D2, E1, E2>
{
    pub fn new(axis1_driver: D1, axis2_driver: D2, axis1_encoder: E1, axis2_encoder: E2) -> Self {
        Self {
            axis1_driver,
            axis2_driver,
            axis1_encoder,
            axis2_encoder,
            axis1_ramp_generator: TrapRampGen::new(1000.0),
            axis2_ramp_generator: TrapRampGen::new(1000.0),
        }
    }

    pub fn decompose(self) -> (D1, D2, E1, E2) {
        (
            self.axis1_driver,
            self.axis2_driver,
            self.axis1_encoder,
            self.axis2_encoder,
        )
    }
}

impl<D1: StepperDriver, D2: StepperDriver, E1: Encoder, E2: Encoder> Driver
    for DualAxisDriver<D1, D2, E1, E2>
{
    type State = DriverState;

    fn sample(&mut self, state: &mut DriverState) {
        state.go_to_preoperational_if_needed();
        state.decrement_last_received_speed_command_counter();

        self.axis1_encoder.sample();
        self.axis2_encoder.sample();

        state
            .object_dictionary()
            .set_axis1_actual_velocity(self.axis1_encoder.get_speed());
        state
            .object_dictionary()
            .set_axis2_actual_velocity(self.axis2_encoder.get_speed());

        state
            .object_dictionary()
            .set_axis1_actual_position(self.axis1_encoder.get_position());
        state
            .object_dictionary()
            .set_axis2_actual_position(self.axis2_encoder.get_position());

        // TODO add PSD control
        let axis1_target_velocity = match state.object_dictionary().axis1_mode() {
            AxisMode::Velocity => state.axis1_target_velocity(),
            AxisMode::Position => Speed::zero(),
        };

        let axis2_target_velocity = match state.object_dictionary().axis2_mode() {
            AxisMode::Velocity => state.axis2_target_velocity(),
            AxisMode::Position => Speed::zero(),
        };

        // TODO change output current
        // TODO update encoder direction

        self.axis1_driver
            .set_output_frequency(self.axis1_ramp_generator.generate(
                axis1_target_velocity.get_rps(),
                state.object_dictionary().axis1_acceleration(),
            ));

        self.axis2_driver
            .set_output_frequency(self.axis2_ramp_generator.generate(
                axis2_target_velocity.get_rps(),
                state.object_dictionary().axis2_acceleration(),
            ));
    }
}
