use crate::prelude::ObjectDictionary;
use sm4_shared::prelude::{NMTState, ObjectDictionaryStorage};

const SPEED_COMMAND_RESET_INTERVAL: u8 = 10; // ticks of a failsafe timer

#[derive(Copy, Clone)]
pub struct DriverState<STORAGE: ObjectDictionaryStorage, const RESOLUTION: u32> {
    nmt_state: NMTState,
    object_dictionary: ObjectDictionary<STORAGE, RESOLUTION>,
    last_received_speed_command_down_counter: u8,
}

impl<STORAGE: ObjectDictionaryStorage, const RESOLUTION: u32> DriverState<STORAGE, RESOLUTION> {
    pub fn new(storage: STORAGE) -> Self {
        Self {
            nmt_state: NMTState::default(),
            object_dictionary: ObjectDictionary::new(storage),
            last_received_speed_command_down_counter: 0,
        }
    }

    pub fn nmt_state(&self) -> NMTState {
        self.nmt_state
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

    pub fn object_dictionary(&mut self) -> &mut ObjectDictionary<STORAGE, RESOLUTION> {
        &mut self.object_dictionary
    }
}
