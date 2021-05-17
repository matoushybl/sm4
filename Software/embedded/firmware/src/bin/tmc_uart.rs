#![no_main]
#![no_std]

use cortex_m::peripheral::DWT;
use rtic::cyccnt::U32Ext as _;
use sm4_firmware as _; // global logger + panicking-behavior + memory layout

use stm32f4xx_hal as hal;

use core::fmt::Write;
use sm4_firmware::prelude::StatusLED;
use sm4_firmware::staging::config::Config;
use sm4_firmware::staging::{NoRx, Serial};
use sm4_firmware::SM4;
use stm32f4xx_hal::gpio::gpioa::PA9;
use stm32f4xx_hal::gpio::{Alternate, AF7};
use stm32f4xx_hal::prelude::*;

const SECOND: u32 = 168_000_000;
const BLINK: u32 = SECOND / 100;
type HDSerial = Serial<stm32f4xx_hal::pac::USART1, (PA9<Alternate<AF7>>, NoRx), u8>;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: StatusLED,
        serial: HDSerial,
    }

    #[init(schedule = [blink,])]
    fn init(cx: init::Context) -> init::LateResources {
        let mut core: rtic::Peripherals = cx.core;
        let device: hal::pac::Peripherals = cx.device;

        core.DCB.enable_trace();
        DWT::unlock();
        core.DWT.enable_cycle_counter();

        let clocks = device
            .RCC
            .constrain()
            .cfgr
            .use_hse(25.mhz())
            .sysclk(168.mhz())
            .hclk(168.mhz())
            .pclk1(42.mhz())
            .pclk2(84.mhz())
            .require_pll48clk()
            .freeze();

        let gpioa = device.GPIOA.split();
        let gpiob = device.GPIOB.split();
        let status_led = gpiob.pb7.into_push_pull_output();

        let uart_pin = gpioa.pa9.into_alternate_af7();
        let config = Config::default().half_duplex();
        let serial: Serial<_, _, u8> =
            Serial::new(device.USART1, (uart_pin, NoRx), config, clocks).unwrap();

        let now = cx.start;
        cx.schedule.blink(now + BLINK.cycles()).unwrap();

        init::LateResources {
            led: status_led,
            serial,
        }
    }

    #[idle()]
    fn main(_cx: main::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(resources = [led, serial], schedule = [blink])]
    fn blink(cx: blink::Context) {
        cx.resources.led.toggle();
        let serial: &mut HDSerial = cx.resources.serial;

        serial
            .bwrite_all(&[0b1010_0000, 0x01, 0b10101010, 0x01, 0x02, 0x03, 0x04, 0x55])
            .unwrap();

        cx.schedule.blink(cx.scheduled + BLINK.cycles()).unwrap();
    }

    extern "C" {
        fn EXTI0();
    }
};
