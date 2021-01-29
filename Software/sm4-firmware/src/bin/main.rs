#![no_main]
#![no_std]

use cortex_m::peripheral::DWT;
use rtic::cyccnt::U32Ext as _;
use sm4_firmware as _; // global logger + panicking-behavior + memory layout

use stm32f4xx_hal as hal;

use stm32f4xx_hal::gpio::gpiob::{PB, PB13, PB15, PB4, PB8, PB9};
use stm32f4xx_hal::gpio::{
    Alternate, AlternateOD, Floating, Input, Output, PushPull, AF1, AF4, AF5, AF9,
};
use stm32f4xx_hal::i2c::I2c;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::stm32;
use stm32f4xx_hal::stm32::{Interrupt, I2C1, I2C3};

use bxcan::{Can, CanConfig, FilterOwner, Frame, Id, Instance, RegisterBlock, StandardId};
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

use usb_device::class_prelude::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::SerialPort;

use embedded_can::Frame as _;
use stm32f4xx_hal::dac::{DacOut, DacPin};
use stm32f4xx_hal::otg_fs::UsbBusType;
use stm32f4xx_hal::time::Hertz;

const SECOND: u32 = 168_000_000;

const BLINK_PERIOD: u32 = SECOND / 4;

trait StepGenerator {
    fn set_frequency<T>(&mut self, freq: T)
    where
        T: Into<Hertz>;
}

trait DirectionSth {}

trait StepCounter {}

// pub struct Can1Instance {
//     peripheral: stm32::CAN1,
// }
//
// impl Can1Instance {
//     pub fn new(can: stm32::CAN1) -> Self {
//         Can1Instance { peripheral: can }
//     }
// }
//
// unsafe impl Instance for Can1Instance {
//     const REGISTERS: *mut RegisterBlock = stm32::CAN1::ptr() as *mut _;
// }
//
// unsafe impl FilterOwner for Can1Instance {
//     const NUM_FILTER_BANKS: u8 = 14;
// }

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: PB<Output<PushPull>>,
        timer: stm32::TIM2,
        tim1: stm32::TIM1,
        serial: SerialPort<'static, UsbBusType>,
        usb_dev: UsbDevice<'static, UsbBusType>,
    }

    #[init(schedule = [blink])]
    fn init(cx: init::Context) -> init::LateResources {
        static mut EP_MEMORY: [u32; 1024] = [0; 1024];
        static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;
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

        // rcc.apb1enr.modify(|_, w| w.can1en().set_bit());
        // rcc.apb1rstr.modify(|_, w| w.can1rst().reset());
        // rcc.apb1rstr.modify(|_, w| w.can1rst().clear_bit());

        // let can = device.CAN1;
        // let mut can = Can::new(Can1Instance::new(can));
        // can.configure(|c| {
        //     c.set_loopback(false);
        //     c.set_silent(false);
        //     c.set_bit_timing(0x001a000b); // maybe try out 0x001c0014
        // });
        //
        // let frame = Frame::new(StandardId::new(0x080).unwrap(), &[]).unwrap();

        let gpioa = device.GPIOA.split();
        let gpiob = device.GPIOB.split();
        let gpioc = device.GPIOC.split();

        let _ = gpiob.pb8.into_alternate_af8();
        let _ = gpiob.pb9.into_alternate_af8();

        let _step1 = gpioa.pa8.into_alternate_af1();

        let mut timer = device.TIM2;

        timer.arr.write(|w| w.arr().bits(20000));

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

        let frequency = 4000; // 2hz
        let pclk_mul = 1;
        let ticks: u32 = 84_000_000 * pclk_mul / frequency;

        let psc = (ticks - 1) / (1 << 16);
        tim1.psc.write(|w| w.psc().bits(psc as u16));

        let arr = ticks / (psc + 1);
        tim1.arr.write(|w| unsafe { w.bits(arr as u32) });
        tim1.dier.write(|w| w.uie().set_bit());

        tim1.cr1.modify(|_, w| w.urs().set_bit());
        tim1.egr.write(|w| w.ug().set_bit());
        tim1.cr1.modify(|_, w| w.urs().clear_bit());

        tim1.bdtr.modify(|_, w| w.aoe().set_bit());

        tim1.cr2.modify(|_, w| w.mms().compare_oc1());

        tim1.ccmr1_output().modify(|_, w| {
            w.cc1s()
                .output()
                .oc1fe()
                .set_bit()
                // .oc1pe()
                // .enabled()
                .oc1m()
                .pwm_mode1()
        });
        tim1.ccer.modify(|_, w| w.cc1e().set_bit());
        tim1.ccr1.modify(|_, w| w.ccr().bits((arr / 2) as u16));

        // start counter
        tim1.cr1
            .modify(|_, w| w.cms().bits(0).opm().disabled().cen().set_bit());

        tim1.ccr1.modify(|_, w| w.ccr().bits((arr / 2) as u16));

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

        // let _in = gpioa.pa0.into_alternate_af1();

        let mut led = gpiob.pb6.into_push_pull_output().downgrade();
        let mut en = gpioa.pa6.into_floating_input().downgrade();
        let mut dac_channel1 = gpioa.pa4.into_analog();
        let mut dac_channel2 = gpioa.pa5.into_analog();

        let (mut dac1, mut dac2) =
            stm32f4xx_hal::dac::dac(device.DAC, (dac_channel1, dac_channel2));

        dac1.enable();
        dac1.set_value(600);

        dac2.enable();
        dac2.set_value(600);

        let now = cx.start;
        cx.schedule.blink(now + BLINK_PERIOD.cycles()).unwrap();

        // defmt::warn!("enabling");
        // nb::block!(can.enable());
        // loop {
        //     defmt::warn!("sending.");
        //     nb::block!(can.transmit(&frame));
        //     led.set_high();
        //     for _ in 0..10_000_000 {
        //         cortex_m::asm::nop();
        //     }
        //     led.set_low();
        // }

        let usb = USB {
            usb_global: device.OTG_FS_GLOBAL,
            usb_device: device.OTG_FS_DEVICE,
            usb_pwrclk: device.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into_alternate_af10(),
            pin_dp: gpioa.pa12.into_alternate_af10(),
            hclk: clocks.hclk(),
        };

        *USB_BUS = Some(UsbBus::new(usb, &mut EP_MEMORY[..]));

        let serial = usbd_serial::SerialPort::new(USB_BUS.as_ref().unwrap());

        let usb_dev = UsbDeviceBuilder::new(USB_BUS.as_ref().unwrap(), UsbVidPid(0x445a, 0x05e1))
            .manufacturer("MH Robotics")
            .product("SM4")
            .serial_number("sm4202101")
            .device_class(usbd_serial::USB_CLASS_CDC)
            .build();

        init::LateResources {
            led,
            timer,
            tim1,
            serial,
            usb_dev,
        }
    }

    #[idle(resources = [])]
    fn main(cx: main::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(binds = OTG_FS, resources = [serial, usb_dev])]
    fn usb_handler(cx: usb_handler::Context) {
        let serial: &mut SerialPort<UsbBusType> = cx.resources.serial;
        let usb_dev: &mut UsbDevice<UsbBusType> = cx.resources.usb_dev;
        if !usb_dev.poll(&mut [serial]) {
            return;
        }
        let mut buf = [0u8; 64];

        match serial.read(&mut buf[..]) {
            Ok(count) => {
                for c in buf[..count].iter() {
                    serial.write(&[*c]).unwrap();
                    defmt::warn!("data {:u8}", c);
                }
                // count bytes were read to &buf[..count]
            }
            Err(UsbError::WouldBlock) => {
                defmt::debug!("would block.");
            } // No data received
            Err(_) => {
                defmt::warn!("err.");
            } // An error occurred
        };
    }

    #[task(resources = [led, timer], schedule = [blink])]
    fn blink(cx: blink::Context) {
        let led: &mut PB<Output<PushPull>> = cx.resources.led;
        let timer: &mut stm32::TIM2 = cx.resources.timer;
        if led.is_low().unwrap() {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }

        defmt::error!("pulses {:u32}", timer.cnt.read().bits());

        cx.schedule
            .blink(cx.scheduled + BLINK_PERIOD.cycles())
            .unwrap();
    }

    #[task(binds = TIM1_UP_TIM10, resources = [tim1, led])]
    fn tim1_handler(cx: tim1_handler::Context) {
        let tim1: &mut stm32::TIM1 = cx.resources.tim1;
        // let led: &mut PC<Output<PushPull>> = cx.resources.led;
        // if led.is_low().unwrap() {
        //     led.set_high().unwrap();
        // } else {
        //     led.set_low().unwrap();
        // }

        tim1.sr.write(|w| w.uif().clear_bit());
        // defmt::error!("trig");
    }

    extern "C" {
        fn EXTI0();
    }
};
