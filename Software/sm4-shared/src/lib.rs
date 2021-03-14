//! The shared library for the SM4 - dual channel stepper motor controller.
//!
//! This shared library contains the abstractions for motor control and common data structures
//! for interfacing with the control software.

#![cfg_attr(not(test), no_std)]

pub mod canopen;
pub mod encoder;
pub mod float;
pub mod hal;
pub mod tmc2100;

/// This enum represents the direction where the motor is turning when looking at the shaft.
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Clockwise
    }
}

impl Direction {
    /// Returns the opposite direction. `CounterClockwise` when `Clockwise` is selected and vice-versa.
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Clockwise => Direction::CounterClockwise,
            Direction::CounterClockwise => Direction::CounterClockwise,
        }
    }

    /// In motor control the direction of rotation is usually denoted by a positive or negative number.
    /// This method returns `1` for `Clockwise` and `-1` for `CounterClockwise`
    pub fn multiplier(&self) -> i32 {
        match self {
            Direction::Clockwise => 1,
            Direction::CounterClockwise => -1,
        }
    }
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
