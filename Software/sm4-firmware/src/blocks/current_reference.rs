use crate::board::definitions::*;
use sm4_shared::prelude::DACChannel;
use stm32f4xx_hal::dac::{DacOut, DacPin};
use stm32f4xx_hal::stm32;

pub struct CurrentDACChannel<CH> {
    channel: CH,
}

impl<CH> CurrentDACChannel<CH>
where
    CH: DacOut<u16> + DacPin,
{
    fn new(mut channel: CH) -> Self {
        channel.enable();
        Self { channel }
    }
}

impl<CH> DACChannel for CurrentDACChannel<CH>
where
    CH: DacOut<u16>,
{
    fn set_output_voltage(&mut self, voltage: u16) {
        self.channel.set_value(voltage);
    }
}

pub fn initialize_current_ref(
    dac: stm32::DAC,
    pin1: CurrentRef1Pin,
    pin2: CurrentRef2Pin,
) -> (
    CurrentDACChannel<CurrentRef1Channel>,
    CurrentDACChannel<CurrentRef2Channel>,
) {
    let (ref1, ref2) = stm32f4xx_hal::dac::dac(dac, (pin1, pin2));
    (CurrentDACChannel::new(ref1), CurrentDACChannel::new(ref2))
}
