use crate::board::BatteryVoltage;
use stm32f4xx_hal::adc::config::{AdcConfig, Dma, SampleTime, Scan, Sequence};
use stm32f4xx_hal::adc::{Adc, Temperature};
use stm32f4xx_hal::dma::traits::{Channel, Stream};
use stm32f4xx_hal::dma::{Channel0, Stream0};
use stm32f4xx_hal::pac;
use stm32f4xx_hal::stm32::DMA2;

pub struct Monitoring {
    adc: Adc<pac::ADC1>,
}

impl Monitoring {
    pub fn new(adc: pac::ADC1, battery_voltage: BatteryVoltage, mut dma: Stream0<DMA2>) -> Self {
        let adc_config = AdcConfig::default()
            .dma(Dma::Continuous)
            .scan(Scan::Enabled);
        let mut adc = Adc::adc1(adc, true, adc_config);
        adc.configure_channel(&Temperature, Sequence::One, SampleTime::Cycles_480);
        adc.configure_channel(&battery_voltage, Sequence::Two, SampleTime::Cycles_480);

        dma.set_channel(Channel0::new());

        dma.set_peripheral_address(adc.data_register_address());
        unsafe { dma.set_peripheral_size(adc.sequence_length()) }

        dma.set_memory_address(); // TODO address to a statically allocated buffer

        adc.enable_temperature_and_vref();
        adc.enable();
        Self { adc }
    }
}
