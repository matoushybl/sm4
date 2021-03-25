use stm32f4xx_hal::otg_fs::*;
use stm32f4xx_hal::stm32;

use crate::board::definitions::{USBDMinus, USBDPlus};
use stm32f4xx_hal::rcc::Clocks;
use usb_device::class_prelude::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::SerialPort;

pub struct USBProtocol {
    serial: SerialPort<'static, UsbBusType>,
    usb_dev: UsbDevice<'static, UsbBusType>,
}

impl USBProtocol {
    pub fn new(
        usb_global: stm32::OTG_FS_GLOBAL,
        usb_device: stm32::OTG_FS_DEVICE,
        usb_pwrclk: stm32::OTG_FS_PWRCLK,
        minus: USBDMinus,
        plus: USBDPlus,
        clocks: Clocks,
    ) -> Self {
        static mut EP_MEMORY: [u32; 1024] = [0; 1024];
        static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;

        let usb = USB {
            usb_global,
            usb_device,
            usb_pwrclk,
            pin_dm: minus,
            pin_dp: plus,
            hclk: clocks.hclk(),
        };

        unsafe {
            USB_BUS = Some(UsbBus::new(usb, &mut EP_MEMORY[..]));
        }

        let serial = unsafe { usbd_serial::SerialPort::new(USB_BUS.as_ref().unwrap()) };

        let usb_dev = unsafe {
            UsbDeviceBuilder::new(USB_BUS.as_ref().unwrap(), UsbVidPid(0x445a, 0x05e1))
                .manufacturer("MH Robotics")
                .product("SM4")
                .serial_number("sm4202101")
                .device_class(usbd_serial::USB_CLASS_CDC)
                .build()
        };

        Self { serial, usb_dev }
    }

    pub fn process_interrupt(&mut self) {
        if !self.usb_dev.poll(&mut [&mut self.serial]) {
            return;
        }
        let mut buf = [0u8; 64];

        match self.serial.read(&mut buf[..]) {
            Ok(count) => {
                for c in buf[..count].iter() {
                    self.serial.write(&[*c]).unwrap();
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
}
