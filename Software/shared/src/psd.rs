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

    pub fn proportional(&self) -> f32 {
        self.proportional
    }
    pub fn integral(&self) -> f32 {
        self.integral
    }
    pub fn derivative(&self) -> f32 {
        self.derivative
    }
    pub fn max_output_amplitude(&self) -> f32 {
        self.max_output_amplitude
    }
    pub fn set_proportional(&mut self, proportional: f32) {
        self.proportional = proportional;
    }
    pub fn set_integral(&mut self, integral: f32) {
        self.integral = integral;
    }
    pub fn set_derivative(&mut self, derivative: f32) {
        self.derivative = derivative;
    }
    pub fn set_max_output_amplitude(&mut self, max_output_amplitude: f32) {
        self.max_output_amplitude = max_output_amplitude;
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
        self.sum = self.sum.clamp(
            -settings.max_output_amplitude,
            settings.max_output_amplitude,
        );

        let action = error * settings.proportional
            + settings.derivative * (error - self.previous) / self.sampling_period
            + settings.integral * self.sum;
        self.previous = error;

        action.clamp(
            -settings.max_output_amplitude,
            settings.max_output_amplitude,
        )
    }
}
