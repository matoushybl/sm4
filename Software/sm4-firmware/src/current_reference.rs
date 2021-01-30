use crate::board::{CurrentRef1Channel, CurrentRef1Pin, CurrentRef2Channel, CurrentRef2Pin};
use sm4_shared::{CurrentReference, Motor1, Motor2};
use stm32f4xx_hal::dac::{DacOut, DacPin};
use stm32f4xx_hal::stm32;

// struct Driver<M, G, C, R> {}
//
// impl<M, G, C, R> Driver<M, G, C, R> where R: CurrentReference<M> {}

pub struct Reference<CH> {
    channel: CH,
}

macro_rules! reference {
    ($ch:ident, $new:ident) => {
        impl Reference<$ch>
        where
            $ch: DacOut<u16>,
        {
            fn $new(mut channel: $ch) -> Self {
                channel.enable();
                Self { channel }
            }
        }
    };
}

reference!(CurrentRef1Channel, new_ref1);
reference!(CurrentRef2Channel, new_ref2);

pub fn initialize_current_ref(
    dac: stm32::DAC,
    pin1: CurrentRef1Pin,
    pin2: CurrentRef2Pin,
) -> (Reference<CurrentRef1Channel>, Reference<CurrentRef2Channel>) {
    let (ref1, ref2) = stm32f4xx_hal::dac::dac(dac, (pin1, pin2));
    (Reference::new_ref1(ref1), Reference::new_ref2(ref2))
}

impl CurrentReference<Motor1> for Reference<CurrentRef1Channel> {
    fn set_current(&mut self, current: u16) {
        self.channel.set_value(current);
    }
}

impl CurrentReference<Motor2> for Reference<CurrentRef2Channel> {
    fn set_current(&mut self, current: u16) {
        self.channel.set_value(current);
    }
}
