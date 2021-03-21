use crate::canopen::PDODeserializationError;
use core::convert::{TryFrom, TryInto};

/// The `PositionPDO` contains the position information of an axis.
#[derive(Copy, Clone, Default, Debug)]
pub struct PositionPDO {
    pub revolutions: i32,
    pub angle: u32,
}

impl PositionPDO {
    const SIZE: usize = 8;
}

impl TryFrom<&[u8]> for PositionPDO {
    type Error = PDODeserializationError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != Self::SIZE {
            return Err(PDODeserializationError::IncorrectDataSize);
        }

        Ok(Self {
            revolutions: i32::from_le_bytes(value[..4].try_into().unwrap()),
            angle: u32::from_le_bytes(value[4..].try_into().unwrap()),
        })
    }
}

impl PositionPDO {
    pub fn to_raw(&self, raw: &mut [u8]) -> Result<usize, ()> {
        if raw.len() < Self::SIZE {
            return Err(());
        }

        raw[..4].clone_from_slice(&self.revolutions.to_le_bytes());
        raw[4..].clone_from_slice(&self.angle.to_le_bytes());

        Ok(Self::SIZE)
    }
}
