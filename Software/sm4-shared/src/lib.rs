//! The shared library for the SM4 - dual channel stepper motor controller.
//!
//! This shared library contains the abstractions for motor control and common data structures
//! for interfacing with the control software.

#![cfg_attr(not(test), no_std)]

mod canopen;
mod encoder;
mod float;
mod hal;
mod motion_controller;
mod psd;
mod ramp;
mod tmc2100;

pub mod prelude {
    pub use crate::canopen::*;
    pub use crate::encoder::*;
    pub use crate::hal::*;
    pub use crate::motion_controller::AxisMotionController;
    pub use crate::psd::PSDController;
    pub use crate::ramp::TrapRampGen;
    pub use crate::tmc2100::TMC2100;
    pub use crate::AxisMode;
}

/// `AxisMode` enum represents the control mode of an axis - either velocity control or position control
#[derive(Copy, Clone, Debug)]
pub enum AxisMode {
    Velocity,
    Position,
}

impl Default for AxisMode {
    fn default() -> Self {
        Self::Velocity
    }
}

impl From<u8> for AxisMode {
    fn from(raw: u8) -> Self {
        match raw & 0x01 {
            1 => AxisMode::Position,
            _ => AxisMode::Velocity,
        }
    }
}

impl From<AxisMode> for u8 {
    fn from(raw: AxisMode) -> Self {
        match raw {
            AxisMode::Velocity => 0x00,
            AxisMode::Position => 0x01,
        }
    }
}
