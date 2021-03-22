use sm4_shared::{
    canopen::NMTState, encoder::Encoder, encoder::Speed, AxisMode, Direction, PSDController,
    StepperDriver,
};

use crate::object_dictionary::ObjectDictionary;
use embedded_time::duration::Microseconds;
use sm4_shared::encoder::Position;
use sm4_shared::ramp::TrapRampGen;

const SPEED_COMMAND_RESET_INTERVAL: u8 = 10; // ticks of a failsafe timer

#[derive(Copy, Clone)]
pub struct DriverState {
    nmt_state: NMTState,
    object_dictionary: ObjectDictionary,
    last_received_speed_command_down_counter: u8,
}

impl DriverState {
    pub fn new(encoder_resolution: u16) -> Self {
        let mut s = Self {
            nmt_state: NMTState::default(),
            object_dictionary: ObjectDictionary::new(encoder_resolution),
            last_received_speed_command_down_counter: 0,
        };

        s.object_dictionary
            .set_axis1_target_velocity(Speed::new(1.0));
        s
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
        // if self.is_movement_blocked() || !self.object_dictionary.axis1_enabled() {
        //     Speed::zero()
        // } else {
        self.object_dictionary.axis1_target_velocity()
        // }
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
    axis1_velocity_controller: PSDController,
    axis2_velocity_controller: PSDController,
    axis1_position_controller: PSDController,
    axis2_position_controller: PSDController,
    axis1_ramp_generator: TrapRampGen,
    axis2_ramp_generator: TrapRampGen,
}

impl<D1: StepperDriver, D2: StepperDriver, E1: Encoder, E2: Encoder>
    DualAxisDriver<D1, D2, E1, E2>
{
    pub fn new(
        axis1_driver: D1,
        axis2_driver: D2,
        axis1_encoder: E1,
        axis2_encoder: E2,
        sampling_period: Microseconds,
    ) -> Self {
        let frequency = 1.0 / (sampling_period.0 as f32 / 1_000_000.0);
        Self {
            axis1_driver,
            axis2_driver,
            axis1_encoder,
            axis2_encoder,
            axis1_velocity_controller: PSDController::new(sampling_period),
            axis2_velocity_controller: PSDController::new(sampling_period),
            axis1_position_controller: PSDController::new(sampling_period),
            axis2_position_controller: PSDController::new(sampling_period),
            axis1_ramp_generator: TrapRampGen::new(frequency),
            axis2_ramp_generator: TrapRampGen::new(frequency),
        }
    }

    pub(crate) fn sample(&mut self, state: &mut DriverState) {
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

        // TODO feedback control to object dictionary
        Self::update_axis(
            &mut self.axis1_encoder,
            &mut self.axis1_driver,
            &mut self.axis1_velocity_controller,
            &mut self.axis1_position_controller,
            &mut self.axis1_ramp_generator,
            &state.object_dictionary().axis1_mode(),
            &state.object_dictionary().axis1_target_velocity(),
            &state.object_dictionary().axis1_actual_velocity(),
            &state.object_dictionary().axis1_velocity_p(),
            &state.object_dictionary().axis1_velocity_s(),
            &state.object_dictionary().axis1_velocity_d(),
            &state.object_dictionary.axis1_acceleration(),
            false,
        );

        Self::update_axis(
            &mut self.axis2_encoder,
            &mut self.axis2_driver,
            &mut self.axis2_velocity_controller,
            &mut self.axis2_position_controller,
            &mut self.axis2_ramp_generator,
            &state.object_dictionary().axis2_mode(),
            &state.object_dictionary().axis2_target_velocity(),
            &state.object_dictionary().axis2_actual_velocity(),
            &state.object_dictionary().axis2_velocity_p(),
            &state.object_dictionary().axis2_velocity_s(),
            &state.object_dictionary().axis2_velocity_d(),
            &state.object_dictionary.axis2_acceleration(),
            false,
        );
    }

    // TODO add current manipulation
    pub fn update_axis(
        encoder: &mut dyn Encoder,
        driver: &mut dyn StepperDriver,
        velocity_controller: &mut PSDController,
        position_controller: &mut PSDController,
        ramp_generator: &mut TrapRampGen,
        mode: &AxisMode,
        target_velocity: &Speed,
        actual_velocity: &Speed,
        p: &f32,
        s: &f32,
        d: &f32,
        acceleration: &f32,
        velocity_feedback_control_enabled: bool,
    ) {
        let target_velocity = match mode {
            AxisMode::Velocity => *target_velocity,
            AxisMode::Position => {}
        };

        let axis_velocity_action = if velocity_feedback_control_enabled {
            velocity_controller.sample(
                &target_velocity.get_rps(),
                &actual_velocity.get_rps(),
                p,
                s,
                d,
                &3.0,
            )
        } else {
            target_velocity.get_rps()
        };
        let axis_new_direction = Direction::from(axis_velocity_action);

        if Direction::from(actual_velocity.get_rps()) != axis_new_direction {
            encoder.notify_direction_changed(axis_new_direction);
        }

        driver.set_output_frequency(ramp_generator.generate(axis_velocity_action, *acceleration));
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
