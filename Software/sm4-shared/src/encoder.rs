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

use core::ops::Add;
use embedded_time::duration::Microseconds;

#[derive(Copy, Clone, Default)]
pub struct Speed {
    pub resolution: u16,
    pub revolution_difference: i32,
    pub angle_difference: i16,
    pub time_difference: Microseconds, // microseconds
}

impl Speed {
    fn from_position_diff(current: &Position, past: &Position, period: Microseconds) -> Self {
        Self {
            resolution: current.resolution,
            revolution_difference: current.revolutions - past.revolutions,
            angle_difference: (current.angle as i32 - past.angle as i32) as i16,
            time_difference: period,
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

            self.current_speed = Speed {
                resolution: ENCODER_RESOLUTION,
                revolution_difference: self.current_position.revolutions - past.revolutions,
                angle_difference: self.current_position.angle as i16 - past.angle as i16,
                time_difference: self.sampling_period,
            };
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
        assert_eq!(speed.angle_difference, 1);
        assert_eq!(speed.revolution_difference, 0);

        let speed = Speed::from_position_diff(&position1, &position2, Microseconds(10));
        assert_eq!(speed.angle_difference, -1);
        assert_eq!(speed.revolution_difference, 0);

        let position1 = Position {
            resolution: ENCODER_RESOLUTION,
            revolutions: 0,
            angle: u16::MAX,
        };

        let position2 = Position {
            resolution: ENCODER_RESOLUTION,
            revolutions: 1,
            angle: 1,
        };

        let speed = Speed::from_position_diff(&position2, &position1, Microseconds(10));

        assert_eq!(speed.angle_difference, 2);
        assert_eq!(speed.revolution_difference, 0);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
