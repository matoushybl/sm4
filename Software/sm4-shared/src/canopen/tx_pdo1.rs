use core::convert::{TryFrom, TryInto};
use crate::canopen::PDODeserializationError;

/// `TxPDO1` represents the first Process Data Object sent by the device to the master.
/// The PDO is reserved for general status information only.
/// As of now it contains the information about the motor supply voltage and die temperature.
#[derive(Copy, Clone, Default)]
pub struct TxPDO1 {
    /// The motor supply temperature. In millivolts.
    pub battery_voltage: u16,
    /// The STM32F4 die temperature. In 0.1 deg C.
    pub temperature: u16,
}

impl TxPDO1 {
    const SIZE: usize = 4;

    pub fn to_raw(&self, raw: &mut [u8]) -> Result<usize, ()> {
        if raw.len() < Self::SIZE {
            return Err(());
        }

        raw[..2].clone_from_slice(&self.battery_voltage.to_le_bytes());
        raw[2..4].clone_from_slice(&self.temperature.to_le_bytes());

        Ok(Self::SIZE)
    }
}

impl TryFrom<&[u8]> for TxPDO1 {
    type Error = PDODeserializationError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != Self::SIZE {
            return Err(PDODeserializationError::IncorrectDataSize);
        }

        Ok(TxPDO1 {
            battery_voltage: u16::from_le_bytes(value[..2].try_into().unwrap()),
            temperature: u16::from_le_bytes(value[2..4].try_into().unwrap()),
        })
    }
}
