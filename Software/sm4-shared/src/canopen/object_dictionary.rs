use crate::models::AxisMode;
use crate::prelude::{Position, Velocity};
use crate::psd::ControllerSettings;

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

#[derive(Copy, Clone)]
pub struct CurrentSettings {
    standstill_current: f32,
    accelerating_current: f32,
    constant_velocity_current: f32,
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

#[derive(Copy, Clone)]
pub struct AxisDictionary {
    mode: AxisMode,
    enabled: bool,
    target_velocity: Velocity,
    actual_velocity: Velocity,
    target_position: Position,
    actual_position: Position,
    current: CurrentSettings,
    velocity_controller_settings: ControllerSettings,
    position_controller_settings: ControllerSettings,
    velocity_feedback_control_enabled: bool,
    acceleration: f32,
}

impl AxisDictionary {
    pub fn new(resolution: u16) -> Self {
        Self {
            mode: Default::default(),
            enabled: false,
            target_velocity: Velocity::zero(),
            actual_velocity: Velocity::zero(),
            target_position: Position::zero(resolution),
            actual_position: Position::zero(resolution),
            current: CurrentSettings::default(),
            velocity_controller_settings: Default::default(),
            position_controller_settings: Default::default(),
            velocity_feedback_control_enabled: false,
            acceleration: 1.0,
        }
    }
}

impl AxisDictionary {
    pub fn mode(&self) -> AxisMode {
        self.mode
    }
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    pub fn target_velocity(&self) -> Velocity {
        self.target_velocity
    }
    pub fn actual_velocity(&self) -> Velocity {
        self.actual_velocity
    }
    pub fn target_position(&self) -> Position {
        self.target_position
    }
    pub fn actual_position(&self) -> Position {
        self.actual_position
    }
    pub fn current(&self) -> CurrentSettings {
        self.current
    }
    pub fn velocity_controller_settings(&self) -> ControllerSettings {
        self.velocity_controller_settings
    }
    pub fn position_controller_settings(&self) -> ControllerSettings {
        self.position_controller_settings
    }
    pub fn velocity_feedback_control_enabled(&self) -> bool {
        self.velocity_feedback_control_enabled
    }

    pub fn set_mode(&mut self, mode: AxisMode) {
        self.mode = mode;
    }
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    pub fn set_target_velocity(&mut self, target_velocity: Velocity) {
        self.target_velocity = target_velocity;
    }
    pub fn set_actual_velocity(&mut self, actual_velocity: Velocity) {
        self.actual_velocity = actual_velocity;
    }
    pub fn set_target_position(&mut self, target_position: Position) {
        self.target_position = target_position;
    }
    pub fn set_actual_position(&mut self, actual_position: Position) {
        self.actual_position = actual_position;
    }
    pub fn set_current(&mut self, current: CurrentSettings) {
        self.current = current;
    }
    pub fn set_velocity_controller_settings(
        &mut self,
        velocity_controller_settings: ControllerSettings,
    ) {
        self.velocity_controller_settings = velocity_controller_settings;
    }
    pub fn set_position_controller_settings(
        &mut self,
        position_controller_settings: ControllerSettings,
    ) {
        self.position_controller_settings = position_controller_settings;
    }
    pub fn set_velocity_feedback_control_enabled(
        &mut self,
        velocity_feedback_control_enabled: bool,
    ) {
        self.velocity_feedback_control_enabled = velocity_feedback_control_enabled;
    }
    pub fn acceleration(&self) -> f32 {
        self.acceleration
    }

    pub fn set_acceleration(&mut self, acceleration: f32) {
        self.acceleration = acceleration;
    }
}
