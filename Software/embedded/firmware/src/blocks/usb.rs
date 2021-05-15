use core::convert::TryInto;
use crc_all::Crc;
use sm4_shared::{
    prelude::{USBMessage, USBProtocolConsumer},
    OnError,
};
use stm32f4xx_hal::otg_fs::*;
use stm32f4xx_hal::stm32;

use crate::{
    board::definitions::{USBDMinus, USBDPlus},
    can::CANOpenMessage,
};
use stm32f4xx_hal::rcc::Clocks;
use usb_device::class_prelude::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::SerialPort;

const BUFFER_SIZE: usize = 20;

pub struct USBProtocol {
    serial: SerialPort<'static, UsbBusType>,
    usb_dev: UsbDevice<'static, UsbBusType>,
    buffer: [u8; BUFFER_SIZE],
    buffer_index: usize,
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
            UsbDeviceBuilder::new(USB_BUS.as_ref().unwrap(), UsbVidPid(0x16c0, 0x05e1))
                .manufacturer("MH Robotics")
                .product("SM4")
                .serial_number("sm4202101")
                .device_class(usbd_serial::USB_CLASS_CDC)
                .build()
        };

        Self {
            serial,
            usb_dev,
            buffer: [0u8; BUFFER_SIZE],
            buffer_index: 0,
        }
    }

    pub fn process_interrupt(&mut self) -> Option<USBMessage> {
        if !self.usb_dev.poll(&mut [&mut self.serial]) {
            return None;
        }
        let mut buf = [0u8; 64];

        match self.serial.read(&mut buf[..]) {
            Ok(count) => {
                if count + self.buffer_index < BUFFER_SIZE {
                    self.buffer[self.buffer_index..].copy_from_slice(&buf[..count]);
                    self.buffer_index += count;
                } else {
                    self.buffer.fill(0);
                    self.buffer_index = 0;
                    self.buffer.copy_from_slice(&buf[..count]);
                }

                for c in buf[..count].iter() {
                    // self.serial.write(&[*c]).unwrap();
                    defmt::warn!("data {:x}", c);
                }

                self.process_data()
            }
            Err(UsbError::WouldBlock) => {
                defmt::debug!("would block.");
                None
            } // No data received
            Err(_) => {
                defmt::warn!("err.");
                None
            } // An error occurred
        }
    }

    pub fn send(&mut self, message: USBMessage) {
        match message {
            USBMessage::Request(_, _) => {
                defmt::error!("Sending requests is not yet implemented.");
            }
            USBMessage::Transfer(index, subindex, length, data) => {
                let mut buffer = [0u8; 11];
                buffer[0] = 0x55;
                buffer[1] = 0x21;
                buffer[2..4].copy_from_slice(&index.to_le_bytes());
                buffer[4] = subindex;
                buffer[5] = length;
                buffer[6..][..length as usize].copy_from_slice(&data[..length as usize]);

                let mut crc = crc_all::Crc::<u8>::new(0x31, 8, 0xff, 0x00, false);
                crc.update(&buffer[..(6 + length as usize)]);
                buffer[6 + length as usize] = crc.finish();

                self.serial
                    .write(&buffer[..6 + length as usize + 1])
                    .on_error(|_| defmt::error!("Failed to write data to USB serial."));
            }
        }
    }
}

impl USBProtocolConsumer for USBProtocol {
    fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn buffer_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    fn buffer_index(&self) -> usize {
        self.buffer_index
    }

    fn buffer_index_mut(&mut self) -> &mut usize {
        &mut self.buffer_index
    }

    fn max_length() -> usize {
        BUFFER_SIZE
    }
}
