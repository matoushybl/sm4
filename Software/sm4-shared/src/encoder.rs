//! Abstraction for handling both incremental and absolute encoders.
//!
//! This module contains the abstraction for encoders, supplementary data objects and tests for these components.
//!
//! The abstraction was developed with both incremental and absolute encoders in mind and
//! was based on the following requirements:
//!
//! # Requirements
//! * We want an abstraction that provides us with the current measured speed and position.
//! * The abstraction is periodically awaken to sample.
//! * The position should be precise.
//! * The measured speed is in generally measured from position difference in two samples.
//! * The encoder shall store information about the total position.
//! * The total position shall be resettable.
//! * For non-quadrature encoders a method that indicates change of motor rotation shall be implemented.
use crate::Direction;
use core::ops::{AddAssign, SubAssign};
use embedded_time::duration::Microseconds;
use embedded_time::fixed_point::FixedPoint;

/// `Position` represents the total "distance" ridden by the motor.
/// For maximal precision, it is split into the counter of revolutions and the current angle.
/// When the number of revolutions is positive, the angle is added to it.
/// When the number of revolutions is negative,
/// there is always one more revolution added (-2.5 revolutions in reality -> -3 revolutions in `Position`) and
/// the resulting position is calculated by adding the positive angle to it.
#[derive(Copy, Clone)]
pub struct Position {
    resolution: u16,
    revolutions: i32,
    angle: u16,
}

impl Position {
    /// Create a zero position with specified resolution
    ///
    /// # Arguments
    /// - resolution - maximal number that can be reached within a single revolution.
    ///
    /// # Example
    /// ```
    /// use sm4_shared::encoder::Position;
    ///
    /// let position = Position::zero(4);
    /// assert_eq!(position.get_resolution(), 4);
    /// assert_eq!(position.get_revolutions(), 0);
    /// assert_eq!(position.get_angle(), 0);
    /// ```
    pub fn zero(resolution: u16) -> Self {
        Self {
            resolution,
            revolutions: 0,
            angle: 0,
        }
    }

    /// Constructs a new position using resolution, revolutions and angle.
    /// When angle is higher than resolution, the values are automagically adjusted to be valid.
    ///
    /// # Arguments
    /// - resolution - maximal number that can be reached within a single revolution
    /// - revolutions - number of revolutions that was made when the position was reached
    /// - angle - position within the revolution
    ///
    /// # Example
    /// ```
    /// use sm4_shared::encoder::Position;
    /// let position = Position::new(4, 5, 2);
    ///
    /// assert_eq!(position.get_resolution(), 4);
    /// assert_eq!(position.get_revolutions(), 5);
    /// assert_eq!(position.get_angle(), 2);
    ///
    /// let invalid_position = Position::new(4, 1, 7);
    /// assert_eq!(invalid_position.get_revolutions(), 2);
    /// assert_eq!(invalid_position.get_angle(), 3);
    ///
    /// let invalid_position = Position::new(4, -1, 2);
    /// assert_eq!(invalid_position.get_revolutions(), -1);
    /// assert_eq!(invalid_position.get_angle(), 2);
    /// ```
    pub fn new(resolution: u16, revolutions: i32, angle: u16) -> Self {
        Self {
            resolution,
            revolutions: revolutions + (angle / resolution) as i32,
            angle: angle % resolution,
        }
    }

    /// Returns the resolution of the encoder.
    /// Resolution means the number of pulses for a full shaft turn.
    pub fn get_resolution(&self) -> u16 {
        self.resolution
    }

    /// Returns the number of revolutions the shaft had travelled.
    pub fn get_revolutions(&self) -> i32 {
        self.revolutions
    }

    /// Returns the angle of the shaft in increments relative to a "zero" position.
    pub fn get_angle(&self) -> u16 {
        self.angle
    }

    /// Returns the position as number of increments, this is useful for precise control.
    /// # Examples
    /// ```
    /// use sm4_shared::encoder::Position;
    ///
    /// let position = Position::new(4, 1, 2);
    /// assert_eq!(position.get_increments(), 6);
    ///
    /// let position = Position::new(4, -1, 2);
    /// assert_eq!(position.get_increments(), -2);
    /// ```
    pub fn get_increments(&self) -> i32 {
        self.revolutions * self.resolution as i32 + self.angle as i32
    }

    /// Returns number of revolutions as float with the angle embedded after the decimal point
    /// # Examples
    /// ```
    /// use sm4_shared::encoder::Position;
    ///
    /// let position = Position::new(4, 1, 2);
    /// assert_eq!(position.get_relative_revolutions(), 1.5);
    ///
    /// let position = Position::new(4, -1, 2);
    /// assert_eq!(position.get_relative_revolutions(), -0.5);
    /// ```
    pub fn get_relative_revolutions(&self) -> f32 {
        self.revolutions as f32 + self.angle as f32 / self.resolution as f32
    }
}

impl AddAssign<i32> for Position {
    /// Adds increments to position
    /// # Examples
    /// ```
    /// use sm4_shared::encoder::Position;
    ///
    /// let mut position = Position::zero(4);
    /// position += 1;
    ///
    /// assert_eq!(position.get_increments(), 1);
    ///
    /// position += 5;
    /// assert_eq!(position.get_increments(), 6);
    ///
    /// position += -2;
    /// assert_eq!(position.get_increments(), 4);
    /// ```
    fn add_assign(&mut self, rhs: i32) {
        let added_revolutions = rhs / self.resolution as i32;
        let added_angle = rhs % self.resolution as i32;

        let mut new_revolutions = self.revolutions + added_revolutions;
        let mut new_angle = added_angle + self.angle as i32;

        if new_angle.abs() as i32 >= self.resolution as i32 {
            new_revolutions += new_angle.signum();
            new_angle = new_angle % self.resolution as i32;
        }

        if new_angle < 0 {
            new_revolutions -= 1;
            new_angle += self.resolution as i32;
        }

        self.revolutions = new_revolutions;
        self.angle = new_angle as u16;
    }
}

impl SubAssign<i32> for Position {
    fn sub_assign(&mut self, rhs: i32) {
        *self += -rhs;
    }
}

/// Represents speed measured by the encoder
#[derive(Copy, Clone)]
pub struct Speed {
    /// revolutions per second
    rps: f32,
}

impl Speed {
    /// Constructs a `Speed` object with internal revolutions per second set to zero.
    pub fn zero() -> Self {
        Self { rps: 0.0 }
    }

    /// Constructs a `Speed` object with internal revolutions per second set to provided argument.
    /// # Arguments
    /// * `rps` - the target RPS to be set as the speed.
    pub fn new(rps: f32) -> Self {
        Self { rps }
    }

    /// Calculates the speed using two sampled positions and the time between those samples.
    fn from_positions(current: &Position, past: &Position, period: Microseconds) -> Self {
        let resolution = current.resolution as f32;
        let diff = (current.get_increments() - past.get_increments()) as f32;
        let rps = diff / resolution * 1.0e6 / *period.integer() as f32;
        Self { rps }
    }

    /// Returns the speed in revolutions per second.
    pub fn get_rps(&self) -> f32 {
        self.rps
    }
}

/// A trait abstracting common encoder functionality.
/// It is suitable for both incremental and absolute encoders.
/// It is designed so its [`Self::sample()`] shall be periodically called with known fixed period,
/// which allows for speed calculations.
pub trait Encoder {
    /// Returns the speed measured by the encoder.
    /// This value is generally calculated from consecutive position readings.
    fn get_speed(&self) -> Speed;

    /// Returns the current position of the shaft.
    fn get_position(&self) -> Position;

    /// Sets the sampled position to zero.
    /// This is applicable only with incremental encoders.
    /// Absolute encoders might offset the zero by software.
    fn reset_position(&mut self) -> Position;

    /// This functions shall be periodically called to sample the encoder.
    /// Sampled values are used for position and speed readings.
    fn sample(&mut self);

    /// This method shall be called with non-directional encoders whenever there is a change of rotation direction.
    /// # Arguments
    /// * `direction` - indicates whether the shaft is now turning in the clockwise or counterclockwise direction.
    fn notify_direction_changed(&mut self, direction: Direction);
}

// #[cfg(test)]
mod tests {
    use super::*;

    const ENCODER_RESOLUTION: u16 = 4;

    struct MockEncoder {
        current_position: Position,
        current_speed: Speed,
        direction: Direction,
        sampling_period: Microseconds,
    }

    impl MockEncoder {
        fn new() -> Self {
            Self {
                current_position: Position::zero(ENCODER_RESOLUTION),
                current_speed: Speed::zero(),
                direction: Direction::Clockwise,
                sampling_period: Microseconds(1000),
            }
        }
    }

    impl Encoder for MockEncoder {
        fn get_speed(&self) -> Speed {
            self.current_speed
        }

        fn get_position(&self) -> Position {
            self.current_position
        }

        fn reset_position(&mut self) -> Position {
            let past = self.current_position;
            self.current_position = Position::zero(ENCODER_RESOLUTION);
            self.current_speed = Speed::zero();
            past
        }

        fn sample(&mut self) {
            let past = self.current_position;
            self.current_position += if self.direction == Direction::Clockwise {
                1
            } else {
                -1
            };

            self.current_speed =
                Speed::from_positions(&self.current_position, &past, self.sampling_period);
        }

        fn notify_direction_changed(&mut self, direction: Direction) {
            self.direction = direction;
        }
    }

    #[test]
    fn test_speed() {
        let position1 = Position {
            resolution: ENCODER_RESOLUTION,
            revolutions: 0,
            angle: 0,
        };

        let position2 = Position {
            resolution: ENCODER_RESOLUTION,
            revolutions: 0,
            angle: 1,
        };

        let speed = Speed::from_positions(&position2, &position1, Microseconds(10));
        assert_eq!(speed.rps, 25000.0);

        let speed = Speed::from_positions(&position1, &position2, Microseconds(10));
        assert_eq!(speed.rps, -25000.0);

        let position1 = Position::new(ENCODER_RESOLUTION, 0, ENCODER_RESOLUTION - 1);

        let position2 = Position::new(ENCODER_RESOLUTION, 1, 0);

        let speed = Speed::from_positions(&position2, &position1, Microseconds(10));
        assert_eq!(speed.rps, 25000.0);
    }

    #[test]
    fn position_manipulation() {
        let mut position = Position::zero(ENCODER_RESOLUTION);
        position += 6;
        assert_eq!(position.revolutions, 1);
        assert_eq!(position.angle, 2);

        position += -2;
        assert_eq!(position.revolutions, 1);
        assert_eq!(position.angle, 0);

        position += -1;
        assert_eq!(position.revolutions, 0);
        assert_eq!(position.angle, 3);

        position -= 5;
        assert_eq!(position.revolutions, -1);
        assert_eq!(position.angle, 2);
        assert_eq!(position.get_increments(), -2);
    }

    #[test]
    fn mock_encoder_test() {
        let mut encoder = MockEncoder::new();
        encoder.notify_direction_changed(true);
        assert_eq!(encoder.get_speed().get_rps(), 0.0);
        assert_eq!(encoder.get_position().get_increments(), 0);

        encoder.sample();

        assert_eq!(encoder.get_speed().get_rps(), 250.0);
        assert_eq!(encoder.get_position().get_increments(), 1);

        encoder.reset_position();

        assert_eq!(encoder.get_speed().get_rps(), 0.0);
        assert_eq!(encoder.get_position().get_increments(), 0);
    }
}
