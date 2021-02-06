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
use sm4_firmware::monitoring::Monitoring;
use sm4_firmware::step_counter::Counter;
use sm4_firmware::step_timer::ControlTimer;
use sm4_firmware::usb::USBProtocol;
use sm4_shared::{Driver, Motor1, Motor2, StepperDriver};
use stm32f4xx_hal::dma::StreamsTuple;

const SECOND: u32 = 168_000_000;

const BLINK_PERIOD: u32 = SECOND / 10;
const MONITORING_PERIOD: u32 = SECOND / 10;
const RAMPING_PERIOD: u32 = SECOND / 10;

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

pub struct TrapRampGen {
    current_speed: f32,
    target_acceleration: f32,
    generation_frequency: f32,
}

impl TrapRampGen {
    pub fn new(target_acceleration: f32, frequency: f32) -> Self {
        Self {
            current_speed: 0.0,
            target_acceleration,
            generation_frequency: frequency
        }
    }
    pub fn generate(&mut self, target_speed: f32) -> f32 {
        let step = self.target_acceleration / self.generation_frequency;
        let diff = target_speed - self.current_speed;
        if diff < step {
            self.current_speed = target_speed;
        } else {
            // self.current_speed += diff.signum() * step;
        }
        self.current_speed
    }
}

pub struct DriverWithGen<T> {
    driver: T,
    ramp_gen: TrapRampGen,
    target_speed: f32
}

impl<T> DriverWithGen<T> where T: StepperDriver {
    pub fn new(driver: T, ramp_gen: TrapRampGen) -> Self {
        Self {
            driver,
            ramp_gen,
            target_speed: 0.0
        }
    }

    pub fn update(&mut self) {
        let new_speed = self.ramp_gen.generate(self.target_speed) * 200.0;
        self.driver.set_step_frequency(new_speed);
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.target_speed = speed;
    }
}

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        leds: LEDs,
        usb: USBProtocol,
        driver1: DriverWithGen<Motor1Driver>,
        driver2: DriverWithGen<Motor2Driver>,
        monitoring: Monitoring,
    }

    #[init(schedule = [blink, monitoring, ramping])]
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

        let monitoring = Monitoring::new(device.ADC1, gpio.battery_voltage, dma2.0);

        let (ref1, ref2) = initialize_current_ref(device.DAC, gpio.ref1, gpio.ref2);
        let dir1 = DirectionPin::dir1(gpio.dir1);
        let dir2 = DirectionPin::dir2(gpio.dir2);
        let timer1 = ControlTimer::init_tim8(device.TIM8, clocks);
        let timer2 = ControlTimer::init_tim1(device.TIM1, clocks);
        let counter1 = Counter::tim5(device.TIM5);
        let counter2 = Counter::tim2(device.TIM2);

        let driver1 = Driver::new(timer1, dir1, counter1, ref1);
        let driver2 = Driver::new(timer2, dir2, counter2, ref2);

        let driver1 = DriverWithGen::new(driver1, TrapRampGen::new(1.0, 10.0));
        let driver2 = DriverWithGen::new(driver2, TrapRampGen::new(1.0, 10.0));

        let now = cx.start;
        cx.schedule.blink(now + BLINK_PERIOD.cycles()).unwrap();
        cx.schedule
            .monitoring(now + MONITORING_PERIOD.cycles())
            .unwrap();
        cx.schedule
            .ramping(now + RAMPING_PERIOD.cycles())
            .unwrap();

        init::LateResources {
            leds,
            usb,
            driver1,
            driver2,
            monitoring,
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

    #[task(resources = [monitoring], schedule = [monitoring])]
    fn monitoring(cx: monitoring::Context) {
        cx.resources.monitoring.poll();

        cx.schedule
            .monitoring(cx.scheduled + MONITORING_PERIOD.cycles())
            .unwrap();
    }

    #[task(resources = [driver1, driver2], schedule = [ramping])]
    fn ramping(cx: ramping::Context) {
        cx.resources.driver1.update();
        cx.resources.driver2.update();

        cx.schedule
            .ramping(cx.scheduled + RAMPING_PERIOD.cycles())
            .unwrap();
    }

    #[task(binds = DMA2_STREAM0, resources = [monitoring])]
    fn dma(cx: dma::Context) {
        cx.resources.monitoring.transfer_complete();
    }

    extern "C" {
        fn EXTI0();
    }
};
