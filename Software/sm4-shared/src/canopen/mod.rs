//! This module contains the definition of objects that are used for CANOpen communication.
//! There are the PDO definitions and the object dictionary.
//! Some of the PDOs are abstracted into the PositionPDO or VelocityPDO to keep the code DRY.

mod object_dictionary;
mod position_pdo;
mod rx_pdo1;
mod tx_pdo1;
mod velocity_pdo;

pub use pdos::{RxPDO1, RxPDO2, RxPDO3, RxPDO4, TxPDO1, TxPDO2, TxPDO3, TxPDO4};

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
