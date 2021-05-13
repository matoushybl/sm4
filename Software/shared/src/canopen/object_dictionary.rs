use crate::models::{Axis, AxisMode, Position, Velocity};
use crate::psd::ControllerSettings;
use core::convert::TryFrom;

/// Trait for Object Dictionary abstraction
pub trait ObjectDictionary<const RESOLUTION: u32> {
    /// Returns battery voltage stored in the Object Dictionary.
    fn battery_voltage(&self) -> f32;
    /// Returns temperature stored in the Object Dictionary.
    fn temperature(&self) -> f32;
    /// Sets the battery voltage value in the Object Dictionary.
    fn set_battery_voltage(&mut self, battery_voltage: f32);
    /// Sets the temperature value in the Object Dictionary.
    fn set_temperature(&mut self, temperature: f32);
    /// Returns the configuration of a specific axis.
    fn axis(&self, axis: Axis) -> &dyn AxisDictionary<RESOLUTION>;
    /// Returns a mutable reference the configuration of a specific axis.
    /// This is the only way an axis configuration can be changed
    fn axis_mut(&mut self, axis: Axis) -> &mut dyn AxisDictionary<RESOLUTION>;
}

pub trait AxisDictionary<const RESOLUTION: u32> {
    fn mode(&self) -> AxisMode;
    fn enabled(&self) -> bool;
    fn target_velocity(&self) -> Velocity;
    fn actual_velocity(&self) -> Velocity;
    fn target_position(&self) -> Position<{ RESOLUTION }>;
    fn actual_position(&self) -> Position<{ RESOLUTION }>;
    fn current(&self) -> CurrentSettings;
    fn velocity_controller_settings(&self) -> ControllerSettings;
    fn position_controller_settings(&self) -> ControllerSettings;
    fn velocity_feedback_control_enabled(&self) -> bool;
    fn set_mode(&mut self, mode: AxisMode);
    fn set_enabled(&mut self, enabled: bool);
    fn set_target_velocity(&mut self, target_velocity: Velocity);
    fn set_actual_velocity(&mut self, actual_velocity: Velocity);
    fn set_target_position(&mut self, target_position: Position<RESOLUTION>);
    fn set_actual_position(&mut self, actual_position: Position<RESOLUTION>);

    fn set_accelerating_current(&mut self, current: f32);
    fn set_standstill_current(&mut self, current: f32);
    fn set_constant_velocity_current(&mut self, current: f32);

    fn set_velocity_controller_p(&mut self, value: f32);
    fn set_velocity_controller_s(&mut self, value: f32);
    fn set_velocity_controller_d(&mut self, value: f32);
    fn set_velocity_controller_max_output(&mut self, value: f32);

    fn set_position_controller_p(&mut self, value: f32);
    fn set_position_controller_s(&mut self, value: f32);
    fn set_position_controller_d(&mut self, value: f32);
    fn set_position_controller_max_output(&mut self, value: f32);

    fn set_velocity_feedback_control_enabled(&mut self, velocity_feedback_control_enabled: bool);
    fn acceleration(&self) -> f32;
    fn set_acceleration(&mut self, acceleration: f32);
}

pub trait ObjectDictionaryKey {
    fn raw(&self) -> u16;
}

#[derive(Copy, Clone)]
pub enum AxisKey {
    Mode,
    Enabled,
    TargetVelocity,
    ActualVelocity,
    TargetPosition,
    ActualPosition,
    Acceleration,
    VelocityFeedbackControlEnabled,
    AcceleratingCurrent,
    StandStillCurrent,
    ConstantVelocityCurrent,
    VelocityP,
    VelocityS,
    VelocityD,
    VelocityMaxAction,
    PositionP,
    PositionS,
    PositionD,
    PositionMaxAction,
}

impl ObjectDictionaryKey for AxisKey {
    fn raw(&self) -> u16 {
        match self {
            AxisKey::Mode => 0x01,
            AxisKey::Enabled => 0x02,
            AxisKey::TargetVelocity => 0x03,
            AxisKey::ActualVelocity => 0x04,
            AxisKey::TargetPosition => 0x05,
            AxisKey::ActualPosition => 0x06,
            AxisKey::Acceleration => 0x07,
            AxisKey::VelocityFeedbackControlEnabled => 0x08,
            AxisKey::AcceleratingCurrent => 0x09,
            AxisKey::StandStillCurrent => 0x0a,
            AxisKey::ConstantVelocityCurrent => 0x0b,
            AxisKey::VelocityP => 0x0c,
            AxisKey::VelocityS => 0x0d,
            AxisKey::VelocityD => 0x0e,
            AxisKey::VelocityMaxAction => 0x0f,
            AxisKey::PositionP => 0x10,
            AxisKey::PositionS => 0x11,
            AxisKey::PositionD => 0x12,
            AxisKey::PositionMaxAction => 0x13,
        }
    }
}

impl TryFrom<u8> for AxisKey {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(AxisKey::Mode),
            0x02 => Ok(AxisKey::Enabled),
            0x03 => Ok(AxisKey::TargetVelocity),
            0x04 => Ok(AxisKey::ActualVelocity),
            0x05 => Ok(AxisKey::TargetPosition),
            0x06 => Ok(AxisKey::ActualPosition),
            0x07 => Ok(AxisKey::Acceleration),
            0x08 => Ok(AxisKey::VelocityFeedbackControlEnabled),
            0x09 => Ok(AxisKey::AcceleratingCurrent),
            0x0a => Ok(AxisKey::StandStillCurrent),
            0x0b => Ok(AxisKey::ConstantVelocityCurrent),
            0x0c => Ok(AxisKey::VelocityP),
            0x0d => Ok(AxisKey::VelocityS),
            0x0e => Ok(AxisKey::VelocityD),
            0x0f => Ok(AxisKey::VelocityMaxAction),
            0x10 => Ok(AxisKey::PositionP),
            0x11 => Ok(AxisKey::PositionS),
            0x12 => Ok(AxisKey::PositionD),
            0x13 => Ok(AxisKey::PositionMaxAction),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Key {
    BatteryVoltage,
    Temperature,
    Axis1(AxisKey),
    Axis2(AxisKey),
}

impl Key {
    fn parse(index: u16, subindex: u8) -> Option<Key> {
        let index = index & 0xff00;
        match index {
            0x0000 => match subindex {
                0x01 => Some(Key::BatteryVoltage),
                0x02 => Some(Key::Temperature),
                _ => None,
            },
            0x6100 => AxisKey::try_from(subindex).map_or(None, |k| Some(Key::Axis1(k))),
            0x6200 => AxisKey::try_from(subindex).map_or(None, |k| Some(Key::Axis2(k))),
            _ => None,
        }
    }

    fn parse_i2c(value: u8) -> Option<Key> {
        None
    }

    fn offset(&self) -> u16 {
        match self {
            Key::BatteryVoltage => 0x0000,
            Key::Temperature => 0x0000,
            Key::Axis1(_) => 0x6100,
            Key::Axis2(_) => 0x6200,
        }
    }

    pub fn key_for_axis(key: AxisKey, axis: Axis) -> Self {
        match axis {
            Axis::Axis1 => Self::Axis1(key),
            Axis::Axis2 => Self::Axis2(key),
        }
    }
}

impl ObjectDictionaryKey for Key {
    fn raw(&self) -> u16 {
        match self {
            Key::BatteryVoltage => 0x0001,
            Key::Temperature => 0x0002,
            Key::Axis1(key) => self.offset() + key.raw(),
            Key::Axis2(key) => self.offset() + key.raw(),
        }
    }
}

pub trait ObjectDictionaryStorage {
    fn save_f32<KEY: ObjectDictionaryKey>(&mut self, key: KEY, value: f32);
    fn save_bool<KEY: ObjectDictionaryKey>(&mut self, key: KEY, value: bool);
    fn load_f32<KEY: ObjectDictionaryKey>(&self, key: KEY) -> Option<f32>;
    fn load_bool<KEY: ObjectDictionaryKey>(&self, key: KEY) -> Option<bool>;
}

#[derive(Copy, Clone)]
pub struct CurrentSettings {
    standstill_current: f32,
    accelerating_current: f32,
    constant_velocity_current: f32,
}

impl CurrentSettings {
    pub fn new(
        standstill_current: f32,
        accelerating_current: f32,
        constant_velocity_current: f32,
    ) -> Self {
        Self {
            standstill_current,
            accelerating_current,
            constant_velocity_current,
        }
    }

    pub fn standstill_current(&self) -> f32 {
        self.standstill_current
    }
    pub fn accelerating_current(&self) -> f32 {
        self.accelerating_current
    }
    pub fn constant_velocity_current(&self) -> f32 {
        self.constant_velocity_current
    }

    pub fn set_standstill_current(&mut self, standstill_current: f32) {
        self.standstill_current = standstill_current;
    }
    pub fn set_accelerating_current(&mut self, accelerating_current: f32) {
        self.accelerating_current = accelerating_current;
    }
    pub fn set_constant_velocity_current(&mut self, constant_velocity_current: f32) {
        self.constant_velocity_current = constant_velocity_current;
    }
}

impl Default for CurrentSettings {
    fn default() -> Self {
        Self {
            standstill_current: 0.4,
            accelerating_current: 0.7,
            constant_velocity_current: 0.6,
        }
    }
}
