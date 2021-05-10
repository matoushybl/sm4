use crate::canopen::{PDODeserializationError, PDOSerializationError, SerializePDO};
use core::convert::{TryFrom, TryInto};

#[derive(Copy, Clone, Default, Debug)]
pub struct VelocityPDO {
    pub axis1_velocity: f32, // revs per second
    pub axis2_velocity: f32, // revs per second
}

impl VelocityPDO {
    const SIZE: usize = 8;
}

impl TryFrom<&[u8]> for VelocityPDO {
    type Error = PDODeserializationError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != Self::SIZE {
            return Err(PDODeserializationError::IncorrectDataSize);
        }

        Ok(Self {
            axis1_velocity: f32::from_le_bytes(value[..4].try_into().unwrap()),
            axis2_velocity: f32::from_le_bytes(value[4..].try_into().unwrap()),
        })
    }
}

impl SerializePDO for VelocityPDO {
    fn len() -> usize {
        Self::SIZE
    }

    fn to_raw(&self) -> Result<[u8; 8], PDOSerializationError> {
        let mut raw = [0u8; 8];
        if raw.len() < Self::SIZE {
            return Err(PDOSerializationError::BufferTooSmall);
        }

        raw[..4].clone_from_slice(&self.axis1_velocity.to_le_bytes());
        raw[4..].clone_from_slice(&self.axis2_velocity.to_le_bytes());

        Ok(raw)
    }
}
