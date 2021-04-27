use crate::canopen::{PDODeserializationError, PDOSerializationError, SerializePDO};
use core::convert::{TryFrom, TryInto};

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
}

impl SerializePDO for TxPDO1 {
    fn len() -> usize {
        Self::SIZE
    }

    fn to_raw(&self) -> Result<[u8; 8], PDOSerializationError> {
        let mut raw = [0u8; 8];
        if raw.len() < Self::SIZE {
            return Err(PDOSerializationError::BufferTooSmall);
        }

        raw[..2].clone_from_slice(&self.battery_voltage.to_le_bytes());
        raw[2..4].clone_from_slice(&self.temperature.to_le_bytes());

        Ok(raw)
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
