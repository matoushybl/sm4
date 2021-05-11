use sm4_shared::prelude::*;

pub trait IObjectDictionary<const RESOLUTION: u32> {
    fn axis1(&self) -> &dyn AxisDictionary<RESOLUTION>;

    fn axis1_mut(&mut self) -> &mut dyn AxisDictionary<RESOLUTION>;

    fn axis2(&self) -> &dyn AxisDictionary<RESOLUTION>;

    fn axis2_mut(&mut self) -> &mut dyn AxisDictionary<RESOLUTION>;
}

/// The object dictionary struct represents the global state of the driver
#[derive(Copy, Clone)]
pub struct ObjectDictionary<STORAGE: ObjectDictionaryStorage, const RESOLUTION: u32> {
    battery_voltage: f32,
    temperature: f32,
    axis1: PersistentStoreAxisDictionary<STORAGE, RESOLUTION>,
    axis2: PersistentStoreAxisDictionary<STORAGE, RESOLUTION>,
}

impl<STORAGE: ObjectDictionaryStorage, const RESOLUTION: u32>
    ObjectDictionary<STORAGE, RESOLUTION>
{
    pub fn new(storage: STORAGE) -> Self {
        Self {
            battery_voltage: 0.0,
            temperature: 0.0,
            axis1: PersistentStoreAxisDictionary::new(storage),
            axis2: PersistentStoreAxisDictionary::new(storage),
        }
    }
}

impl<STORAGE: ObjectDictionaryStorage, const RESOLUTION: u32> IObjectDictionary<RESOLUTION>
    for ObjectDictionary<STORAGE, RESOLUTION>
{
    fn axis1(&self) -> &dyn AxisDictionary<RESOLUTION> {
        &self.axis1
    }

    fn axis1_mut(&mut self) -> &mut dyn AxisDictionary<RESOLUTION> {
        &mut self.axis1
    }

    fn axis2(&self) -> &dyn AxisDictionary<RESOLUTION> {
        &self.axis2
    }

    fn axis2_mut(&mut self) -> &mut dyn AxisDictionary<RESOLUTION> {
        &mut self.axis2
    }
}

impl<STORAGE: ObjectDictionaryStorage, const RESOLUTION: u32>
    ObjectDictionary<STORAGE, RESOLUTION>
{
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
