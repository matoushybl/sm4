//! This module contains the definition of objects that are used for CANOpen communication.
//! There are the PDO definitions and the object dictionary.
//! Some of the PDOs are abstracted into the PositionPDO or VelocityPDO to keep the code DRY.

mod object_dictionary;
mod position_pdo;
mod rx_pdo1;
mod tx_pdo1;
mod velocity_pdo;

pub use pdos::{RxPDO1, RxPDO2, RxPDO3, RxPDO4, TxPDO1, TxPDO2, TxPDO3, TxPDO4};

use core::convert::TryFrom;
pub use object_dictionary::Key;

mod pdos {
    use crate::canopen::position_pdo::PositionPDO;
    use crate::canopen::velocity_pdo::VelocityPDO;

    pub use crate::canopen::rx_pdo1::RxPDO1;
    pub use crate::canopen::tx_pdo1::TxPDO1;

    pub type RxPDO2 = VelocityPDO;
    pub type RxPDO3 = PositionPDO;
    pub type RxPDO4 = PositionPDO;

    pub type TxPDO2 = VelocityPDO;
    pub type TxPDO3 = PositionPDO;
    pub type TxPDO4 = PositionPDO;
}

/// Error returned from deserializing the raw data from CAN frames.
pub enum PDODeserializationError {
    /// The received data length is not the same as the length required to deserialize the frame.
    IncorrectDataSize,
}

/// Error returned from serializing the PDO to raw data for CAN frames.
pub enum PDOSerializationError {
    /// The provided buffer into which the raw data should be serialized is too small.
    BufferTooSmall,
}

pub enum NMTRequestedState {
    Operational,
    Stopped,
    PreOperational,
    ResetNode,
    ResetCommunication,
}

#[derive(Copy, Clone, PartialEq)]
pub enum NMTState {
    BootUp,
    Stopped,
    Operational,
    PreOperational,
}

impl Default for NMTState {
    fn default() -> Self {
        NMTState::BootUp
    }
}

impl From<NMTState> for u8 {
    fn from(raw: NMTState) -> Self {
        match raw {
            NMTState::BootUp => 0x00,
            NMTState::Stopped => 0x04,
            NMTState::Operational => 0x05,
            NMTState::PreOperational => 0x7f,
        }
    }
}

impl TryFrom<u8> for NMTRequestedState {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Operational),
            0x02 => Ok(Self::Stopped),
            0x80 => Ok(Self::PreOperational),
            0x81 => Ok(Self::ResetNode),
            0x82 => Ok(Self::ResetCommunication),
            _ => Err(()),
        }
    }
}

pub enum PDOId {
    PDO1,
    PDO2,
    PDO3,
    PDO4,
}

impl PDOId {
    pub fn tx_id(&self) -> u16 {
        match self {
            PDOId::PDO1 => 0x180,
            PDOId::PDO2 => 0x280,
            PDOId::PDO3 => 0x380,
            PDOId::PDO4 => 0x480,
        }
    }

    pub fn rx_id(&self) -> u16 {
        match self {
            PDOId::PDO1 => 0x200,
            PDOId::PDO2 => 0x300,
            PDOId::PDO3 => 0x400,
            PDOId::PDO4 => 0x500,
        }
    }

    pub fn from_rx(id: u16) -> Result<Self, ()> {
        match id {
            0x200 => Ok(PDOId::PDO1),
            0x300 => Ok(PDOId::PDO2),
            0x400 => Ok(PDOId::PDO3),
            0x500 => Ok(PDOId::PDO4),
            _ => Err(()),
        }
    }
}
