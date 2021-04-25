use byteorder::{ByteOrder, LittleEndian};
use parking_lot::Mutex;
use sm4_shared::prelude::{
    AxisMode, PDODeserializationError, Position, RxPDO1, RxPDO2, RxPDO3, RxPDO4, TxPDO1, TxPDO2,
    TxPDO3, TxPDO4,
};
use socketcan::canopen::{CANOpen, CANOpenNodeCommand, CANOpenNodeMessage, NMTCommand, PDO};
use socketcan::CANFrame;
use std::convert::TryFrom;
use std::sync::Arc;

#[derive(Copy, Clone)]
pub struct State {
    pub enabled: bool,
    pub voltage: f32,
    pub temperature: f32,
    pub axis1_actual_velocity: f32,
    pub axis2_actual_velocity: f32,
    pub axis1_target_velocity: f32,
    pub axis2_target_velocity: f32,
    pub axis1_actual_position: Position,
    pub axis2_actual_position: Position,
}

impl Default for State {
    fn default() -> Self {
        Self {
            enabled: false,
            voltage: 0.0,
            temperature: 0.0,
            axis1_actual_velocity: 0.0,
            axis2_actual_velocity: 0.0,
            axis1_target_velocity: 0.0,
            axis2_target_velocity: 0.0,
            axis1_actual_position: Position::zero(16 * 200),
            axis2_actual_position: Position::zero(16 * 200),
        }
    }
}

pub struct CANOpenBackend {
    sender: crossbeam::channel::Sender<CANFrame>,
    state: Arc<Mutex<State>>,
}

impl CANOpenBackend {
    pub fn new(name: &str, id: u8, sync_period: u64) -> Self {
        let bus =
            CANOpen::new(name, Some(sync_period)).expect("Failed to access the selected CAN bus.");
        let device = bus.create_device(id);
        let receiver = device.get_receiver();
        let sender = device.get_sender();

        let state = Arc::new(Mutex::new(State::default()));

        std::thread::spawn({
            let sender = sender.clone();
            let state = state.clone();
            move || loop {
                if let Ok(Some(frame)) = receiver
                    .recv()
                    .map(|frame| Option::<CANOpenNodeMessage>::from(frame))
                {
                    match frame {
                        CANOpenNodeMessage::SyncReceived => {
                            // FIXME move where it belongs
                            sender.send(CANFrame::from(CANOpenNodeCommand::SendNMT(
                                1,
                                NMTCommand::GoToOperational,
                            )));
                            let state = state.lock();
                            let mut rx_pdo1 = RxPDO1::default();
                            rx_pdo1.axis1_enabled = state.enabled;
                            rx_pdo1.axis1_mode = AxisMode::Velocity;
                            rx_pdo1.axis2_enabled = state.enabled;
                            rx_pdo1.axis2_mode = AxisMode::Velocity;
                            let mut raw = [0u8; 8];
                            let size = rx_pdo1.to_raw(&mut raw).unwrap();
                            sender
                                .send(CANFrame::from(CANOpenNodeCommand::SendPDO(
                                    id,
                                    PDO::PDO1,
                                    raw,
                                    size,
                                )))
                                .unwrap();

                            let mut rx_pdo2 = RxPDO2::default();
                            rx_pdo2.axis1_velocity = state.axis1_target_velocity;
                            rx_pdo2.axis2_velocity = state.axis2_target_velocity;
                            let mut raw = [0u8; 8];
                            let size = rx_pdo2.to_raw(&mut raw).unwrap();
                            sender
                                .send(CANFrame::from(CANOpenNodeCommand::SendPDO(
                                    id,
                                    PDO::PDO2,
                                    raw,
                                    size,
                                )))
                                .unwrap()
                        }
                        CANOpenNodeMessage::PDOReceived(pdo, data, len) => match pdo {
                            PDO::PDO1 => match TxPDO1::try_from(&data[..(len as usize)]) {
                                Ok(pdo) => {
                                    let mut state = state.lock();
                                    state.voltage = pdo.battery_voltage as f32 / 1000.0;
                                    state.temperature = pdo.temperature as f32 / 10.0;
                                }
                                Err(_) => {
                                    println!("received malformed TxPDO1");
                                }
                            },
                            PDO::PDO2 => match TxPDO2::try_from(&data[..(len as usize)]) {
                                Ok(pdo) => {
                                    let mut state = state.lock();
                                    state.axis1_actual_velocity = pdo.axis1_velocity;
                                    state.axis2_actual_velocity = pdo.axis2_velocity;
                                }
                                Err(_) => {
                                    println!("received malformed TxPDO2");
                                }
                            },
                            PDO::PDO3 => match TxPDO3::try_from(&data[..(len as usize)]) {
                                Ok(pdo) => {
                                    state.lock().axis1_actual_position =
                                        Position::new(200 * 16, pdo.revolutions, pdo.angle as u16);
                                }
                                Err(_) => {
                                    println!("received malformed TxPDO3");
                                }
                            },
                            PDO::PDO4 => match TxPDO4::try_from(&data[..(len as usize)]) {
                                Ok(pdo) => {
                                    state.lock().axis2_actual_position =
                                        Position::new(200 * 16, pdo.revolutions, pdo.angle as u16);
                                }
                                Err(_) => {
                                    println!("received malformed TxPDO4");
                                }
                            },
                        },
                        CANOpenNodeMessage::NMTReceived(_) => {}
                        CANOpenNodeMessage::SDOReceived(_, _, _, _, _) => {}
                    }
                }
            }
        });
        Self { sender, state }
    }

    pub fn get_state(&self) -> State {
        *self.state.lock()
    }

    pub fn set_axis1_target_velocity(&self, velocity: f32) {
        self.state.lock().axis1_target_velocity = velocity;
    }

    pub fn set_axis2_target_velocity(&self, velocity: f32) {
        self.state.lock().axis2_target_velocity = velocity;
    }

    pub fn enabled(&self) -> bool {
        self.state.lock().enabled
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.state.lock().enabled = enabled;
    }
}
