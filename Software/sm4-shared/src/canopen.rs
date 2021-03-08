use core::convert::{TryFrom, TryInto};

#[derive(Copy, Clone, Default, Debug)]
pub struct RxPDO1 {
    pub driver1_speed: f32, // revs per second
    pub driver2_speed: f32, // revs per second
}

impl RxPDO1 {
    const SIZE: usize = 8;
}

impl TryFrom<&[u8]> for RxPDO1 {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != Self::SIZE {
            return Err(());
        }

        Ok(Self {
            driver1_speed: f32::from_le_bytes(value[..4].try_into().unwrap()),
            driver2_speed: f32::from_le_bytes(value[4..].try_into().unwrap()),
        })
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct TxPDO1 {
    pub battery_voltage: u16, // millivolts
    pub temperature: u16,     // 0.1 deg C
}

impl TxPDO1 {
    const SIZE: usize = 4;

    pub fn to_raw(&self, raw: &mut [u8]) -> Result<usize, ()> {
        if raw.len() < Self::SIZE {
            return Err(());
        }

        raw[0..2].clone_from_slice(&self.battery_voltage.to_le_bytes());
        raw[2..4].clone_from_slice(&self.temperature.to_le_bytes());

        Ok(core::mem::size_of::<Self>())
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct TxPDO2 {
    battery_voltage: u16, // millivolts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let valid_raw_data = [0u8; 8];
        assert!(RxPDO1::try_from(valid_raw_data.as_ref()).is_ok());

        let invalid_raw_data = [0u8; 4];
        assert!(RxPDO1::try_from(invalid_raw_data.as_ref()).is_err());
    }
}
