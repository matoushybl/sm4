use embedded_time::duration::Microseconds;
use num_traits::Float;

pub struct TrapRampGen {
    current_speed: f32,
    generation_frequency: f32,
}

impl TrapRampGen {
    pub fn new(period: Microseconds) -> Self {
        Self {
            current_speed: 0.0,
            generation_frequency: 1.0 / (period.0 as f32 / 1_000_000.0),
        }
    }
    pub fn generate(&mut self, target_speed: f32, target_acceleration: f32) -> f32 {
        let step = target_acceleration / self.generation_frequency;
        let diff = target_speed - self.current_speed;
        if diff < step {
            self.current_speed = target_speed;
        } else {
            self.current_speed += diff.signum() * step;
        }
        self.current_speed
    }
}
