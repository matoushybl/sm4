use num_traits::Float;

pub struct TrapRampGen {
    current_speed: f32,
    generation_frequency: f32,
}

impl TrapRampGen {
    pub fn new(frequency: f32) -> Self {
        Self {
            current_speed: 0.0,
            generation_frequency: frequency,
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
