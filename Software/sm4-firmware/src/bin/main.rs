#![no_main]
#![no_std]

use cortex_m::peripheral::DWT;
use rtic::cyccnt::U32Ext as _;
use sm4_firmware as _; // global logger + panicking-behavior + memory layout

use stm32f4xx_hal as hal;

use stm32f4xx_hal::gpio::gpiob::{PB, PB13, PB15, PB4, PB8, PB9};
use stm32f4xx_hal::gpio::{
    Alternate, AlternateOD, Floating, Input, Output, PushPull, AF4, AF5, AF9,
};
use stm32f4xx_hal::i2c::I2c;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::stm32;
use stm32f4xx_hal::stm32::{I2C1, I2C3};

use core::borrow::Borrow;
use embedded_hal::blocking::delay::DelayMs;
use stm32f4xx_hal::bb;
use stm32f4xx_hal::gpio::gpioa::{PA, PA8};
use stm32f4xx_hal::gpio::gpioc::PC;
use stm32f4xx_hal::otg_fs::*;
use stm32f4xx_hal::rcc::Clocks;
use stm32f4xx_hal::spi::{NoMiso, Spi};
use stm32f4xx_hal::stm32::Interrupt::TIM1_UP_TIM10;
use stm32f4xx_hal::timer::{Event, Timer};

const SECOND: u32 = 84_000_000;

const BLINK_PERIOD: u32 = SECOND / 10;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: PC<Output<PushPull>>,
        timer: stm32::TIM2,
        tim1: stm32::TIM1,
    }

    #[init(schedule = [blink])]
    fn init(cx: init::Context) -> init::LateResources {
        let mut core: rtic::Peripherals = cx.core;
        let device: stm32::Peripherals = cx.device;

        // enable CYCCNT
        core.DCB.enable_trace();
        DWT::unlock();
        core.DWT.enable_cycle_counter();

        let rcc = device.RCC;
        rcc.apb1enr.modify(|_, w| w.tim2en().enabled());
        rcc.apb1rstr.modify(|_, w| w.tim2rst().reset());
        rcc.apb1rstr.modify(|_, w| w.tim2rst().clear_bit());

        rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
        rcc.apb2rstr.modify(|_, w| w.tim1rst().reset());
        rcc.apb2rstr.modify(|_, w| w.tim1rst().clear_bit());

        let mut timer = device.TIM2;

        timer.arr.write(|w| w.arr().bits(15));

        timer
            .ccmr1_input_mut()
            .modify(|_, w| w.cc1s().ti1().ic2f().bits(0));

        timer
            .ccer
            .modify(|_, w| w.cc1p().clear_bit().cc1np().clear_bit().cc1e().set_bit());

        timer.smcr.write(|w| w.sms().ext_clock_mode().ts().itr0());

        timer.cr1.modify(|_, w| w.cen().enabled());

        // TIM1
        // pause
        let tim1 = device.TIM1;
        tim1.cr1.modify(|_, w| w.cen().clear_bit());
        // reset counter
        tim1.cnt.reset();

        let frequency = 200; // 2hz
        let pclk_mul = 1;
        let ticks: u32 = 84_000_000 * pclk_mul / frequency;

        let psc = (ticks - 1) / (1 << 16);
        tim1.psc.write(|w| w.psc().bits(psc as u16));

        let arr = ticks / (psc + 1);
        tim1.arr.write(|w| unsafe { w.bits(arr as u32) });
        tim1.dier.write(|w| w.uie().set_bit());

        tim1.cr2.modify(|_, w| w.mms().update());

        // start counter
        tim1.cr1.modify(|_, w| w.cen().set_bit());

        let rcc = rcc
            .constrain()
            .cfgr
            .sysclk(84.mhz())
            .hclk(84.mhz())
            .pclk1(42.mhz())
            .pclk2(84.mhz());
        let clocks = rcc.freeze();

        let gpioa = device.GPIOA.split();
        let gpiob = device.GPIOB.split();
        let gpioc = device.GPIOC.split();

        // let _in = gpioa.pa0.into_alternate_af1();

        let led = gpioc.pc13.into_push_pull_output().downgrade();

        let now = cx.start;
        cx.schedule.blink(now + BLINK_PERIOD.cycles()).unwrap();

        init::LateResources { led, timer, tim1 }
    }

    #[idle(resources = [])]
    fn main(cx: main::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(resources = [led, timer], schedule = [blink])]
    fn blink(cx: blink::Context) {
        let led: &mut PC<Output<PushPull>> = cx.resources.led;
        let timer: &mut stm32::TIM2 = cx.resources.timer;
        // if led.is_low().unwrap() {
        //     led.set_high().unwrap();
        // } else {
        //     led.set_low().unwrap();
        // }

        defmt::error!("pulses {:u32}", timer.cnt.read().bits());

        cx.schedule
            .blink(cx.scheduled + BLINK_PERIOD.cycles())
            .unwrap();
    }

    #[task(binds = TIM1_UP_TIM10, resources = [tim1, led])]
    fn tim1_handler(cx: tim1_handler::Context) {
        let tim1: &mut stm32::TIM1 = cx.resources.tim1;
        let led: &mut PC<Output<PushPull>> = cx.resources.led;
        if led.is_low().unwrap() {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }

        tim1.sr.write(|w| w.uif().clear_bit());
        defmt::error!("trig");
    }

    extern "C" {
        fn EXTI0();
    }
};
