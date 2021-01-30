use crate::board::{Dir1, Dir2};
use sm4_shared::{Direction, DirectionController, Motor1, Motor2};
use stm32f4xx_hal::hal::digital::v2::OutputPin;

pub struct DirectionPin<T> {
    pin: T,
}

macro_rules! direction {
    ($motor:ident, $pin:ident, $new:ident) => {
        impl DirectionPin<$pin> {
            pub fn $new(pin: $pin) -> Self {
                Self { pin }
            }
        }

        impl DirectionController<$motor> for DirectionPin<$pin> {
            fn set_direction(&mut self, direction: Direction) {
                match direction {
                    Direction::Clockwise => self.pin.set_high().unwrap(),
                    Direction::CounterClockwise => self.pin.set_low().unwrap(),
                }
            }
        }
    };
}

direction!(Motor1, Dir1, dir1);
direction!(Motor2, Dir2, dir2);
