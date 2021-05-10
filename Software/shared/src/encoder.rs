//! Abstraction for handling both incremental and absolute encoders.
//!
//! This module contains only the abstraction for encoders, supplementary data objects are in the `models` module.
//! There are tests for majority of the components
//!
//! The abstraction was developed with both incremental and absolute encoders in mind and
//! was based on the following requirements:
//!
//! # Requirements
//! * We want an abstraction that provides us with the current measured velocity and position.
//! * The abstraction is periodically awaken to sample.
//! * The position should be precise.
//! * The measured velocity is in generally measured from position difference in two samples.
//! * The encoder shall store information about the total position.
//! * The total position shall be resettable.
//! * For non-quadrature encoders a method that indicates change of motor rotation shall be implemented.
use crate::models::{Position, Velocity};

/// `Direction` enum represents the direction where the motor is turning when looking at the shaft.
#[derive(Copy, Clone, PartialEq)]
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
    /// Returns the opposite direction. `CounterClockwise` when `Clockwise` is selected and vice-versa.
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Clockwise => Direction::CounterClockwise,
            Direction::CounterClockwise => Direction::Clockwise,
        }
    }

    /// In motor control the direction of rotation is usually denoted by a positive or negative number.
    /// This method returns `1` for `Clockwise` and `-1` for `CounterClockwise`
    pub fn multiplier(&self) -> i32 {
        match self {
            Direction::Clockwise => 1,
            Direction::CounterClockwise => -1,
        }
    }
}

impl From<f32> for Direction {
    fn from(velocity: f32) -> Self {
        if velocity > 0.0 {
            Direction::Clockwise
        } else {
            Direction::CounterClockwise
        }
    }
}

/// A trait abstracting common encoder functionality.
/// It is suitable for both incremental and absolute encoders.
/// It is designed so its [`Self::sample()`] shall be periodically called with known fixed period,
/// which allows for velocity calculations.
pub trait Encoder<const RESOLUTION: u32> {
    /// Returns the velocity measured by the encoder.
    /// This value is generally calculated from consecutive position readings.
    fn get_velocity(&self) -> Velocity;

    /// Returns the current position of the shaft.
    fn get_position(&self) -> Position<RESOLUTION>;

    /// Sets the sampled position to zero.
    /// This is applicable only with incremental encoders.
    /// Absolute encoders might offset the zero by software.
    fn reset_position(&mut self) -> Position<RESOLUTION>;

    /// This functions shall be periodically called to sample the encoder.
    /// Sampled values are used for position and velocity readings.
    fn sample(&mut self);

    /// This method shall be called with non-directional encoders whenever there is a change of rotation direction.
    /// # Arguments
    /// * `direction` - indicates whether the shaft is now turning in the clockwise or counterclockwise direction.
    fn notify_direction_changed(&mut self, direction: Direction);
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_time::duration::Microseconds;

    const ENCODER_RESOLUTION: u32 = 4;

    struct MockEncoder {
        current_position: Position<ENCODER_RESOLUTION>,
        current_velocity: Velocity,
        direction: Direction,
        sampling_period: Microseconds,
    }

    impl MockEncoder {
        fn new() -> Self {
            Self {
                current_position: Position::zero(),
                current_velocity: Velocity::zero(),
                direction: Direction::Clockwise,
                sampling_period: Microseconds(1000),
            }
        }
    }

    impl Encoder<ENCODER_RESOLUTION> for MockEncoder {
        fn get_velocity(&self) -> Velocity {
            self.current_velocity
        }

        fn get_position(&self) -> Position<ENCODER_RESOLUTION> {
            self.current_position
        }

        fn reset_position(&mut self) -> Position<ENCODER_RESOLUTION> {
            let past = self.current_position;
            self.current_position = Position::zero();
            self.current_velocity = Velocity::zero();
            past
        }

        fn sample(&mut self) {
            let past = self.current_position;
            self.current_position += if self.direction == Direction::Clockwise {
                1
            } else {
                -1
            };

            self.current_velocity =
                Velocity::from_positions(&self.current_position, &past, self.sampling_period);
        }

        fn notify_direction_changed(&mut self, direction: Direction) {
            self.direction = direction;
        }
    }

    #[test]
    fn mock_encoder_test() {
        let mut encoder = MockEncoder::new();
        encoder.notify_direction_changed(Direction::Clockwise);
        assert_eq!(encoder.get_velocity().get_rps(), 0.0);
        assert_eq!(encoder.get_position().get_increments(), 0);

        encoder.sample();

        assert_eq!(encoder.get_velocity().get_rps(), 250.0);
        assert_eq!(encoder.get_position().get_increments(), 1);

        encoder.reset_position();

        assert_eq!(encoder.get_velocity().get_rps(), 0.0);
        assert_eq!(encoder.get_position().get_increments(), 0);
    }
}
