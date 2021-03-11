/// We want an abstraction that provides us with the current measured speed and position.
/// The abstraction is periodically awaken to sample.
/// The position should be precise.
/// The measured speed is in generally measured from position difference in two samples.
/// The encoder shall store information about the total position.
/// The total position shall be resettable.
/// For non-quadrature encoders a method that indicates change of motor rotation shall be implemented.
use core::ops::{AddAssign, SubAssign};
use embedded_time::duration::Microseconds;
use embedded_time::fixed_point::FixedPoint;

/// `Position` represents the total "distance" ridden by the motor.
/// For maximal precision, it is split into the counter of revolutions and the current angle.
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

    pub fn get_resolution(&self) -> u16 {
        self.resolution
    }

    pub fn get_revolutions(&self) -> i32 {
        self.revolutions
    }

    pub fn get_angle(&self) -> u16 {
        self.angle
    }

    /// Returns the position as number of increments, this is useful for precise control.
    /// # Examples
    /// ```
    /// use sm4_shared::encoder::Position;
    ///
    /// let position = Position::new(4, 1, 2);
    /// assert_eq!(position.as_increments(), 6);
    ///
    /// let position = Position::new(4, -1, 2);
    /// assert_eq!(position.as_increments(), -2);
    /// ```
    pub fn as_increments(&self) -> i32 {
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
    /// assert_eq!(position.as_increments(), 1);
    ///
    /// position += 5;
    /// assert_eq!(position.as_increments(), 6);
    ///
    /// position += -2;
    /// assert_eq!(position.as_increments(), 4);
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
    pub rps: f32,
}

impl Speed {
    pub fn zero() -> Self {
        Self { rps: 0.0 }
    }

    pub fn new(rps: f32) -> Self {
        Self { rps }
    }

    fn from_positions(current: &Position, past: &Position, period: Microseconds) -> Self {
        let resolution = current.resolution as f32;
        let diff = (current.as_increments() - past.as_increments()) as f32;
        let rps = diff / resolution * 1.0e6 / *period.integer() as f32;
        Self { rps }
    }
}

/// A trait abstracting common encoder functionality.
/// It is suitable for both incremental and absolute encoders.
/// It is designed so its [`Self::sample()`] shall be periodically called with known fixed period,
/// which allows for speed calculations.
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
            self.current_position += if self.clockwise { 1 } else { -1 };

            self.current_speed =
                Speed::from_positions(&self.current_position, &past, self.sampling_period);
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
        assert_eq!(position.as_increments(), -2);
    }
}
