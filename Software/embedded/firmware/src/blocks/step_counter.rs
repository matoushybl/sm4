use embedded_time::duration::Microseconds;
use sm4_shared::prelude::*;
use stm32f4xx_hal::stm32;
use stm32f4xx_hal::stm32::{TIM2, TIM5};

/// Trait used to abstract STEP pulse and other counters.
pub trait Counter {
    /// Return the current value of the counter.
    fn get_value(&self) -> u32;
    /// Resets the value of the counter.
    fn reset_value(&mut self);
}

pub struct StepCounterEncoder<T, const RESOLUTION: u32> {
    timer: T,
    past_position: Position<RESOLUTION>,
    current_position: Position<RESOLUTION>,
    current_velocity: Velocity,
    direction: Direction,
    sampling_period: Microseconds,
    past_value: u32,
}

impl<T, const RESOLUTION: u32> StepCounterEncoder<T, RESOLUTION>
where
    T: Counter,
{
    fn update_current_position(&mut self) {
        let value = self.timer.get_value();
        let increment = value - self.past_value;
        self.past_value = value;
        self.current_position += if self.direction == Direction::Clockwise {
            increment as i32
        } else {
            -(increment as i32)
        };

        // self.timer.reset_value();
    }
}

impl<T, const RESOLUTION: u32> Encoder<RESOLUTION> for StepCounterEncoder<T, RESOLUTION>
where
    T: Counter,
{
    fn get_velocity(&self) -> Velocity {
        self.current_velocity
    }

    fn get_position(&self) -> Position<RESOLUTION> {
        self.current_position
    }

    fn reset_position(&mut self) -> Position<RESOLUTION> {
        let past = self.current_position;
        self.current_position = Position::zero();
        self.past_position = Position::zero();
        self.current_velocity = Velocity::zero();
        self.past_value = 0;
        self.timer.reset_value();
        past
    }

    fn sample(&mut self) {
        self.update_current_position();

        self.current_velocity = Velocity::from_positions(
            &self.current_position,
            &self.past_position,
            self.sampling_period,
        );

        self.past_position = self.current_position;
    }

    fn notify_direction_changed(&mut self, direction: Direction) {
        self.update_current_position();

        self.direction = direction;
    }
}

macro_rules! counter {
    ($tim:ident, $new:ident, $ts:literal, $en:ident, $rst:ident) => {
        impl<const RESOLUTION: u32> StepCounterEncoder<$tim, RESOLUTION> {
            pub fn $new(timer: $tim, sampling_period: Microseconds) -> Self {
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
                    past_position: Position::zero(),
                    current_position: Position::zero(),
                    current_velocity: Velocity::zero(),
                    sampling_period,
                    past_value: 0,
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
