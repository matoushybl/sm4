use bxcan::filter::Mask32;
use bxcan::{Can, Data, Frame, Interrupts};
use core::convert::TryFrom;
use embedded_can::{Id, StandardId};
use stm32f4xx_hal as hal;

pub struct CANOpen {
    bus: Can<hal::can::Can<hal::pac::CAN1>>,
    id: u8,
}

impl CANOpen {
    pub fn new(bus: hal::can::Can<hal::pac::CAN1>, id: u8) -> Self {
        let mut bus = Can::new(bus);
        bus.configure(|config| {
            config.set_bit_timing(0x001a000b);
        });
        bus.enable_interrupts(
            Interrupts::FIFO0_MESSAGE_PENDING | Interrupts::FIFO1_MESSAGE_PENDING,
        );
        // TODO properly configure filters.
        bus.modify_filters()
            .clear()
            .enable_bank(0, Mask32::accept_all());
        bus.set_automatic_wakeup(true);
        nb::block!(bus.enable()).unwrap();
        Self { bus, id }
    }

    pub fn process_incoming_frame(&mut self) -> Option<(CANOpenMessage, Frame)> {
        match nb::block!(self.bus.receive()) {
            Ok(frame) => {
                if let Some(message) = frame.parse_id() {
                    Some((message, frame))
                } else {
                    None
                }
            }
            Err(_) => {
                defmt::debug!("Failed to read.");
                None
            }
        }
    }

    pub fn send(&mut self, message: CANOpenMessage, data: &[u8]) -> Result<(), ()> {
        let frame = Frame::new_data(
            message.message_id_with_device(self.id),
            Data::new(data).unwrap(),
        );
        nb::block!(self.bus.transmit(&frame).map(|_| ())).map_err(|_| ())
    }
}

#[derive(Copy, Clone)]
pub enum CANOpenMessage {
    NMTNodeControl,
    GlobalFailsafeCommand,
    Sync,
    Emergency,
    TimeStamp,
    TxPDO1,
    RxPDO1,
    TxPDO2,
    RxPDO2,
    TxPDO3,
    RxPDO3,
    TxPDO4,
    RxPDO4,
    TxSDO,
    RxSDO,
    NMTNodeMonitoring,
}

impl TryFrom<u16> for CANOpenMessage {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value & 0xff80 {
            0x000 => Ok(Self::NMTNodeControl),
            0x001 => Ok(Self::GlobalFailsafeCommand),
            0x080 => Ok(Self::Sync),
            0x081 => Ok(Self::Emergency),
            0x100 => Ok(Self::TimeStamp),
            0x180 => Ok(Self::TxPDO1),
            0x200 => Ok(Self::RxPDO1),
            0x280 => Ok(Self::TxPDO2),
            0x300 => Ok(Self::RxPDO2),
            0x380 => Ok(Self::TxPDO3),
            0x400 => Ok(Self::RxPDO3),
            0x480 => Ok(Self::TxPDO4),
            0x500 => Ok(Self::RxPDO4),
            0x580 => Ok(Self::TxSDO),
            0x600 => Ok(Self::RxSDO),
            0x700 => Ok(Self::NMTNodeMonitoring),
            _ => Err(()),
        }
    }
}

impl From<&CANOpenMessage> for u16 {
    fn from(message: &CANOpenMessage) -> Self {
        match message {
            CANOpenMessage::NMTNodeControl => 0x000,
            CANOpenMessage::GlobalFailsafeCommand => 0x001,
            CANOpenMessage::Sync => 0x080,
            CANOpenMessage::Emergency => 0x081,
            CANOpenMessage::TimeStamp => 0x100,
            CANOpenMessage::TxPDO1 => 0x180,
            CANOpenMessage::RxPDO1 => 0x200,
            CANOpenMessage::TxPDO2 => 0x280,
            CANOpenMessage::RxPDO2 => 0x300,
            CANOpenMessage::TxPDO3 => 0x380,
            CANOpenMessage::RxPDO3 => 0x400,
            CANOpenMessage::TxPDO4 => 0x480,
            CANOpenMessage::RxPDO4 => 0x500,
            CANOpenMessage::TxSDO => 0x580,
            CANOpenMessage::RxSDO => 0x600,
            CANOpenMessage::NMTNodeMonitoring => 0x700,
        }
    }
}

impl CANOpenMessage {
    fn message_id_with_device(&self, device_id: u8) -> StandardId {
        match self {
            CANOpenMessage::NMTNodeControl
            | CANOpenMessage::GlobalFailsafeCommand
            | CANOpenMessage::Sync
            | CANOpenMessage::TimeStamp => StandardId::new(u16::from(self)).unwrap(),
            CANOpenMessage::Emergency
            | CANOpenMessage::TxPDO1
            | CANOpenMessage::RxPDO1
            | CANOpenMessage::TxPDO2
            | CANOpenMessage::RxPDO2
            | CANOpenMessage::TxPDO3
            | CANOpenMessage::RxPDO3
            | CANOpenMessage::TxPDO4
            | CANOpenMessage::RxPDO4
            | CANOpenMessage::TxSDO
            | CANOpenMessage::RxSDO
            | CANOpenMessage::NMTNodeMonitoring => {
                StandardId::new(u16::from(self) | device_id as u16).unwrap()
            }
        }
    }
}

pub trait CANOpenFrame {
    fn parse_id(&self) -> Option<CANOpenMessage>;
}

impl CANOpenFrame for Frame {
    fn parse_id(&self) -> Option<CANOpenMessage> {
        let frame_id = match self.id() {
            Id::Standard(std) => std.as_raw(),
            Id::Extended(_) => {
                return None;
            }
        };
        // let target_device = (frame_id & 0x7f) as u8;
        let message_id = frame_id & 0xff80;
        match CANOpenMessage::try_from(message_id) {
            Ok(message) => Some(message),
            Err(_) => None,
        }
    }
}

//
// pub enum RxCANMessage {
//     Sync(Data),
//     PDO(PDO, Data),
//     NMT(NMTRequestedState),
//     SDO,
// }
//
// pub fn message_from_frame(id: u8, frame: &Frame) -> Result<RxCANMessage, ()> {
//     let mut frame_id = 0u16;
//     match frame.id() {
//         Id::Standard(std) => {
//             frame_id = std.as_raw();
//         }
//         Id::Extended(_) => {
//             return Err(());
//         }
//     }
//     let frame_id = frame_id;
//
//     match message_id {
//         0x00 => {
//             if frame.dlc() < 2 {
//                 return Err(());
//             }
//             if frame.data().unwrap()[1] != id {
//                 return Err(());
//             }
//             match NMTRequestedState::try_from(frame.data().unwrap()[0]) {
//                 Ok(state) => Ok(RxCANMessage::NMT(state)),
//                 Err(_) => Err(()),
//             }
//         }
//         0x80 => Ok(RxCANMessage::Sync(*frame.data().unwrap())),
//         0x200 | 0x300 | 0x400 | 0x500 => {
//             if target_device != id {
//                 return Err(());
//             }
//             // unwrapping is here because if there is an Err, it is a programming error and should fail fast.
//             let pdo = PDO::from_rx(message_id).unwrap();
//             Ok(RxCANMessage::PDO(pdo, *frame.data().unwrap()))
//         }
//         _ => Err(()),
//     }
// }
//
// pub enum TxCANMessage {
//     NMTHeartbeat(NMTState),
//     PDO(PDO, [u8; 8], usize),
// }
//
// pub fn message_to_frame(id: u8, message: TxCANMessage) -> Frame {
//     match message {
//         TxCANMessage::NMTHeartbeat(state) => Frame::new_data(
//             StandardId::new(0x700 | (id as u16)).unwrap(),
//             [state.into()],
//         ),
//         TxCANMessage::PDO(pdo, data, size) => Frame::new_data(
//             StandardId::new(pdo.tx_id() | (id as u16)).unwrap(),
//             Data::new(&data[..size]).unwrap(),
//         ),
//     }
// }