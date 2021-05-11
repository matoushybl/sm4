use embedded_time::rate::Hertz;

/// This trait is an abstraction over hardware/software that is capable of generating square wave signal of specific frequency.
/// It is generally implemented by timers.
pub trait StepGenerator {
    /// Sets output frequency of the generator.
    ///
    /// # Arguments
    /// * `frequency` - frequency of the output square wave signal
    fn set_step_frequency(&mut self, frequency: Hertz);
}

pub trait DACChannel {
    fn set_output_voltage(&mut self, voltage: u16);
}

/// This trait is an abstraction over stepper drivers.
/// Generally the drivers have two functions - generate steps and set output current.
pub trait StepperDriver {
    /// Sets output frequency of the driver.
    /// this shall be the angular frequency of the output shaft in revolutions per second.
    ///
    /// # Arguments
    /// * `frequency` - frequency of the output motor shaft in revolutions per second
    fn set_output_frequency(&mut self, frequency: f32);

    /// Sets the target current the driver shall drive the stepper motor with.
    ///
    /// # Arguments
    /// * `current` - the desired current in Amps
    fn set_current(&mut self, current: f32);
}
