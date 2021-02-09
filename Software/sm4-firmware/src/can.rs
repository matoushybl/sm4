use bxcan::filter::Mask32;
use bxcan::{Can, Data, Frame, Interrupts};
use core::convert::TryFrom;
use embedded_can::Id;
use stm32f4xx_hal as hal;

type BUS = Can<hal::can::Can<hal::pac::CAN1>>;

pub struct CAN {
    bus: BUS,
    device: CANOpenDevice,
}

impl CAN {
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
        Self {
            bus,
            device: CANOpenDevice::new(id),
        }
    }

    pub fn process_incoming_frame(&mut self) {
        match nb::block!(self.bus.receive()) {
            Ok(frame) => {
                defmt::error!("Received can message: {:?}", frame.dlc())
            }
            Err(_) => {
                defmt::debug!("Failed to read.");
            }
        }
    }
}

pub struct CANOpenDevice {
    id: u8,
    nmt_state: NMTState,
}

impl CANOpenDevice {
    pub fn new(id: u8) -> Self {
        Self {
            id,
            nmt_state: NMTState::BootUp,
        }
    }

    pub fn process_frame(&mut self, frame: &Frame) {
        let frame_id = match frame.id() {
            Id::Standard(std) => std.as_raw(),
            Id::Extended(_) => {
                return;
            }
        };
        let target_device = (frame_id & 0x7f) as u8;
        let message_id = frame_id & 0xff80;
        match message_id {
            0x000 => {
                // NMT requested state
                let data = frame.data().unwrap_or(&Data::empty());
                if frame.dlc() == 2 && data[1] == self.id {
                    if let Ok(new_state) = NMTRequestedState::try_from(data[0]) {
                        defmt::info!("Requested NMT state: {:?}", data[0]);
                    }
                }
            }
            _ => {
                defmt::error!("Unknown CANOpen frame id received.");
            }
        }
    }
}

pub enum NMTRequestedState {
    Operational,
    Stopped,
    PreOperational,
    ResetNode,
    ResetCommunication,
}

#[derive(Copy, Clone)]
pub enum NMTState {
    BootUp,
    Stopped,
    Operational,
    PreOperational,
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

pub enum PDO {
    PDO1,
    PDO2,
    PDO3,
    PDO4,
}

impl PDO {
    pub fn tx_id(&self) -> u16 {
        match self {
            PDO::PDO1 => 0x180,
            PDO::PDO2 => 0x280,
            PDO::PDO3 => 0x380,
            PDO::PDO4 => 0x480,
        }
    }

    pub fn rx_id(&self) -> u16 {
        match self {
            PDO::PDO1 => 0x200,
            PDO::PDO2 => 0x300,
            PDO::PDO3 => 0x400,
            PDO::PDO4 => 0x500,
        }
    }

    pub fn from_rx(id: u16) -> Result<Self, ()> {
        match id {
            0x200 => Ok(PDO::PDO1),
            0x300 => Ok(PDO::PDO2),
            0x400 => Ok(PDO::PDO3),
            0x500 => Ok(PDO::PDO4),
            _ => Err(()),
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
