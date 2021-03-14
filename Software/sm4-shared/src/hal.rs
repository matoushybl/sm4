use embedded_time::rate::Hertz;

pub trait StepGenerator {
    fn set_step_frequency(&mut self, frequency: Hertz);
}

pub trait DACChannel {
    fn set_output_voltage(&mut self, voltage: u16);
}
