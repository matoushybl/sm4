use crate::models::Position;
use embedded_time::duration::Microseconds;
use embedded_time::fixed_point::FixedPoint;

/// Represents velocity measured by the encoder
#[derive(Copy, Clone)]
pub struct Velocity {
    /// revolutions per second
    rps: f32,
}

impl Velocity {
    /// Constructs a `Velocity` object with internal revolutions per second set to zero.
    pub fn zero() -> Self {
        Self { rps: 0.0 }
    }

    /// Constructs a `Velocity` object with internal revolutions per second set to provided argument.
    /// # Arguments
    /// * `rps` - the target RPS to be set as the velocity.
    pub fn new(rps: f32) -> Self {
        Self { rps }
    }

    /// Calculates the velocity using two sampled positions and the time between those samples.
    pub fn from_positions<const RESOLUTION: u32>(
        current: &Position<RESOLUTION>,
        past: &Position<RESOLUTION>,
        period: Microseconds,
    ) -> Self {
        let diff = (current.get_increments() - past.get_increments()) as f32;
        let rps = diff / RESOLUTION as f32 * 1.0e6 / *period.integer() as f32;
        Self { rps }
    }

    /// Returns the velocity in revolutions per second.
    pub fn get_rps(&self) -> f32 {
        self.rps
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_velocity() {
        let position1 = Position::<ENCODER_RESOLUTION> {
            revolutions: 0,
            angle: 0,
        };

        let position2 = Position::<ENCODER_RESOLUTION> {
            revolutions: 0,
            angle: 1,
        };

        let velocity = Velocity::from_positions(&position2, &position1, Microseconds(10));
        assert_eq!(velocity.rps, 25000.0);

        let velocity = Velocity::from_positions(&position1, &position2, Microseconds(10));
        assert_eq!(velocity.rps, -25000.0);

        let position1 = Position::<ENCODER_RESOLUTION>::new(0, ENCODER_RESOLUTION - 1);

        let position2 = Position::<ENCODER_RESOLUTION>::new(1, 0);

        let velocity = Velocity::from_positions(&position2, &position1, Microseconds(10));
        assert_eq!(velocity.rps, 25000.0);
    }
}
