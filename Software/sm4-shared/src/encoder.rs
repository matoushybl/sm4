/// We want an abstraction that provides us with the current measured speed and position.
/// The abstraction is periodically awaken to sample.
/// The position should be precise.
/// The measured speed is in generally measured from position difference in two samples.
/// The encoder shall store information about the total position.
/// The total position shall be resettable.
/// For non-quadrature encoders a method that indicates change of motor rotation shall be implemented.

/// `Position` represents the total "distance" ridden by the motor.
/// For maximal precision, it is split into the counter of revolutions and the current angle.
#[derive(Copy, Clone, Default)]
pub struct Position {
    pub resolution: u16,
    pub revolutions: i32,
    pub angle: u16,
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
    /// assert_eq!(position.resolution, 4);
    /// assert_eq!(position.revolutions, 0);
    /// assert_eq!(position.angle, 0);
    /// ```
    pub fn zero(resolution: u16) -> Self {
        Self {
            resolution,
            revolutions: 0,
            angle: 0
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
    /// assert_eq!(position.resolution, 4);
    /// assert_eq!(position.revolutions, 5);
    /// assert_eq!(position.angle, 2);
    ///
    /// let invalid_position = Position::new(4, 1, 7);
    /// assert_eq!(invalid_position.resolution, 4);
    /// assert_eq!(invalid_position.revolutions, 2);
    /// assert_eq!(invalid_position.angle, 3);
    /// ```
    pub fn new(resolution: u16, revolutions: i32, angle: u16) -> Self {
        Self {
            resolution,
            revolutions: revolutions + (angle / resolution) as i32,
            angle: angle % resolution
        }
    }

    fn as_increments(&self) -> i32 {
        self.revolutions * self.resolution as i32 + self.angle as i32
    }
}

use core::ops::Add;
use embedded_time::duration::Microseconds;
use embedded_time::fixed_point::FixedPoint;

/// Represents speed measured by the encoder
#[derive(Copy, Clone, Default)]
pub struct Speed {
    /// revolutions per second
    pub rps: f32,
}

impl Speed {
    fn from_position_diff(current: &Position, past: &Position, period: Microseconds) -> Self {
        let resolution = current.resolution as f32;
        let diff = (current.as_increments() - past.as_increments()) as f32;
        let rps = diff / resolution * 1.0e6 / *period.integer() as f32;
        Self {
            rps
        }
    }
}

pub trait Encoder {
    fn get_speed(&self) -> Speed;
    fn get_position(&self) -> Position;

    fn reset_position(&mut self) -> Position;

    /// A function to
    fn sample(&mut self);

    fn notify_direction_changed(&mut self, clockwise: bool);
}

// #[cfg(test)]
mod tests {
    use super::*;

    const ENCODER_RESOLUTION: u16 = 4;

    struct MockEncoder {
        current_position: Position,
        current_speed: Speed,
        clockwise: bool,
        sampling_period: Microseconds,
    }

    impl Encoder for MockEncoder {
        fn get_speed(&self) -> Speed {
            unimplemented!()
        }

        fn get_position(&self) -> Position {
            self.current_position
        }

        fn reset_position(&mut self) -> Position {
            let past = self.current_position;
            self.current_position = Position::default();
            self.current_speed = Speed::default();
            past
        }

        fn sample(&mut self) {
            let past = self.current_position;
            if self.clockwise {
                self.current_position.angle += 1;
            } else {
                self.current_position.angle -= 1;
            }

            self.current_speed = Speed::from_position_diff(&self.current_position, &past, self.sampling_period);
        }

        fn notify_direction_changed(&mut self, clockwise: bool) {
            self.clockwise = clockwise;
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

        let speed = Speed::from_position_diff(&position2, &position1, Microseconds(10));
        assert_eq!(speed.rps, 25000.0);

        let speed = Speed::from_position_diff(&position1, &position2, Microseconds(10));
        assert_eq!(speed.rps, -25000.0);

        let position1 = Position {
            resolution: ENCODER_RESOLUTION,
            revolutions: 0,
            angle: ENCODER_RESOLUTION - 1,
        };

        let position2 = Position {
            resolution: ENCODER_RESOLUTION,
            revolutions: 1,
            angle: 0,
        };

        let speed = Speed::from_position_diff(&position2, &position1, Microseconds(10));
        assert_eq!(speed.rps, 25000.0);

        let position = Position::new(4, 5, 2);

        assert_eq!(position.resolution, 4);
        assert_eq!(position.revolutions, 5);
        assert_eq!(position.angle, 2);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
