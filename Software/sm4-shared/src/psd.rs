use crate::float;
use embedded_time::duration::Microseconds;

#[derive(Copy, Clone, Default)]
pub struct ControllerSettings {
    proportional: f32,
    integral: f32,
    derivative: f32,
    max_output_amplitude: f32,
}

impl ControllerSettings {
    pub fn new(
        proportional: f32,
        integral: f32,
        derivative: f32,
        max_output_amplitude: f32,
    ) -> Self {
        Self {
            proportional,
            integral,
            derivative,
            max_output_amplitude,
        }
    }
}

#[derive(Copy, Clone)]
pub struct PSDController {
    sum: f32,
    previous: f32,
    sampling_period: f32, // seconds
}

impl PSDController {
    pub fn new(sampling_period: Microseconds) -> Self {
        Self {
            sum: 0.0,
            previous: 0.0,
            sampling_period: sampling_period.0 as f32 / 1_000_000.0,
        }
    }

    pub fn sample(&mut self, desired: &f32, actual: &f32, settings: &ControllerSettings) -> f32 {
        let error = desired - actual;

        self.sum += error * self.sampling_period;
        self.sum = float::fmaxf(
            float::fminf(self.sum, settings.max_output_amplitude),
            -settings.max_output_amplitude,
        );

        let action = error * settings.proportional
            + settings.derivative * (error - self.previous) / self.sampling_period
            + settings.integral * self.sum;
        self.previous = error;

        float::fmaxf(
            float::fminf(action, settings.max_output_amplitude),
            -settings.max_output_amplitude,
        )
    }
}
