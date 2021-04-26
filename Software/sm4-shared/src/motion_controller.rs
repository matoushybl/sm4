use crate::prelude::*;
use num_traits::Float;

use embedded_time::duration::Microseconds;

pub struct AxisMotionController<D: StepperDriver, E: Encoder> {
    driver: D,
    encoder: E,
    velocity_controller: PSDController,
    position_controller: PSDController,
    ramp_generator: TrapRampGen,
}

impl<D: StepperDriver, E: Encoder> AxisMotionController<D, E> {
    pub fn new(driver: D, encoder: E, sampling_period: Microseconds) -> Self {
        Self {
            driver,
            encoder,
            velocity_controller: PSDController::new(sampling_period),
            position_controller: PSDController::new(sampling_period),
            ramp_generator: TrapRampGen::new(sampling_period),
        }
    }

    pub fn control(&mut self, global_disable: bool, dictionary: &mut AxisDictionary) {
        self.encoder.sample();
        dictionary.set_actual_velocity(self.encoder.get_velocity());
        dictionary.set_actual_position(self.encoder.get_position());

        let target_velocity = if dictionary.enabled() && !global_disable {
            match dictionary.mode() {
                AxisMode::Velocity => dictionary.target_velocity(),
                AxisMode::Position => Velocity::zero(),
            }
        } else {
            Velocity::zero()
        };

        let axis_velocity_action = if dictionary.velocity_feedback_control_enabled() {
            self.velocity_controller.sample(
                &target_velocity.get_rps(),
                &dictionary.actual_velocity().get_rps(),
                &dictionary.velocity_controller_settings(),
            )
        } else {
            target_velocity.get_rps()
        };
        let axis_new_direction = Direction::from(axis_velocity_action);

        if Direction::from(dictionary.actual_velocity().get_rps()) != axis_new_direction {
            self.encoder.notify_direction_changed(axis_new_direction);
        }

        let output_frequency = self
            .ramp_generator
            .generate(axis_velocity_action, dictionary.acceleration());

        self.driver.set_output_frequency(output_frequency);
        let current = if output_frequency.abs() < 0.1 {
            dictionary.current().standstill_current()
        } else if (output_frequency - axis_velocity_action).abs() < f32::EPSILON {
            dictionary.current().constant_velocity_current()
        } else {
            dictionary.current().accelerating_current()
        };
        self.driver.set_current(current);
    }

    pub fn decompose(self) -> (D, E) {
        (self.driver, self.encoder)
    }
}
