// use sm4_shared::{Direction, Motor1, Motor2, StepCounter};
// use stm32f4xx_hal::stm32;
// use stm32f4xx_hal::stm32::{TIM2, TIM5};
//
// pub struct Counter<T> {
//     timer: T,
//     direction: Direction,
//     accumulator: i64,
// }
//
// macro_rules! counter {
//     ($m:ident, $tim:ident, $new:ident, $ts:literal, $en:ident, $rst:ident) => {
//         impl Counter<$tim> {
//             pub fn $new(timer: $tim) -> Self {
//                 unsafe {
//                     let rcc = &(*stm32::RCC::ptr());
//                     rcc.apb1enr.modify(|_, w| w.$en().enabled());
//                     rcc.apb1rstr.modify(|_, w| w.$rst().reset());
//                     rcc.apb1rstr.modify(|_, w| w.$rst().clear_bit());
//                 }
//
//                 timer.arr.write(|w| w.arr().bits(u32::MAX));
//
//                 timer
//                     .ccmr1_input_mut()
//                     .modify(|_, w| w.cc1s().ti1().ic2f().bits(0));
//
//                 timer
//                     .ccer
//                     .modify(|_, w| w.cc1p().clear_bit().cc1np().clear_bit().cc1e().set_bit());
//
//                 unsafe {
//                     timer
//                         .smcr
//                         .write(|w| w.sms().ext_clock_mode().ts().bits($ts));
//                 }
//
//                 timer.cr1.modify(|_, w| w.cen().enabled());
//
//                 Self {
//                     timer,
//                     direction: Direction::Clockwise,
//                     accumulator: 0,
//                 }
//             }
//
//             fn get_raw_count(&self) -> u32 {
//                 self.timer.cnt.read().cnt().bits()
//             }
//
//             fn reset_raw_count(&mut self) {
//                 self.timer.cnt.write(|w| w.cnt().bits(0));
//             }
//         }
//
//         impl StepCounter<$m> for Counter<$tim> {
//             fn reset_steps(&mut self) {
//                 self.accumulator = 0;
//                 self.reset_raw_count();
//             }
//
//             fn get_steps(&mut self) -> i64 {
//                 self.accumulator +=
//                     (self.direction.multiplier() as i64) * (self.get_raw_count() as i64);
//                 self.reset_raw_count();
//                 self.accumulator
//             }
//
//             fn set_direction(&mut self, direction: Direction) {
//                 self.accumulator +=
//                     (self.direction.multiplier() as i64) * (self.get_raw_count() as i64);
//                 self.reset_raw_count();
//                 self.direction = direction;
//             }
//         }
//     };
// }
//
// counter!(Motor2, TIM2, tim2, 0, tim2en, tim2rst);
// counter!(Motor1, TIM5, tim5, 3, tim5en, tim5rst);
