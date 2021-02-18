use crate::board::{CurrentRef1Channel, CurrentRef1Pin, CurrentRef2Channel, CurrentRef2Pin};
use sm4_shared::{CurrentReference, Motor1, Motor2};
use stm32f4xx_hal::dac::{DacOut, DacPin};
use stm32f4xx_hal::stm32;

const V_FS: f32 = 0.32; // V
const R_SENSE: f32 = 0.220; // Ohm
const R_OFFSET: f32 = 0.02; // Ohm
const MAX_V_REF: u16 = 2500; // mV

pub struct Reference<CH> {
    channel: CH,
}

macro_rules! reference {
    ($motor:ident, $ch:ident, $new:ident) => {
        impl Reference<$ch>
        where
            $ch: DacOut<u16>,
        {
            fn $new(mut channel: $ch) -> Self {
                channel.enable();
                Self { channel }
            }
        }

        impl CurrentReference<$motor> for Reference<$ch> {
            fn set_current(&mut self, current: f32) {
                let voltage = (crate::float::fabs(current) * MAX_V_REF as f32 / V_FS
                    * (R_SENSE + R_OFFSET)
                    / 0.707) as u16;
                self.channel.set_value(voltage.min(MAX_V_REF));
            }
        }
    };
}

reference!(Motor1, CurrentRef1Channel, new_ref1);
reference!(Motor2, CurrentRef2Channel, new_ref2);

pub fn initialize_current_ref(
    dac: stm32::DAC,
    pin1: CurrentRef1Pin,
    pin2: CurrentRef2Pin,
) -> (Reference<CurrentRef1Channel>, Reference<CurrentRef2Channel>) {
    let (ref1, ref2) = stm32f4xx_hal::dac::dac(dac, (pin1, pin2));
    (Reference::new_ref1(ref1), Reference::new_ref2(ref2))
}
