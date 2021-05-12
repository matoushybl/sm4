use crate::canopen::object_dictionary::{AxisKey, CurrentSettings, Key, ObjectDictionary};
use crate::canopen::ObjectDictionaryStorage;
use crate::prelude::*;
use crate::psd::ControllerSettings;
use core::cell::RefCell;
use spin::Mutex;

/// The object dictionary struct represents the global state of the driver
#[derive(Copy, Clone)]
pub struct PersistentStoreObjectDictionary<
    STORAGE: 'static + ObjectDictionaryStorage,
    const RESOLUTION: u32,
> {
    battery_voltage: f32,
    temperature: f32,
    axis1: PersistentStoreAxisDictionary<STORAGE, RESOLUTION>,
    axis2: PersistentStoreAxisDictionary<STORAGE, RESOLUTION>,
}

impl<STORAGE: 'static + ObjectDictionaryStorage, const RESOLUTION: u32>
    PersistentStoreObjectDictionary<STORAGE, RESOLUTION>
{
    pub fn new(storage: &'static Mutex<RefCell<STORAGE>>) -> Self {
        Self {
            battery_voltage: 0.0,
            temperature: 0.0,
            axis1: PersistentStoreAxisDictionary::new(Axis::Axis1, storage),
            axis2: PersistentStoreAxisDictionary::new(Axis::Axis2, storage),
        }
    }
}

impl<STORAGE: 'static + ObjectDictionaryStorage, const RESOLUTION: u32> ObjectDictionary<RESOLUTION>
    for PersistentStoreObjectDictionary<STORAGE, RESOLUTION>
{
    fn battery_voltage(&self) -> f32 {
        self.battery_voltage
    }

    fn temperature(&self) -> f32 {
        self.temperature
    }

    fn set_battery_voltage(&mut self, battery_voltage: f32) {
        self.battery_voltage = battery_voltage;
    }

    fn set_temperature(&mut self, temperature: f32) {
        self.temperature = temperature;
    }

    fn axis(&self, axis: Axis) -> &dyn AxisDictionary<RESOLUTION> {
        match axis {
            Axis::Axis1 => &self.axis1,
            Axis::Axis2 => &self.axis2,
        }
    }

    fn axis_mut(&mut self, axis: Axis) -> &mut dyn AxisDictionary<RESOLUTION> {
        match axis {
            Axis::Axis1 => &mut self.axis1,
            Axis::Axis2 => &mut self.axis2,
        }
    }
}

#[derive(Copy, Clone)]
pub struct PersistentStoreAxisDictionary<
    STORAGE: 'static + ObjectDictionaryStorage,
    const RESOLUTION: u32,
> {
    axis: Axis,
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
    storage: &'static Mutex<RefCell<STORAGE>>,
}

impl<STORAGE: 'static + ObjectDictionaryStorage, const RESOLUTION: u32>
    PersistentStoreAxisDictionary<STORAGE, RESOLUTION>
{
    pub fn new(axis: Axis, storage: &'static Mutex<RefCell<STORAGE>>) -> Self {
        let acceleration = storage
            .lock()
            .borrow()
            .load_f32(Key::key_for_axis(AxisKey::Acceleration, axis))
            .unwrap_or(50.0);
        let velocity_feedback_control_enabled = storage
            .lock()
            .borrow()
            .load_bool(Key::key_for_axis(
                AxisKey::VelocityFeedbackControlEnabled,
                axis,
            ))
            .unwrap_or(false);
        let current = CurrentSettings::new(
            storage
                .lock()
                .borrow()
                .load_f32(Key::key_for_axis(AxisKey::StandStillCurrent, axis))
                .unwrap_or(0.4),
            storage
                .lock()
                .borrow()
                .load_f32(Key::key_for_axis(AxisKey::AcceleratingCurrent, axis))
                .unwrap_or(0.7),
            storage
                .lock()
                .borrow()
                .load_f32(Key::key_for_axis(AxisKey::ConstantVelocityCurrent, axis))
                .unwrap_or(0.6),
        );
        let velocity_controller_settings = ControllerSettings::new(
            storage
                .lock()
                .borrow()
                .load_f32(Key::key_for_axis(AxisKey::VelocityP, axis))
                .unwrap_or(1.0),
            storage
                .lock()
                .borrow()
                .load_f32(Key::key_for_axis(AxisKey::VelocityS, axis))
                .unwrap_or(0.1),
            storage
                .lock()
                .borrow()
                .load_f32(Key::key_for_axis(AxisKey::VelocityD, axis))
                .unwrap_or(0.0),
            storage
                .lock()
                .borrow()
                .load_f32(Key::key_for_axis(AxisKey::VelocityMaxAction, axis))
                .unwrap_or(3.0),
        );
        let position_controller_settings = ControllerSettings::new(
            storage
                .lock()
                .borrow()
                .load_f32(Key::key_for_axis(AxisKey::PositionP, axis))
                .unwrap_or(3.0),
            storage
                .lock()
                .borrow()
                .load_f32(Key::key_for_axis(AxisKey::PositionS, axis))
                .unwrap_or(0.001),
            storage
                .lock()
                .borrow()
                .load_f32(Key::key_for_axis(AxisKey::PositionD, axis))
                .unwrap_or(0.0001),
            storage
                .lock()
                .borrow()
                .load_f32(Key::key_for_axis(AxisKey::PositionMaxAction, axis))
                .unwrap_or(3.0),
        );
        Self {
            axis,
            mode: Default::default(),
            enabled: false,
            target_velocity: Velocity::zero(),
            actual_velocity: Velocity::zero(),
            target_position: Position::zero(),
            actual_position: Position::zero(),
            current,
            velocity_controller_settings,
            position_controller_settings,
            velocity_feedback_control_enabled,
            acceleration,
            storage,
        }
    }
}

impl<STORAGE: 'static + ObjectDictionaryStorage, const RESOLUTION: u32>
    AxisDictionary<{ RESOLUTION }> for PersistentStoreAxisDictionary<STORAGE, RESOLUTION>
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
    fn set_accelerating_current(&mut self, current: f32) {
        self.current.set_accelerating_current(current);
        self.storage.lock().borrow_mut().save_f32(
            Key::key_for_axis(AxisKey::AcceleratingCurrent, self.axis),
            current,
        );
    }
    fn set_standstill_current(&mut self, current: f32) {
        self.current.set_standstill_current(current);
        self.storage.lock().borrow_mut().save_f32(
            Key::key_for_axis(AxisKey::StandStillCurrent, self.axis),
            current,
        );
    }
    fn set_constant_velocity_current(&mut self, current: f32) {
        self.current.set_constant_velocity_current(current);
        self.storage.lock().borrow_mut().save_f32(
            Key::key_for_axis(AxisKey::ConstantVelocityCurrent, self.axis),
            current,
        );
    }

    fn set_velocity_controller_p(&mut self, value: f32) {
        self.velocity_controller_settings.set_proportional(value);
        self.storage
            .lock()
            .borrow_mut()
            .save_f32(Key::key_for_axis(AxisKey::VelocityP, self.axis), value);
    }

    fn set_velocity_controller_s(&mut self, value: f32) {
        self.velocity_controller_settings.set_integral(value);
        self.storage
            .lock()
            .borrow_mut()
            .save_f32(Key::key_for_axis(AxisKey::VelocityS, self.axis), value);
    }

    fn set_velocity_controller_d(&mut self, value: f32) {
        self.velocity_controller_settings.set_derivative(value);
        self.storage
            .lock()
            .borrow_mut()
            .save_f32(Key::key_for_axis(AxisKey::VelocityD, self.axis), value);
    }

    fn set_velocity_controller_max_output(&mut self, value: f32) {
        self.velocity_controller_settings
            .set_max_output_amplitude(value);
        self.storage.lock().borrow_mut().save_f32(
            Key::key_for_axis(AxisKey::VelocityMaxAction, self.axis),
            value,
        );
    }

    fn set_position_controller_p(&mut self, value: f32) {
        self.position_controller_settings.set_proportional(value);
        self.storage
            .lock()
            .borrow_mut()
            .save_f32(Key::key_for_axis(AxisKey::PositionP, self.axis), value);
    }

    fn set_position_controller_s(&mut self, value: f32) {
        self.position_controller_settings.set_integral(value);
        self.storage
            .lock()
            .borrow_mut()
            .save_f32(Key::key_for_axis(AxisKey::PositionS, self.axis), value);
    }

    fn set_position_controller_d(&mut self, value: f32) {
        self.position_controller_settings.set_derivative(value);
        self.storage
            .lock()
            .borrow_mut()
            .save_f32(Key::key_for_axis(AxisKey::PositionD, self.axis), value);
    }

    fn set_position_controller_max_output(&mut self, value: f32) {
        self.position_controller_settings
            .set_max_output_amplitude(value);
        self.storage.lock().borrow_mut().save_f32(
            Key::key_for_axis(AxisKey::PositionMaxAction, self.axis),
            value,
        );
    }

    fn set_velocity_feedback_control_enabled(&mut self, velocity_feedback_control_enabled: bool) {
        self.velocity_feedback_control_enabled = velocity_feedback_control_enabled;
        self.storage.lock().borrow_mut().save_bool(
            Key::key_for_axis(AxisKey::VelocityFeedbackControlEnabled, self.axis),
            velocity_feedback_control_enabled,
        );
    }

    fn acceleration(&self) -> f32 {
        self.acceleration
    }

    fn set_acceleration(&mut self, acceleration: f32) {
        self.acceleration = acceleration;
        self.storage.lock().borrow_mut().save_f32(
            Key::key_for_axis(AxisKey::Acceleration, self.axis),
            acceleration,
        );
    }
}
