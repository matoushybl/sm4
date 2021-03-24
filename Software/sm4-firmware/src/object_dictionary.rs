use bxcan::Data;
use sm4_shared::canopen::Key;
use sm4_shared::encoder::{Position, Speed};
use sm4_shared::AxisMode;

const DEFAULT_STANDSTILL_CURRENT: f32 = 0.4;
const DEFAULT_ACCELERATING_CURRENT: f32 = 0.7;
const DEFAULT_CONSTANT_VELOCITY_CURRENT: f32 = 0.6;

/// The object dictionary struct represents the global state of the driver
#[derive(Copy, Clone)]
pub struct ObjectDictionary {
    battery_voltage: f32,
    temperature: f32,
    axis1_mode: AxisMode,
    axis2_mode: AxisMode,
    axis1_enabled: bool,
    axis2_enabled: bool,
    axis1_target_velocity: Speed,
    axis2_target_velocity: Speed,
    axis1_actual_velocity: Speed,
    axis2_actual_velocity: Speed,
    axis1_target_position: Position,
    axis2_target_position: Position,
    axis1_actual_position: Position,
    axis2_actual_position: Position,
    axis1_standstill_current: f32,
    axis2_standstill_current: f32,
    axis1_acceleration_current: f32,
    axis2_acceleration_current: f32,
    axis1_constant_velocity_current: f32,
    axis2_constant_velocity_current: f32,
    axis1_acceleration: f32,
    axis2_acceleration: f32,
    axis1_velocity_p: f32,
    axis2_velocity_p: f32,
    axis1_velocity_s: f32,
    axis2_velocity_s: f32,
    axis1_velocity_d: f32,
    axis2_velocity_d: f32,
    axis1_velocity_feedback_control_enable: bool,
    axis2_velocity_feedback_control_enable: bool,
    axis1_position_p: f32,
    axis2_position_p: f32,
    axis1_position_s: f32,
    axis2_position_s: f32,
    axis1_position_d: f32,
    axis2_position_d: f32,
}

impl ObjectDictionary {
    pub fn new(resolution: u16) -> Self {
        Self {
            battery_voltage: 0.0,
            temperature: 0.0,
            axis1_mode: Default::default(),
            axis2_mode: Default::default(),
            axis1_enabled: false,
            axis2_enabled: false,
            axis1_target_velocity: Speed::zero(),
            axis2_target_velocity: Speed::zero(),
            axis1_actual_velocity: Speed::zero(),
            axis2_actual_velocity: Speed::zero(),
            axis1_target_position: Position::zero(resolution),
            axis2_target_position: Position::zero(resolution),
            axis1_actual_position: Position::zero(resolution),
            axis2_actual_position: Position::zero(resolution),
            axis1_standstill_current: DEFAULT_STANDSTILL_CURRENT,
            axis2_standstill_current: DEFAULT_STANDSTILL_CURRENT,
            axis1_acceleration_current: DEFAULT_ACCELERATING_CURRENT,
            axis2_acceleration_current: DEFAULT_ACCELERATING_CURRENT,
            axis1_constant_velocity_current: DEFAULT_CONSTANT_VELOCITY_CURRENT,
            axis2_constant_velocity_current: DEFAULT_CONSTANT_VELOCITY_CURRENT,
            axis1_acceleration: 1.0,
            axis2_acceleration: 1.0,
            axis1_velocity_p: 3.0,
            axis2_velocity_p: 3.0,
            axis1_velocity_s: 1.0,
            axis2_velocity_s: 1.0,
            axis1_velocity_d: 0.0,
            axis2_velocity_d: 0.0,
            axis1_velocity_feedback_control_enable: false,
            axis2_velocity_feedback_control_enable: false,
            axis1_position_p: 1.0,
            axis2_position_p: 1.0,
            axis1_position_s: 0.0,
            axis2_position_s: 0.0,
            axis1_position_d: 0.0,
            axis2_position_d: 0.0,
        }
    }

    pub fn update_from_raw(&mut self, index: u16, subindex: Key, raw: Data) -> Result<(), ()> {
        Ok(())
    }
}

impl ObjectDictionary {
    pub fn axis1_mode(&self) -> AxisMode {
        self.axis1_mode
    }

    pub fn axis2_mode(&self) -> AxisMode {
        self.axis2_mode
    }

    pub fn axis1_enabled(&self) -> bool {
        self.axis1_enabled
    }

    pub fn axis2_enabled(&self) -> bool {
        self.axis2_enabled
    }

    pub fn axis1_target_velocity(&self) -> Speed {
        self.axis1_target_velocity
    }

    pub fn axis2_target_velocity(&self) -> Speed {
        self.axis2_target_velocity
    }

    pub fn axis1_actual_velocity(&self) -> Speed {
        self.axis1_actual_velocity
    }

    pub fn axis2_actual_velocity(&self) -> Speed {
        self.axis2_actual_velocity
    }

    pub fn axis1_target_position(&self) -> Position {
        self.axis1_target_position
    }

    pub fn axis2_target_position(&self) -> Position {
        self.axis2_target_position
    }

    pub fn axis1_actual_position(&self) -> Position {
        self.axis1_actual_position
    }

    pub fn axis2_actual_position(&self) -> Position {
        self.axis2_actual_position
    }

    pub fn axis1_standstill_current(&self) -> f32 {
        self.axis1_standstill_current
    }

    pub fn axis2_standstill_current(&self) -> f32 {
        self.axis2_standstill_current
    }

    pub fn axis1_acceleration_current(&self) -> f32 {
        self.axis1_acceleration_current
    }

    pub fn axis2_acceleration_current(&self) -> f32 {
        self.axis2_acceleration_current
    }

    pub fn axis1_constant_velocity_current(&self) -> f32 {
        self.axis1_constant_velocity_current
    }

    pub fn axis2_constant_velocity_current(&self) -> f32 {
        self.axis2_constant_velocity_current
    }

    pub fn axis1_acceleration(&self) -> f32 {
        self.axis1_acceleration
    }

    pub fn axis2_acceleration(&self) -> f32 {
        self.axis2_acceleration
    }

    pub fn set_axis1_mode(&mut self, axis1_mode: AxisMode) {
        self.axis1_mode = axis1_mode;
    }

    pub fn set_axis2_mode(&mut self, axis2_mode: AxisMode) {
        self.axis2_mode = axis2_mode;
    }

    pub fn set_axis1_enabled(&mut self, axis1_enabled: bool) {
        self.axis1_enabled = axis1_enabled;
    }

    pub fn set_axis2_enabled(&mut self, axis2_enabled: bool) {
        self.axis2_enabled = axis2_enabled;
    }

    pub fn set_axis1_target_velocity(&mut self, axis1_target_velocity: Speed) {
        self.axis1_target_velocity = axis1_target_velocity;
    }

    pub fn set_axis2_target_velocity(&mut self, axis2_target_velocity: Speed) {
        self.axis2_target_velocity = axis2_target_velocity;
    }

    pub fn set_axis1_actual_velocity(&mut self, axis1_actual_velocity: Speed) {
        self.axis1_actual_velocity = axis1_actual_velocity;
    }

    pub fn set_axis2_actual_velocity(&mut self, axis2_actual_velocity: Speed) {
        self.axis2_actual_velocity = axis2_actual_velocity;
    }

    pub fn set_axis1_target_position(&mut self, axis1_target_position: Position) {
        self.axis1_target_position = axis1_target_position;
    }

    pub fn set_axis2_target_position(&mut self, axis2_target_position: Position) {
        self.axis2_target_position = axis2_target_position;
    }

    pub fn set_axis1_actual_position(&mut self, axis1_actual_position: Position) {
        self.axis1_actual_position = axis1_actual_position;
    }

    pub fn set_axis2_actual_position(&mut self, axis2_actual_position: Position) {
        self.axis2_actual_position = axis2_actual_position;
    }

    pub fn set_axis1_standstill_current(&mut self, axis1_standstill_current: f32) {
        self.axis1_standstill_current = axis1_standstill_current;
    }

    pub fn set_axis2_standstill_current(&mut self, axis2_standstill_current: f32) {
        self.axis2_standstill_current = axis2_standstill_current;
    }

    pub fn set_axis1_acceleration_current(&mut self, axis1_acceleration_current: f32) {
        self.axis1_acceleration_current = axis1_acceleration_current;
    }

    pub fn set_axis2_acceleration_current(&mut self, axis2_acceleration_current: f32) {
        self.axis2_acceleration_current = axis2_acceleration_current;
    }

    pub fn set_axis1_constant_velocity_current(&mut self, axis1_constant_velocity_current: f32) {
        self.axis1_constant_velocity_current = axis1_constant_velocity_current;
    }

    pub fn set_axis2_constant_velocity_current(&mut self, axis2_constant_velocity_current: f32) {
        self.axis2_constant_velocity_current = axis2_constant_velocity_current;
    }

    pub fn set_axis1_acceleration(&mut self, axis1_acceleration: f32) {
        self.axis1_acceleration = axis1_acceleration;
    }

    pub fn set_axis2_acceleration(&mut self, axis2_acceleration: f32) {
        self.axis2_acceleration = axis2_acceleration;
    }

    pub fn axis1_velocity_p(&self) -> f32 {
        self.axis1_velocity_p
    }

    pub fn axis2_velocity_p(&self) -> f32 {
        self.axis2_velocity_p
    }

    pub fn axis1_velocity_s(&self) -> f32 {
        self.axis1_velocity_s
    }

    pub fn axis2_velocity_s(&self) -> f32 {
        self.axis2_velocity_s
    }

    pub fn axis1_velocity_d(&self) -> f32 {
        self.axis1_velocity_d
    }

    pub fn axis2_velocity_d(&self) -> f32 {
        self.axis2_velocity_d
    }

    pub fn axis1_position_p(&self) -> f32 {
        self.axis1_position_p
    }

    pub fn axis2_position_p(&self) -> f32 {
        self.axis2_position_p
    }

    pub fn axis1_position_s(&self) -> f32 {
        self.axis1_position_s
    }

    pub fn axis2_position_s(&self) -> f32 {
        self.axis2_position_s
    }

    pub fn axis1_position_d(&self) -> f32 {
        self.axis1_position_d
    }

    pub fn axis2_position_d(&self) -> f32 {
        self.axis2_position_d
    }

    pub fn set_axis1_velocity_p(&mut self, axis1_velocity_p: f32) {
        self.axis1_velocity_p = axis1_velocity_p;
    }

    pub fn set_axis2_velocity_p(&mut self, axis2_velocity_p: f32) {
        self.axis2_velocity_p = axis2_velocity_p;
    }

    pub fn set_axis1_velocity_s(&mut self, axis1_velocity_s: f32) {
        self.axis1_velocity_s = axis1_velocity_s;
    }

    pub fn set_axis2_velocity_s(&mut self, axis2_velocity_s: f32) {
        self.axis2_velocity_s = axis2_velocity_s;
    }

    pub fn set_axis1_velocity_d(&mut self, axis1_velocity_d: f32) {
        self.axis1_velocity_d = axis1_velocity_d;
    }

    pub fn set_axis2_velocity_d(&mut self, axis2_velocity_d: f32) {
        self.axis2_velocity_d = axis2_velocity_d;
    }

    pub fn set_axis1_position_p(&mut self, axis1_position_p: f32) {
        self.axis1_position_p = axis1_position_p;
    }

    pub fn set_axis2_position_p(&mut self, axis2_position_p: f32) {
        self.axis2_position_p = axis2_position_p;
    }

    pub fn set_axis1_position_s(&mut self, axis1_position_s: f32) {
        self.axis1_position_s = axis1_position_s;
    }

    pub fn set_axis2_position_s(&mut self, axis2_position_s: f32) {
        self.axis2_position_s = axis2_position_s;
    }

    pub fn set_axis1_position_d(&mut self, axis1_position_d: f32) {
        self.axis1_position_d = axis1_position_d;
    }

    pub fn set_axis2_position_d(&mut self, axis2_position_d: f32) {
        self.axis2_position_d = axis2_position_d;
    }

    pub fn axis1_velocity_feedback_control_enable(&self) -> bool {
        self.axis1_velocity_feedback_control_enable
    }

    pub fn axis2_velocity_feedback_control_enable(&self) -> bool {
        self.axis2_velocity_feedback_control_enable
    }

    pub fn battery_voltage(&self) -> f32 {
        self.battery_voltage
    }

    pub fn temperature(&self) -> f32 {
        self.temperature
    }

    pub fn set_battery_voltage(&mut self, battery_voltage: f32) {
        self.battery_voltage = battery_voltage;
    }

    pub fn set_temperature(&mut self, temperature: f32) {
        self.temperature = temperature;
    }

    pub fn set_axis1_velocity_feedback_control_enable(
        &mut self,
        axis1_velocity_feedback_control_enable: bool,
    ) {
        self.axis1_velocity_feedback_control_enable = axis1_velocity_feedback_control_enable;
    }

    pub fn set_axis2_velocity_feedback_control_enable(
        &mut self,
        axis2_velocity_feedback_control_enable: bool,
    ) {
        self.axis2_velocity_feedback_control_enable = axis2_velocity_feedback_control_enable;
    }
}
