use crate::models::AxisMode;
use crate::prelude::{Position, Velocity};
use crate::psd::ControllerSettings;

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

pub trait ObjectDictionaryStorage {
    fn save_f32(&mut self, value: f32);
    fn load_f32(&mut self) -> Option<f32>;
}

#[derive(Copy, Clone)]
pub struct PersistentStoreAxisDictionary<STORAGE: ObjectDictionaryStorage, const RESOLUTION: u32> {
    mode: AxisMode,
    enabled: bool,
    target_velocity: Velocity,
    actual_velocity: Velocity,
    target_position: Position<RESOLUTION>,
    actual_position: Position<RESOLUTION>,
    current: CurrentSettings,
    velocity_controller_settings: ControllerSettings,
    position_controller_settings: ControllerSettings,
    velocity_feedback_control_enabled: bool,
    acceleration: f32,
    storage: STORAGE,
}

impl<STORAGE: ObjectDictionaryStorage, const RESOLUTION: u32>
    PersistentStoreAxisDictionary<STORAGE, RESOLUTION>
{
    pub fn new(storage: STORAGE) -> Self {
        Self {
            mode: Default::default(),
            enabled: false,
            target_velocity: Velocity::zero(),
            actual_velocity: Velocity::zero(),
            target_position: Position::zero(),
            actual_position: Position::zero(),
            current: CurrentSettings::default(),
            velocity_controller_settings: ControllerSettings::new(1.0, 0.1, 0.0, 3.0),
            position_controller_settings: ControllerSettings::new(3.0, 0.001, 0.0001, 3.0),
            velocity_feedback_control_enabled: false,
            acceleration: 50.0,
            storage,
        }
    }
}

impl<STORAGE: ObjectDictionaryStorage, const RESOLUTION: u32> AxisDictionary<{ RESOLUTION }>
    for PersistentStoreAxisDictionary<STORAGE, RESOLUTION>
{
    fn mode(&self) -> AxisMode {
        self.mode
    }
    fn enabled(&self) -> bool {
        self.enabled
    }
    fn target_velocity(&self) -> Velocity {
        self.target_velocity
    }
    fn actual_velocity(&self) -> Velocity {
        self.actual_velocity
    }
    fn target_position(&self) -> Position<RESOLUTION> {
        self.target_position
    }
    fn actual_position(&self) -> Position<RESOLUTION> {
        self.actual_position
    }
    fn current(&self) -> CurrentSettings {
        self.current
    }
    fn velocity_controller_settings(&self) -> ControllerSettings {
        self.velocity_controller_settings
    }
    fn position_controller_settings(&self) -> ControllerSettings {
        self.position_controller_settings
    }
    fn velocity_feedback_control_enabled(&self) -> bool {
        self.velocity_feedback_control_enabled
    }

    fn set_mode(&mut self, mode: AxisMode) {
        self.mode = mode;
    }
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    fn set_target_velocity(&mut self, target_velocity: Velocity) {
        self.target_velocity = target_velocity;
    }
    fn set_actual_velocity(&mut self, actual_velocity: Velocity) {
        self.actual_velocity = actual_velocity;
    }
    fn set_target_position(&mut self, target_position: Position<RESOLUTION>) {
        self.target_position = target_position;
    }
    fn set_actual_position(&mut self, actual_position: Position<RESOLUTION>) {
        self.actual_position = actual_position;
    }
    fn set_current(&mut self, current: CurrentSettings) {
        self.current = current;
    }
    fn set_velocity_controller_settings(
        &mut self,
        velocity_controller_settings: ControllerSettings,
    ) {
        self.velocity_controller_settings = velocity_controller_settings;
    }
    fn set_position_controller_settings(
        &mut self,
        position_controller_settings: ControllerSettings,
    ) {
        self.position_controller_settings = position_controller_settings;
    }
    fn set_velocity_feedback_control_enabled(&mut self, velocity_feedback_control_enabled: bool) {
        self.velocity_feedback_control_enabled = velocity_feedback_control_enabled;
    }
    fn acceleration(&self) -> f32 {
        self.acceleration
    }

    fn set_acceleration(&mut self, acceleration: f32) {
        self.acceleration = acceleration;
    }
}
