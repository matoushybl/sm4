use num_traits::Float;
use sm4_shared::StepperDriver;

pub struct TrapRampGen {
    current_speed: f32,
    target_acceleration: f32,
    generation_frequency: f32,
}

impl TrapRampGen {
    pub fn new(target_acceleration: f32, frequency: f32) -> Self {
        Self {
            current_speed: 0.0,
            target_acceleration,
            generation_frequency: frequency,
        }
    }
    pub fn generate(&mut self, target_speed: f32) -> f32 {
        let step = self.target_acceleration / self.generation_frequency;
        let diff = target_speed - self.current_speed;
        if diff < step {
            self.current_speed = target_speed;
        } else {
            self.current_speed += diff.signum() * step;
        }
        self.current_speed
    }
}

pub struct DriverWithGen<T> {
    driver: T,
    ramp_gen: TrapRampGen,
    target_speed: f32,
    max_speed: f32,
}

impl<T> DriverWithGen<T>
where
    T: StepperDriver,
{
    pub fn new(driver: T, max_speed: f32, ramp_gen: TrapRampGen) -> Self {
        Self {
            driver,
            ramp_gen,
            target_speed: 0.0,
            max_speed,
        }
    }

    pub fn update(&mut self) {
        let new_speed = self.ramp_gen.generate(self.target_speed) * 200.0;
        self.driver.set_output_frequency(new_speed);
    }

    pub fn set_speed(&mut self, speed: f32) {
        // clamp is unstable
        self.target_speed = fmaxf(-self.max_speed, fminf(speed, self.max_speed));
    }
}

fn fmaxf(a: f32, b: f32) -> f32 {
    if a > b {
        return a;
    }
    b
}

fn fminf(a: f32, b: f32) -> f32 {
    if b > a {
        return a;
    }
    b
}
