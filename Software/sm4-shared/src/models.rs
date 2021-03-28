//! This module contains data objects used in the shared API.

/// `AxisMode` enum represents the control mode of an axis - either velocity control or position control
/// In raw data, the [Self::Velocity] variant is represented as a zero and the [Self::Position] variant is represented as 1.
/// The variant [Self::Velocity] is the default.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum AxisMode {
    Velocity,
    Position,
}

/// By default, the driver's axis mode shall be [Self::Velocity].
impl Default for AxisMode {
    fn default() -> Self {
        Self::Velocity
    }
}

/// Used to implement `AxisMode` deserialization from a single bit (the lowest bit in a byte).
impl From<u8> for AxisMode {
    fn from(raw: u8) -> Self {
        match raw & 0x01 {
            1 => AxisMode::Position,
            _ => AxisMode::Velocity,
        }
    }
}

/// Used for serialization as a single bit (the lowest bit in a byte).
impl From<AxisMode> for u8 {
    fn from(raw: AxisMode) -> Self {
        match raw {
            AxisMode::Velocity => 0x00,
            AxisMode::Position => 0x01,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::AxisMode;

    #[test]
    fn axis_mode_deserialize() {
        let raw = 10u8;
        assert_eq!(AxisMode::from(raw), AxisMode::Velocity);

        let raw = 11u8;
        assert_eq!(AxisMode::from(raw), AxisMode::Position);
    }

    #[test]
    fn axis_mode_serialize() {
        assert_eq!(u8::from(AxisMode::Velocity), 0u8);
        assert_eq!(u8::from(AxisMode::Position), 1u8);
    }
}
