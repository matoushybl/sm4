use crate::canopen::{PDODeserializationError, PDOSerializationError, SerializePDO};
use crate::models::AxisMode;
use core::convert::TryFrom;

#[derive(Copy, Clone, Default, Debug)]
pub struct RxPDO1 {
    pub axis1_mode: AxisMode,
    pub axis2_mode: AxisMode,
    pub axis1_enabled: bool,
    pub axis2_enabled: bool,
}

impl RxPDO1 {
    const SIZE: usize = 2;
}

impl TryFrom<&[u8]> for RxPDO1 {
    type Error = PDODeserializationError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != Self::SIZE {
            return Err(PDODeserializationError::IncorrectDataSize);
        }

        Ok(RxPDO1 {
            axis1_mode: AxisMode::from(value[0]),
            axis2_mode: AxisMode::from(value[0] >> 4),
            axis1_enabled: value[1] & 0x01 > 0,
            axis2_enabled: value[1] & 0x02 > 0,
        })
    }
}

impl SerializePDO for RxPDO1 {
    fn len() -> usize {
        Self::SIZE
    }

    fn to_raw(&self) -> Result<[u8; 8], PDOSerializationError> {
        let mut raw = [0u8; 8];
        if raw.len() < Self::SIZE {
            return Err(PDOSerializationError::BufferTooSmall);
        }

        raw[0] = u8::from(self.axis1_mode) | u8::from(self.axis2_mode) << 4;
        raw[1] = if self.axis1_enabled { 1 } else { 0 } | if self.axis2_enabled { 2 } else { 0 };

        Ok(raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let valid_raw_data = [0u8; 2];
        assert!(RxPDO1::try_from(valid_raw_data.as_ref()).is_ok());

        let invalid_raw_data = [0u8; 4];
        assert!(RxPDO1::try_from(invalid_raw_data.as_ref()).is_err());
    }
}
