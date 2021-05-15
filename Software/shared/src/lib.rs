//! The shared library for the SM4 - dual channel stepper motor controller.
//!
//! This shared library contains the abstractions for motor control and common data structures
//! for interfacing with the control software.

#![cfg_attr(not(test), no_std)]

mod canopen;
mod encoder;
mod hal;
mod models;
mod motion_controller;
mod psd;
mod ramp;
mod tmc2100;
mod usb_protocol;

pub mod prelude {
    pub use crate::canopen::*;
    pub use crate::encoder::*;
    pub use crate::hal::*;
    pub use crate::models::{Axis, AxisMode, Position, Velocity};
    pub use crate::motion_controller::AxisMotionController;
    pub use crate::psd::PSDController;
    pub use crate::ramp::TrapRampGen;
    pub use crate::tmc2100::TMC2100;
    pub use crate::usb_protocol::*;
    pub use crate::OnError;
}

pub trait OnError {
    fn on_error<F: FnOnce(&Self) -> ()>(&self, closure: F)
    where
        Self: Sized;
}

impl<T, E> OnError for Result<T, E> {
    fn on_error<F: FnOnce(&Self)>(&self, closure: F)
    where
        Self: Sized,
    {
        if self.is_err() {
            closure(self)
        }
    }
}
