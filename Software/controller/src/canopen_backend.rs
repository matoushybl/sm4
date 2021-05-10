use parking_lot::Mutex;
use sm4_shared::prelude::{
    AxisMode, Position, RxPDO1, RxPDO2, RxPDO3, RxPDO4, SerializePDO, TxPDO1, TxPDO2, TxPDO3,
    TxPDO4,
};
use socketcan::canopen::{
    CANOpen, CANOpenNodeCommand, CANOpenNodeMessage, NMTCommand, NMTState, PDO,
};
use socketcan::CANFrame;
use std::convert::TryFrom;
use std::sync::Arc;

pub const ENCODER_RESOLUTION: u32 = 16 * 200;

#[derive(Copy, Clone)]
pub struct AxisState {
    pub enabled: bool,
    pub mode: AxisMode,
    pub actual_velocity: f32,
    pub target_velocity: f32,
    pub actual_position: Position<ENCODER_RESOLUTION>,
    pub target_position: Position<ENCODER_RESOLUTION>,
}

impl Default for AxisState {
    fn default() -> Self {
        Self {
            enabled: false,
            mode: AxisMode::Velocity,
            actual_velocity: 0.0,
            target_velocity: 0.0,
            actual_position: Position::zero(),
            target_position: Position::zero(),
        }
    }
}

impl AxisState {
    pub fn mode(&self) -> &'static str {
        match self.mode {
            AxisMode::Velocity => "Velocity",
            AxisMode::Position => "Position",
        }
    }
}

#[derive(Copy, Clone)]
pub struct State {
    pub nmt_state: NMTState,
    pub voltage: f32,
    pub temperature: f32,
    pub axis1: AxisState,
    pub axis2: AxisState,
}

impl State {
    pub fn nmt_state(&self) -> &'static str {
        match self.nmt_state {
            NMTState::Initializing => "Initializing",
            NMTState::Stopped => "Stopped",
            NMTState::Operational => "Operational",
            NMTState::PreOperational => "Preoperational",
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            nmt_state: NMTState::Initializing,
            voltage: 0.0,
            temperature: 0.0,
            axis1: Default::default(),
            axis2: Default::default(),
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
                if let Ok(Some(frame)) = receiver.recv().map(Option::<CANOpenNodeMessage>::from) {
                    match frame {
                        CANOpenNodeMessage::SyncReceived => {
                            let state = state.lock();
                            sender
                                .send(CANFrame::from(CANOpenNodeCommand::SendPDO(
                                    id,
                                    PDO::PDO1,
                                    RxPDO1 {
                                        axis1_mode: state.axis1.mode,
                                        axis2_mode: state.axis2.mode,
                                        axis1_enabled: state.axis1.enabled,
                                        axis2_enabled: state.axis2.enabled,
                                    }
                                    .to_raw()
                                    .unwrap(),
                                    TxPDO1::len(),
                                )))
                                .unwrap();

                            sender
                                .send(CANFrame::from(CANOpenNodeCommand::SendPDO(
                                    id,
                                    PDO::PDO2,
                                    RxPDO2 {
                                        axis1_velocity: state.axis1.target_velocity,
                                        axis2_velocity: state.axis2.target_velocity,
                                    }
                                    .to_raw()
                                    .unwrap(),
                                    RxPDO2::len(),
                                )))
                                .unwrap();

                            sender
                                .send(CANFrame::from(CANOpenNodeCommand::SendPDO(
                                    id,
                                    PDO::PDO3,
                                    RxPDO3 {
                                        revolutions: state.axis1.target_position.get_revolutions(),
                                        angle: state.axis1.target_position.get_angle(),
                                    }
                                    .to_raw()
                                    .unwrap(),
                                    RxPDO3::len(),
                                )))
                                .unwrap();

                            sender
                                .send(CANFrame::from(CANOpenNodeCommand::SendPDO(
                                    id,
                                    PDO::PDO4,
                                    RxPDO4 {
                                        revolutions: state.axis2.target_position.get_revolutions(),
                                        angle: state.axis2.target_position.get_angle(),
                                    }
                                    .to_raw()
                                    .unwrap_or_default(),
                                    RxPDO4::len(),
                                )))
                                .unwrap();
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
                                    state.axis1.actual_velocity = pdo.axis1_velocity;
                                    state.axis2.actual_velocity = pdo.axis2_velocity;
                                }
                                Err(_) => {
                                    println!("received malformed TxPDO2");
                                }
                            },
                            PDO::PDO3 => match TxPDO3::try_from(&data[..(len as usize)]) {
                                Ok(pdo) => {
                                    state.lock().axis1.actual_position =
                                        Position::new(pdo.revolutions, pdo.angle);
                                }
                                Err(_) => {
                                    println!("received malformed TxPDO3");
                                }
                            },
                            PDO::PDO4 => match TxPDO4::try_from(&data[..(len as usize)]) {
                                Ok(pdo) => {
                                    state.lock().axis2.actual_position =
                                        Position::new(pdo.revolutions, pdo.angle);
                                }
                                Err(_) => {
                                    println!("received malformed TxPDO4");
                                }
                            },
                        },
                        CANOpenNodeMessage::NMTReceived(nmt_state) => {
                            state.lock().nmt_state = nmt_state;
                            if nmt_state != NMTState::Operational {
                                sender
                                    .send(CANFrame::from(CANOpenNodeCommand::SendNMT(
                                        id,
                                        NMTCommand::GoToOperational,
                                    )))
                                    .unwrap();
                            }
                        }
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
        self.state.lock().axis1.target_velocity = velocity;
    }

    pub fn set_axis2_target_velocity(&self, velocity: f32) {
        self.state.lock().axis2.target_velocity = velocity;
    }

    pub fn axis1_enabled(&self) -> bool {
        self.state.lock().axis1.enabled
    }

    pub fn axis2_enabled(&self) -> bool {
        self.state.lock().axis2.enabled
    }

    pub fn set_axis1_enabled(&self, enabled: bool) {
        self.state.lock().axis1.enabled = enabled;
    }

    pub fn set_axis2_enabled(&self, enabled: bool) {
        self.state.lock().axis2.enabled = enabled;
    }

    pub fn toggle_axis1_mode(&self) {
        let mut state = self.state.lock();
        state.axis1.mode = match state.axis1.mode {
            AxisMode::Velocity => AxisMode::Position,
            AxisMode::Position => AxisMode::Velocity,
        };
    }

    pub fn toggle_axis2_mode(&self) {
        let mut state = self.state.lock();
        state.axis2.mode = match state.axis2.mode {
            AxisMode::Velocity => AxisMode::Position,
            AxisMode::Position => AxisMode::Velocity,
        };
    }

    pub fn set_axis1_target_position(&self, position: Position<ENCODER_RESOLUTION>) {
        self.state.lock().axis1.target_position = position;
    }

    pub fn set_axis2_target_position(&self, position: Position<ENCODER_RESOLUTION>) {
        self.state.lock().axis2.target_position = position;
    }
}
