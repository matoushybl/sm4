use crate::board::definitions::BatteryVoltage;
use stm32f4xx_hal::adc::config::{AdcConfig, Dma, SampleTime, Scan, Sequence};
use stm32f4xx_hal::adc::{Adc, Temperature};
use stm32f4xx_hal::dma::config::DmaConfig;
use stm32f4xx_hal::dma::{Channel0, PeripheralToMemory, Stream0, Transfer};
use stm32f4xx_hal::pac;
use stm32f4xx_hal::stm32::{ADC1, DMA2};

use cortex_m::singleton;
use stm32f4xx_hal::signature::{VtempCal110, VtempCal30};

pub struct Monitoring {
    transfer:
        Transfer<Stream0<DMA2>, Channel0, Adc<ADC1>, PeripheralToMemory, &'static mut [u16; 2]>,
    temperature: f32,
    battery_voltage: f32,
    transfer_ongoing: bool,
    buffer: Option<&'static mut [u16; 2]>,
}

impl Monitoring {
    pub fn new(raw_adc: pac::ADC1, battery_voltage: BatteryVoltage, dma: Stream0<DMA2>) -> Self {
        let config = DmaConfig::default()
            .transfer_complete_interrupt(true)
            .memory_increment(true)
            .double_buffer(false);

        let adc_config = AdcConfig::default()
            .dma(Dma::Continuous)
            .scan(Scan::Enabled);
        let mut adc = Adc::adc1(raw_adc, true, adc_config);
        adc.configure_channel(&Temperature, Sequence::One, SampleTime::Cycles_480);
        adc.configure_channel(&battery_voltage, Sequence::Two, SampleTime::Cycles_480);
        adc.enable_temperature_and_vref();

        let first_buffer = singleton!(: [u16; 2] = [0; 2]).unwrap();
        let second_buffer = Some(cortex_m::singleton!(: [u16; 2] = [0; 2]).unwrap());
        let transfer = Transfer::init(dma, adc, first_buffer, None, config);

        let mut s = Self {
            transfer,
            temperature: 0.0,
            battery_voltage: 0.0,
            transfer_ongoing: false,
            buffer: second_buffer,
        };

        s.poll();

        s
    }

    pub fn poll(&mut self) {
        if self.transfer_ongoing {
            return;
        }
        self.transfer.start(|adc| {
            adc.start_conversion();
        });
        self.transfer_ongoing = true;
    }

    pub fn transfer_complete(&mut self) {
        let (buffer, _) = self
            .transfer
            .next_transfer(self.buffer.take().unwrap())
            .unwrap();

        let raw_temp = buffer[0];
        let raw_volt = buffer[1];

        self.buffer = Some(buffer);

        self.transfer_ongoing = false;

        let cal30 = VtempCal30::get().read() as f32;
        let cal110 = VtempCal110::get().read() as f32;

        self.temperature = (110.0 - 30.0) * ((raw_temp as f32) - cal30) / (cal110 - cal30) + 30.0;
        self.battery_voltage = (raw_volt as f32) / ((2_i32.pow(12) - 1) as f32) * 3.3 / 4.7 * 104.7;
    }

    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }

    pub fn get_battery_voltage(&self) -> f32 {
        self.battery_voltage
    }
}
