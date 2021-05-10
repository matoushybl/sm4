use embedded_time::rate::Hertz;
use sm4_shared::prelude::StepGenerator;
use stm32f4xx_hal::rcc::Clocks;
use stm32f4xx_hal::stm32::TIM8;
use stm32f4xx_hal::{stm32, stm32::TIM1};

pub struct StepGeneratorTimer<T> {
    timer: T,
    clocks: Clocks,
    frequency: u32,
}

macro_rules! generator {
    ($tim:ident, $init:ident, $tim_en:ident, $tim_rst:ident) => {
        impl StepGeneratorTimer<$tim> {
            pub fn $init(timer: $tim, clocks: Clocks) -> Self {
                unsafe {
                    let rcc = &(*stm32::RCC::ptr());
                    rcc.apb2enr.modify(|_, w| w.$tim_en().enabled());
                    rcc.apb2rstr.modify(|_, w| w.$tim_rst().reset());
                    rcc.apb2rstr.modify(|_, w| w.$tim_rst().clear_bit());
                }

                // pause
                timer.cr1.modify(|_, w| w.cen().clear_bit());
                // reset counter
                timer.cnt.reset();

                timer.cr2.modify(|_, w| w.mms().compare_oc1());

                timer.ccmr1_output().modify(|_, w| {
                    w.cc1s()
                        .output()
                        .oc1fe()
                        .set_bit()
                        // .oc1pe()
                        // .enabled()
                        .oc1m()
                        .pwm_mode1()
                });
                timer.ccer.modify(|_, w| w.cc1e().set_bit());
                timer.cr1.modify(|_, w| w.cms().bits(0).opm().disabled());
                timer.bdtr.modify(|_, w| w.aoe().set_bit());

                Self {
                    timer,
                    clocks,
                    frequency: 0,
                }
            }
        }

        impl StepGenerator for StepGeneratorTimer<$tim> {
            fn set_step_frequency(&mut self, freq: Hertz) {
                if self.frequency == freq.0 {
                    return;
                }
                self.frequency = freq.0;
                // pause
                self.timer.cr1.modify(|_, w| w.cen().clear_bit());
                // reset counter
                self.timer.cnt.reset();

                let frequency = freq.0;
                if frequency == 0 {
                    return; // leave the timer in a paused state
                }

                let pclk_mul = if self.clocks.ppre2() == 1 { 1 } else { 2 };
                let ticks: u32 = self.clocks.pclk2().0 * pclk_mul / frequency;

                let psc = (ticks - 1) / (1 << 16);
                self.timer.psc.write(|w| w.psc().bits(psc as u16));

                let arr = ticks / (psc + 1);
                self.timer.arr.write(|w| unsafe { w.bits(arr as u32) });

                self.timer.cr1.modify(|_, w| w.urs().set_bit());
                self.timer.egr.write(|w| w.ug().set_bit());
                self.timer.cr1.modify(|_, w| w.urs().clear_bit());

                self.timer
                    .ccr1
                    .modify(|_, w| w.ccr().bits((arr / 2) as u16));

                // start counter
                self.timer.cr1.modify(|_, w| w.cen().set_bit());
            }
        }
    };
}

generator!(TIM1, init_tim1, tim1en, tim1rst);
generator!(TIM8, init_tim8, tim8en, tim8rst);
