#![no_main]
#![no_std]

use cortex_m::peripheral::DWT;
use rtic::cyccnt::U32Ext as _;
use sm4_firmware as _; // global logger + panicking-behavior + memory layout

use stm32f4xx_hal as hal;

use hal::prelude::*;
use hal::stm32;

use sm4_firmware::board::{Dir1, StatusLED, GPIO};
use sm4_firmware::current_reference::initialize_current_ref;
use sm4_firmware::direction::DirectionPin;
use sm4_firmware::step_counter::Counter;
use sm4_firmware::step_timer::ControlTimer;
use sm4_firmware::usb::USBProtocol;
use sm4_shared::{CurrentReference, Direction, DirectionController, StepCounter, StepGenerator};

const SECOND: u32 = 168_000_000;

const BLINK_PERIOD: u32 = SECOND / 4;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: StatusLED,
        usb: USBProtocol,
        counter1: Counter<stm32::TIM2>,
        dir1: DirectionPin<Dir1>,
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
        let rcc = rcc
            .constrain()
            .cfgr
            .use_hse(25.mhz())
            .sysclk(168.mhz())
            .hclk(168.mhz())
            .pclk1(42.mhz())
            .pclk2(84.mhz())
            .require_pll48clk();
        let clocks = rcc.freeze();

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

        let (mut ref1, mut ref2) = initialize_current_ref(device.DAC, gpio.ref1, gpio.ref2);

        ref1.set_current(600);
        ref2.set_current(600);

        let mut dir1 = DirectionPin::dir1(gpio.dir1);
        let mut dir2 = DirectionPin::dir2(gpio.dir2);
        dir1.set_direction(Direction::CounterClockwise);
        dir2.set_direction(Direction::Clockwise);

        let mut timer1 = ControlTimer::init_tim1(device.TIM1, clocks);
        let mut timer2 = ControlTimer::init_tim8(device.TIM8, clocks);
        timer1.set_step_frequency(200.0);
        timer2.set_step_frequency(0.0);

        let counter1 = Counter::tim2(device.TIM2);
        let counter2 = Counter::tim5(device.TIM5);

        let now = cx.start;
        cx.schedule.blink(now + BLINK_PERIOD.cycles()).unwrap();

        init::LateResources {
            led: gpio.status_led,
            counter1,
            usb,
            dir1,
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

    #[task(resources = [led, counter1, dir1], schedule = [blink])]
    fn blink(cx: blink::Context) {
        let led: &mut StatusLED = cx.resources.led;
        let counter1: &mut Counter<stm32::TIM2> = cx.resources.counter1;
        if led.is_low().unwrap() {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }

        defmt::error!("pulses {:f32}", counter1.get_steps());

        cx.schedule
            .blink(cx.scheduled + BLINK_PERIOD.cycles())
            .unwrap();
    }

    extern "C" {
        fn EXTI0();
    }
};
