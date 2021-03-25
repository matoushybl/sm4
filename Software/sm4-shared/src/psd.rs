use crate::float;
use embedded_time::duration::Microseconds;

pub struct ControllerSettings {
    proportional: f32,
    integral: f32,
    derivative: f32,
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

    pub fn sample(
        &mut self,
        desired: &f32,
        actual: &f32,
        settings: &ControllerSettings,
        limit: &f32,
    ) -> f32 {
        let error = desired - actual;

        self.sum += error * self.sampling_period;
        self.sum = float::fmaxf(float::fminf(self.sum, *limit), -*limit);

        let action = error * settings.proportional
            + settings.derivative * (error - self.previous) / self.sampling_period
            + settings.integral * self.sum;
        self.previous = error;

        float::fmaxf(float::fminf(action, *limit), -*limit)
    }
}
