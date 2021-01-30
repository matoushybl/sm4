#![no_main]
#![no_std]

use cortex_m::peripheral::DWT;
use rtic::cyccnt::U32Ext as _;
use sm4_firmware as _; // global logger + panicking-behavior + memory layout

use stm32f4xx_hal as hal;

use hal::prelude::*;
use hal::stm32;

use sm4_firmware::board::prelude::*;
use sm4_firmware::current_reference::{initialize_current_ref, Reference};
use sm4_firmware::direction::DirectionPin;
use sm4_firmware::leds::LEDs;
use sm4_firmware::step_counter::Counter;
use sm4_firmware::step_timer::ControlTimer;
use sm4_firmware::usb::USBProtocol;
use sm4_shared::{Direction, Driver, Motor1, Motor2, StepperDriver};
use stm32f4xx_hal::adc::config::{AdcConfig, Align, Clock, Resolution, SampleTime};
use stm32f4xx_hal::adc::Temperature;
use stm32f4xx_hal::dma::traits::{Channel, Stream};
use stm32f4xx_hal::dma::{Channel0, Stream0, StreamsTuple};
use stm32f4xx_hal::hal::adc::Channel;
use stm32f4xx_hal::signature::{VtempCal110, VtempCal30};

const SECOND: u32 = 168_000_000;

const BLINK_PERIOD: u32 = SECOND / 10;

type Motor1Driver = Driver<
    Motor1,
    ControlTimer<stm32::TIM8>,
    DirectionPin<Dir1>,
    Counter<stm32::TIM5>,
    Reference<CurrentRef1Channel>,
>;

type Motor2Driver = Driver<
    Motor2,
    ControlTimer<stm32::TIM1>,
    DirectionPin<Dir2>,
    Counter<stm32::TIM2>,
    Reference<CurrentRef2Channel>,
>;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        leds: LEDs,
        usb: USBProtocol,
        driver1: Motor1Driver,
        driver2: Motor2Driver,
    }

    #[init(schedule = [blink])]
    fn init(cx: init::Context) -> init::LateResources {
        let mut core: rtic::Peripherals = cx.core;
        let device: stm32::Peripherals = cx.device;

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

        let gpio = GPIO::configure(
            device.GPIOA.split(),
            device.GPIOB.split(),
            device.GPIOC.split(),
        );

        let usb = USBProtocol::new(
            device.OTG_FS_GLOBAL,
            device.OTG_FS_DEVICE,
            device.OTG_FS_PWRCLK,
            gpio.usb_minus,
            gpio.usb_plus,
            clocks,
        );

        let mut leds = LEDs::new(gpio.status_led, gpio.error_led);
        leds.signalize_sync();

        let dma2 = StreamsTuple::new(device.DMA2);

        let mut stream0 = dma2.0;

        // let res = adc.convert::<Temperature>(&Temperature, SampleTime::Cycles_480);
        // defmt::error!(
        //     "raw {:?}, {:?}, {:?}",
        //     res,
        //     VtempCal30::get().read(),
        //     VtempCal110::get().read()
        // );
        // defmt::error!(
        //     "temp: {:?}",
        //     (110 - 30) * (res - VtempCal30::get().read())
        //         / (VtempCal110::get().read() - VtempCal30::get().read())
        //         + 30
        // );
        //
        // let res = adc.convert(&gpio.battery_voltage, SampleTime::Cycles_480);
        // defmt::error!("volt: {:?}", res);

        let (ref1, ref2) = initialize_current_ref(device.DAC, gpio.ref1, gpio.ref2);
        let dir1 = DirectionPin::dir1(gpio.dir1);
        let dir2 = DirectionPin::dir2(gpio.dir2);
        let timer1 = ControlTimer::init_tim8(device.TIM8, clocks);
        let timer2 = ControlTimer::init_tim1(device.TIM1, clocks);
        let counter1 = Counter::tim5(device.TIM5);
        let counter2 = Counter::tim2(device.TIM2);

        let driver1 = Driver::new(timer1, dir1, counter1, ref1);
        let mut driver2 = Driver::new(timer2, dir2, counter2, ref2);

        driver2.set_step_frequency(20.0);

        let now = cx.start;
        cx.schedule.blink(now + BLINK_PERIOD.cycles()).unwrap();

        init::LateResources {
            leds,
            usb,
            driver1,
            driver2,
        }
    }

    #[idle(resources = [])]
    fn main(_cx: main::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(binds = OTG_FS, resources = [usb])]
    fn usb_handler(cx: usb_handler::Context) {
        cx.resources.usb.process_interrupt();
    }

    #[task(resources = [leds], schedule = [blink])]
    fn blink(cx: blink::Context) {
        cx.resources.leds.tick();

        cx.schedule
            .blink(cx.scheduled + BLINK_PERIOD.cycles())
            .unwrap();
    }

    extern "C" {
        fn EXTI0();
    }
};
