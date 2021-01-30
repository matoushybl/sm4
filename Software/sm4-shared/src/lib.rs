#![no_std]

/// Marker struct denoting the first motor.
/// The struct is meant to be used in trait implementations to provide extra level of type safety.
pub struct Motor1;

/// Marker struct denoting the second motor.
/// The struct is meant to be used in trait implementations to provide extra level of type safety.
pub struct Motor2;

/// Defines a shared interface for methods of setting motor current,
/// be it inbuilt DAC or external one or something completely different.
/// The `M` parameter is used to distinguish between motors, providing additional type safety.
pub trait CurrentReference<M> {
    /// Sets the reference current to the supplied value
    /// # Arguments
    /// * current - target current in milliamps
    fn set_current(&mut self, current: u16);
}

/// Defines a shared interface for working with systems that generates steps that control the stepper motors.
/// The stepper generator can be PWM or manual toggling of an output pin.
/// It is worth noting that the API works directly with step frequency, not microstep frequency, to be independent on microstepping.
/// The `M` parameter is used to distinguish between motors, providing additional type safety.
pub trait StepGenerator<M> {
    /// Sets the output step frequency.
    /// # Arguments
    /// * freq - frequency of whole steps output by the generator **not microsteps**.
    fn set_step_frequency(&mut self, freq: f32);
}

/// This enum represents the direction where the motor is turning.
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Clockwise
    }
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Clockwise => Direction::CounterClockwise,
            Direction::CounterClockwise => Direction::CounterClockwise,
        }
    }

    pub fn multiplier(&self) -> i32 {
        match self {
            Direction::Clockwise => 1,
            Direction::CounterClockwise => -1,
        }
    }
}

/// The `DirectionController` interface specifies a way of controlling direction where the motor is turning.
/// /// The `M` parameter is used to distinguish between motors, providing additional type safety.
pub trait DirectionController<M> {
    fn set_direction(&mut self, direction: Direction);
}

// FIXME TODO WIP section
pub trait StepCounter<M> {
    fn reset_steps(&mut self);

    fn get_steps(&mut self) -> f32;

    fn set_direction(&mut self, direction: Direction);
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
