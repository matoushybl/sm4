use embedded_time::duration::Microseconds;
use sm4_shared::{encoder::*, Direction};
use stm32f4xx_hal::stm32;
use stm32f4xx_hal::stm32::{TIM2, TIM5};

pub trait Counter {
    fn get_value(&self) -> u32;
    fn reset_value(&mut self);
}

pub struct StepCounterEncoder<T> {
    timer: T,
    past_position: Position,
    current_position: Position,
    current_speed: Speed,
    direction: Direction,
    sampling_period: Microseconds,
    resolution: u16,
}

impl<T> StepCounterEncoder<T>
where
    T: Counter,
{
    fn update_current_position(&mut self) {
        self.current_position += if self.direction == Direction::Clockwise {
            self.timer.get_value() as i32
        } else {
            -(self.timer.get_value() as i32)
        };
        self.timer.reset_value();
    }
}

impl<T> Encoder for StepCounterEncoder<T>
where
    T: Counter,
{
    fn get_speed(&self) -> Speed {
        self.current_speed
    }

    fn get_position(&self) -> Position {
        self.current_position
    }

    fn reset_position(&mut self) -> Position {
        let past = self.current_position;
        self.current_position = Position::zero(self.resolution);
        self.past_position = Position::zero(self.resolution);
        self.current_speed = Speed::zero();
        self.timer.reset_value();
        past
    }

    fn sample(&mut self) {
        self.past_position = self.current_position;
        self.update_current_position();

        self.current_speed = Speed::from_positions(
            &self.current_position,
            &self.past_position,
            self.sampling_period,
        );
    }

    fn notify_direction_changed(&mut self, direction: Direction) {
        self.update_current_position();

        self.direction = direction;
    }
}

macro_rules! counter {
    ($tim:ident, $new:ident, $ts:literal, $en:ident, $rst:ident) => {
        impl StepCounterEncoder<$tim> {
            pub fn $new(timer: $tim, sampling_period: Microseconds, resolution: u16) -> Self {
                unsafe {
                    let rcc = &(*stm32::RCC::ptr());
                    rcc.apb1enr.modify(|_, w| w.$en().enabled());
                    rcc.apb1rstr.modify(|_, w| w.$rst().reset());
                    rcc.apb1rstr.modify(|_, w| w.$rst().clear_bit());
                }

                timer.arr.write(|w| w.arr().bits(u32::MAX));

                timer
                    .ccmr1_input_mut()
                    .modify(|_, w| w.cc1s().ti1().ic2f().bits(0));

                timer
                    .ccer
                    .modify(|_, w| w.cc1p().clear_bit().cc1np().clear_bit().cc1e().set_bit());

                unsafe {
                    timer
                        .smcr
                        .write(|w| w.sms().ext_clock_mode().ts().bits($ts));
                }

                timer.cr1.modify(|_, w| w.cen().enabled());

                Self {
                    timer,
                    direction: Direction::Clockwise,
                    past_position: Position::zero(resolution),
                    current_position: Position::zero(resolution),
                    current_speed: Speed::zero(),
                    sampling_period,
                    resolution,
                }
            }
        }

        impl Counter for $tim {
            fn get_value(&self) -> u32 {
                self.cnt.read().cnt().bits()
            }

            fn reset_value(&mut self) {
                self.cnt.write(|w| w.cnt().bits(0));
            }
        }
    };
}

counter!(TIM2, tim2, 0, tim2en, tim2rst);
counter!(TIM5, tim5, 3, tim5en, tim5rst);
