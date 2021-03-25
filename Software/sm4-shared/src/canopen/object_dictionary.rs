use crate::prelude::{Position, Speed};
use crate::AxisMode;

pub enum Key {
    Axis1Mode,
    Axis2Mode,
    Axis1Enabled,
    Axis2Enabled,
    Axis1TargetVelocity,
    Axis2TargetVelocity,
    Axis1ActualVelocity,
    Axis2ActualVelocity,
    Axis1SetVelocity,
    Axis2SetVelocity,
    Axis1TargetPosition,
    Axis2TargetPosition,
    Axis1ActualPosition,
    Axis2ActualPosition,
    Axis1StandstillCurrent,
    Axis2StandstillCurrent,
    Axis1AcceleratingCurrent,
    Axis2AcceleratingCurrent,
    Axis1ConstantVelocityCurrent,
    Axis2ConstantVelocityCurrent,
    Axis1Acceleration,
    Axis2Acceleration,
    // TODO ramp profiles
}

impl Key {
    // pub fn data_len(&self) -> usize {}
    //
    // pub fn is_persistent() -> bool {}
}

impl From<u8> for Key {
    fn from(raw: u8) -> Self {
        unimplemented!()
    }
}

impl From<Key> for u8 {
    fn from(key: Key) -> Self {
        unimplemented!()
    }
}

pub struct CurrentSettings {
    standstill_current: f32,
    accelerating_current: f32,
    constant_velocity_current: f32,
}

pub struct AxisDictionary {
    mode: AxisMode,
    enabled: bool,
    target_velocity: Speed,
    actual_velocity: Speed,
    target_position: Position,
    actual_position: Position,
    current: CurrentSettings,
    velocity_controller_settings: ControllerSettings,
    position_controller_settings: ControllerSettings,
    velocity_feedback_control_enabled: bool,
}
