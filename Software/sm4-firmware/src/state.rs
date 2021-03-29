use crate::prelude::ObjectDictionary;
use sm4_shared::prelude::NMTState;

const SPEED_COMMAND_RESET_INTERVAL: u8 = 10; // ticks of a failsafe timer

#[derive(Copy, Clone)]
pub struct DriverState {
    nmt_state: NMTState,
    object_dictionary: ObjectDictionary,
    last_received_speed_command_down_counter: u8,
}

impl DriverState {
    pub fn new(encoder_resolution: u16) -> Self {
        Self {
            nmt_state: NMTState::default(),
            object_dictionary: ObjectDictionary::new(encoder_resolution),
            last_received_speed_command_down_counter: 0,
        }
    }

    pub fn go_to_preoperational_if_needed(&mut self) {
        if self.nmt_state == NMTState::BootUp {
            self.nmt_state = NMTState::PreOperational;
        }
    }

    pub fn go_to_operational(&mut self) {
        self.nmt_state = NMTState::Operational;
    }

    pub fn go_to_stopped(&mut self) {
        self.nmt_state = NMTState::Stopped;
    }

    pub fn go_to_preoperational(&mut self) {
        self.nmt_state = NMTState::PreOperational;
    }

    pub fn is_movement_blocked(&self) -> bool {
        self.nmt_state != NMTState::Operational
            || self.last_received_speed_command_down_counter == 0
    }

    pub fn decrement_last_received_speed_command_counter(&mut self) {
        if self.last_received_speed_command_down_counter != 0 {
            self.last_received_speed_command_down_counter -= 1;
        }
    }

    pub fn invalidate_last_received_speed_command_counter(&mut self) {
        self.last_received_speed_command_down_counter = SPEED_COMMAND_RESET_INTERVAL;
    }

    pub fn object_dictionary(&mut self) -> &mut ObjectDictionary {
        &mut self.object_dictionary
    }
}