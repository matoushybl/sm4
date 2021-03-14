use crate::float::fabs;
use crate::hal::{DACChannel, StepGenerator};
use crate::StepperDriver;
use embedded_time::rate::Hertz;

const V_FS: f32 = 0.32; // V
const R_OFFSET: f32 = 0.02; // Ohm
const MAX_V_REF: u16 = 2500; // mV

pub struct TMC2100<G, STEP, DIR, DAC> {
    generator: G,
    step_pin: STEP,
    dir_pin: DIR,
    current_dac: DAC,
    sense_r: f32,
}

impl<G, STEP, DIR, DAC> TMC2100<G, STEP, DIR, DAC>
where
    G: StepGenerator,
    DIR: embedded_hal::digital::v2::OutputPin,
    DAC: DACChannel,
{
    pub fn new(generator: G, step_pin: STEP, dir_pin: DIR, current_dac: DAC, sense_r: f32) -> Self {
        let mut s = Self {
            generator,
            step_pin,
            dir_pin,
            current_dac,
            sense_r,
        };

        s.set_current(0.2);

        s
    }
}

impl<G, STEP, DIR, DAC> StepperDriver for TMC2100<G, STEP, DIR, DAC>
where
    G: StepGenerator,
    DIR: embedded_hal::digital::v2::OutputPin,
    DAC: DACChannel,
{
    fn set_output_frequency(&mut self, frequency: f32) {
        if frequency < 0.0 {
            self.dir_pin.set_high();
        } else {
            self.dir_pin.set_low();
        };

        self.generator
            .set_step_frequency(Hertz::new((fabs(frequency) * 16.0 * 200.0) as u32))
    }

    fn set_current(&mut self, current: f32) {
        let voltage = (self::float::fabs(current) * MAX_V_REF as f32 / V_FS
            * (self.sense_r + R_OFFSET)
            / 0.707) as u16;
        self.current_dac.set_output_voltage(voltage.min(MAX_V_REF));
    }
}
