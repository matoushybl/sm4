#![no_main]
#![no_std]

use rtic::cyccnt::U32Ext as _;
use sm4_firmware as _; // global logger + panicking-behavior + memory layout

use stm32f4xx_hal as hal;

use sm4_firmware::SM4;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        driver: SM4,
    }

    #[init(schedule = [blink, monitoring, ramping, failsafe])]
    fn init(cx: init::Context) -> init::LateResources {
        let core: rtic::Peripherals = cx.core;
        let device: hal::pac::Peripherals = cx.device;

        let driver = SM4::init(device, core);

        let now = cx.start;
        cx.schedule
            .blink(now + SM4::blink_period().cycles())
            .unwrap();
        cx.schedule
            .monitoring(now + SM4::monitoring_period().cycles())
            .unwrap();
        cx.schedule
            .ramping(now + SM4::ramping_period().cycles())
            .unwrap();
        cx.schedule
            .failsafe(now + SM4::failsafe_period().cycles())
            .unwrap();

        init::LateResources { driver }
    }

    #[idle(resources = [driver])]
    fn main(mut cx: main::Context) -> ! {
        loop {
            cx.resources.driver.lock(|driver| driver.run());
        }
    }

    #[task(binds = OTG_FS, resources = [driver])]
    fn usb_handler(cx: usb_handler::Context) {
        cx.resources.driver.process_usb();
    }

    #[task(resources = [driver], schedule = [blink])]
    fn blink(cx: blink::Context) {
        cx.resources.driver.blink_leds();

        cx.schedule
            .blink(cx.scheduled + SM4::blink_period().cycles())
            .unwrap();
    }

    #[task(resources = [driver], schedule = [monitoring])]
    fn monitoring(cx: monitoring::Context) {
        cx.resources.driver.monitor();

        cx.schedule
            .monitoring(cx.scheduled + SM4::monitoring_period().cycles())
            .unwrap();
    }

    #[task(resources = [driver], schedule = [ramping])]
    fn ramping(cx: ramping::Context) {
        cx.resources.driver.ramp();

        cx.schedule
            .ramping(cx.scheduled + SM4::ramping_period().cycles())
            .unwrap();
    }

    #[task(resources = [driver], schedule = [failsafe])]
    fn failsafe(cx: failsafe::Context) {
        cx.resources.driver.update_failsafe();

        cx.schedule
            .failsafe(cx.scheduled + SM4::failsafe_period().cycles())
            .unwrap();
    }

    #[task(binds = DMA2_STREAM0, resources = [driver])]
    fn dma(cx: dma::Context) {
        cx.resources.driver.monitoring_complete();
    }

    #[task(binds = CAN1_RX0, resources = [driver])]
    fn can_handler(cx: can_handler::Context) {
        cx.resources.driver.process_can();
    }

    extern "C" {
        fn EXTI0();
    }
};
