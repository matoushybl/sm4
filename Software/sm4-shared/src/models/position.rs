use core::ops::{Add, AddAssign, Sub, SubAssign};

/// `Position` represents the total "distance" ridden by the motor.
/// For maximal precision, it is split into the counter of revolutions and the current angle.
/// When the number of revolutions is positive, the angle is added to it.
/// When the number of revolutions is negative,
/// there is always one more revolution added (-2.5 revolutions in reality -> -3 revolutions in `Position`) and
/// the resulting position is calculated by adding the positive angle to it.
#[derive(Copy, Clone)]
pub struct Position<const RESOLUTION: u32> {
    revolutions: i32,
    angle: u32,
}

impl<const RESOLUTION: u32> Position<RESOLUTION> {
    /// Create a zero position with specified resolution
    ///
    /// # Arguments
    /// - resolution - maximal number that can be reached within a single revolution.
    ///
    /// # Example
    /// ```
    /// use sm4_shared::prelude::Position;
    ///
    /// let position = Position::zero();
    /// assert_eq!(position.get_resolution(), 4);
    /// assert_eq!(position.get_revolutions(), 0);
    /// assert_eq!(position.get_angle(), 0);
    /// ```
    pub fn zero() -> Self {
        Self {
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
    /// use sm4_shared::prelude::Position;
    /// let position = Position::new(5, 2);
    ///
    /// assert_eq!(position.get_resolution(), 4);
    /// assert_eq!(position.get_revolutions(), 5);
    /// assert_eq!(position.get_angle(), 2);
    ///
    /// let invalid_position = Position::new(1, 7);
    /// assert_eq!(invalid_position.get_revolutions(), 2);
    /// assert_eq!(invalid_position.get_angle(), 3);
    ///
    /// let invalid_position = Position::new(-1, 2);
    /// assert_eq!(invalid_position.get_revolutions(), -1);
    /// assert_eq!(invalid_position.get_angle(), 2);
    /// ```
    pub fn new(revolutions: i32, angle: u32) -> Self {
        Self {
            revolutions: revolutions + (angle / RESOLUTION) as i32,
            angle: angle % RESOLUTION,
        }
    }

    /// Returns the resolution of the encoder.
    /// Resolution means the number of pulses for a full shaft turn.
    pub const fn get_resolution(&self) -> u32 {
        RESOLUTION
    }

    /// Returns the number of revolutions the shaft had travelled.
    pub fn get_revolutions(&self) -> i32 {
        self.revolutions
    }

    /// Returns the angle of the shaft in increments relative to a "zero" position.
    pub fn get_angle(&self) -> u32 {
        self.angle
    }

    /// Returns the position as number of increments, this is useful for precise control.
    /// # Examples
    /// ```
    /// use sm4_shared::prelude::Position;
    ///
    /// let position = Position::new(1, 2);
    /// assert_eq!(position.get_increments(), 6);
    ///
    /// let position = Position::new(-1, 2);
    /// assert_eq!(position.get_increments(), -2);
    /// ```
    pub fn get_increments(&self) -> i32 {
        self.revolutions * RESOLUTION as i32 + self.angle as i32
    }

    /// Returns number of revolutions as float with the angle embedded after the decimal point
    /// # Examples
    /// ```
    /// use sm4_shared::prelude::Position;
    ///
    /// let position = Position::new(1, 2);
    /// assert_eq!(position.get_relative_revolutions(), 1.5);
    ///
    /// let position = Position::new(-1, 2);
    /// assert_eq!(position.get_relative_revolutions(), -0.5);
    /// ```
    pub fn get_relative_revolutions(&self) -> f32 {
        self.revolutions as f32 + self.angle as f32 / RESOLUTION as f32
    }

    fn from_raw(mut revolutions: i32, mut angle: i32) -> Position<RESOLUTION> {
        if angle.abs() as i32 >= RESOLUTION as i32 {
            revolutions += angle.signum() * angle / RESOLUTION as i32;
            angle %= RESOLUTION as i32;
        }

        if angle < 0 {
            revolutions -= 1;
            angle += RESOLUTION as i32;
        }

        Position {
            revolutions,
            angle: angle as u32,
        }
    }
}

impl<const RESOLUTION: u32> AddAssign<i32> for Position<RESOLUTION> {
    /// Adds increments to position
    /// # Examples
    /// ```
    /// use sm4_shared::prelude::Position;
    ///
    /// let mut position = Position::zero();
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
        let added_revolutions = rhs / RESOLUTION as i32;
        let added_angle = rhs % RESOLUTION as i32;

        let new_revolutions = self.revolutions + added_revolutions;
        let new_angle = added_angle + self.angle as i32;

        let position = Position::<RESOLUTION>::from_raw(new_revolutions, new_angle);

        self.revolutions = position.revolutions;
        self.angle = position.angle;
    }
}

impl<const RESOLUTION: u32> SubAssign<i32> for Position<RESOLUTION> {
    fn sub_assign(&mut self, rhs: i32) {
        *self += -rhs;
    }
}

/// Adds position to position
/// # Examples
/// ```
/// use sm4_shared::prelude::Position;
///
/// let position = Position::zero();
/// let new_position = position + &Position::new(3, 1);
///
/// assert_eq!(new_position.get_revolutions(), 3);
/// assert_eq!(new_position.get_angle(), 1);
/// ```
impl<const RESOLUTION: u32> Add<&Position<RESOLUTION>> for Position<RESOLUTION> {
    type Output = Position<RESOLUTION>;

    fn add(self, rhs: &Position<RESOLUTION>) -> Self::Output {
        let new_revolutions = self.revolutions + rhs.revolutions;
        let new_angle = rhs.angle as i32 + self.angle as i32;

        Position::from_raw(new_revolutions, new_angle)
    }
}

impl<const RESOLUTION: u32> AddAssign<&Position<RESOLUTION>> for Position<RESOLUTION> {
    fn add_assign(&mut self, rhs: &Position<RESOLUTION>) {
        let new = *self + rhs;

        self.revolutions = new.revolutions;
        self.angle = new.angle;
    }
}

impl<const RESOLUTION: u32> Sub<&Position<RESOLUTION>> for Position<RESOLUTION> {
    type Output = Position<RESOLUTION>;

    fn sub(self, rhs: &Position<RESOLUTION>) -> Self::Output {
        let new_revolutions = self.revolutions - rhs.revolutions;
        let new_angle = self.angle as i32 - rhs.angle as i32;

        Position::from_raw(new_revolutions, new_angle)
    }
}

impl<const RESOLUTION: u32> SubAssign<&Position<RESOLUTION>> for Position<RESOLUTION> {
    fn sub_assign(&mut self, rhs: &Position<RESOLUTION>) {
        let new = *self - rhs;

        self.revolutions = new.revolutions;
        self.angle = new.angle;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn position_manipulation() {
        let mut position = Position::<ENCODER_RESOLUTION>::zero();
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

        let position = Position::<ENCODER_RESOLUTION>::zero();
        let new_position = position + &Position::new(3, 1);
        assert_eq!(new_position.get_revolutions(), 3);
        assert_eq!(new_position.get_angle(), 1);

        let position = Position::<ENCODER_RESOLUTION>::new(1, 1);
        let new_position = position + &Position::new(3, 1);
        assert_eq!(new_position.get_revolutions(), 4);
        assert_eq!(new_position.get_angle(), 2);

        let position = Position::<ENCODER_RESOLUTION>::new(1, 1);
        let new_position = position + &Position::new(3, 3);
        assert_eq!(new_position.get_revolutions(), 5);
        assert_eq!(new_position.get_angle(), 0);

        let position = Position::<ENCODER_RESOLUTION>::new(1, 1);
        let new_position = position - &Position::new(0, 1);
        assert_eq!(new_position.get_revolutions(), 1);
        assert_eq!(new_position.get_angle(), 0);

        let position = Position::<ENCODER_RESOLUTION>::new(1, 1);
        let new_position = position - &Position::new(1, 1);
        assert_eq!(new_position.get_revolutions(), 0);
        assert_eq!(new_position.get_angle(), 0);

        let position = Position::<ENCODER_RESOLUTION>::new(1, 1);
        let new_position = position - &Position::new(1, 2);
        assert_eq!(new_position.get_revolutions(), -1);
        assert_eq!(new_position.get_angle(), 3);
    }
}
