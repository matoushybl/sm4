use bxcan::Data;
use sm4_shared::prelude::*;

const DEFAULT_STANDSTILL_CURRENT: f32 = 0.4;
const DEFAULT_ACCELERATING_CURRENT: f32 = 0.7;
const DEFAULT_CONSTANT_VELOCITY_CURRENT: f32 = 0.6;

/// The object dictionary struct represents the global state of the driver
#[derive(Copy, Clone)]
pub struct ObjectDictionary {
    battery_voltage: f32,
    temperature: f32,
    axis1: AxisDictionary,
    axis2: AxisDictionary,
}

impl ObjectDictionary {
    pub fn new(resolution: u16) -> Self {
        Self {
            battery_voltage: 0.0,
            temperature: 0.0,
            axis1: AxisDictionary::new(resolution),
            axis2: AxisDictionary::new(resolution),
        }
    }

    pub fn update_from_raw(&mut self, index: u16, subindex: Key, raw: Data) -> Result<(), ()> {
        Ok(())
    }
}

impl ObjectDictionary {
    pub fn axis1(&self) -> &AxisDictionary {
        &self.axis1
    }

    pub fn axis1_mut(&mut self) -> &mut AxisDictionary {
        &mut self.axis1
    }

    pub fn axis2(&self) -> &AxisDictionary {
        &self.axis2
    }

    pub fn axis2_mut(&mut self) -> &mut AxisDictionary {
        &mut self.axis2
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
}
