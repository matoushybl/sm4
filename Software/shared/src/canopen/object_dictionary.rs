use crate::models::{Axis, AxisMode, Position, Velocity};
use crate::psd::ControllerSettings;

pub trait ObjectDictionary<const RESOLUTION: u32> {
    fn battery_voltage(&self) -> f32;
    fn temperature(&self) -> f32;
    fn set_battery_voltage(&mut self, battery_voltage: f32);
    fn set_temperature(&mut self, temperature: f32);

    fn axis(&self, axis: Axis) -> &dyn AxisDictionary<RESOLUTION>;
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
    fn set_current(&mut self, current: CurrentSettings);
    fn set_velocity_controller_settings(
        &mut self,
        velocity_controller_settings: ControllerSettings,
    );
    fn set_position_controller_settings(
        &mut self,
        position_controller_settings: ControllerSettings,
    );
    fn set_velocity_feedback_control_enabled(&mut self, velocity_feedback_control_enabled: bool);
    fn acceleration(&self) -> f32;
    fn set_acceleration(&mut self, acceleration: f32);
}

pub trait ObjectDictionaryKey {
    fn raw(&self) -> u16;
}

#[derive(Copy, Clone)]
pub enum Key {
    Acceleration,
    VelocityFeedbackControlEnabled,
    AcceleratingCurrent,
    StandStillCurrent,
    ConstantVelocityCurrent,
}

impl Key {
    fn value(&self) -> u16 {
        match self {
            Key::Acceleration => 0x01,
            Key::VelocityFeedbackControlEnabled => 0x02,
            Key::AcceleratingCurrent => 0x03,
            Key::StandStillCurrent => 0x04,
            Key::ConstantVelocityCurrent => 0x05,
        }
    }

    pub fn for_axis(&self, axis: Axis) -> u16 {
        axis.object_dictionary_offset() + self.value()
    }
}

impl ObjectDictionaryKey for u16 {
    fn raw(&self) -> u16 {
        *self
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
    pub fn standstill_current(&self) -> f32 {
        self.standstill_current
    }
    pub fn accelerating_current(&self) -> f32 {
        self.accelerating_current
    }
    pub fn constant_velocity_current(&self) -> f32 {
        self.constant_velocity_current
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
