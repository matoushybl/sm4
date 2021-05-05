use stm32f4xx_hal::i2c::{Instance, PinScl, PinSda};
use stm32f4xx_hal::stm32::RCC;

#[derive(Copy, Clone, PartialEq)]
pub enum TransferState {
    Idle,
    Addressed,
    RegisterSet,
    Receiving,
    Transmitting,
}

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    DataRequested(u8),
    DataReceived(u8),
}

// direction as specified in the datasheet
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Write, // slave is receiver
    Read,  // slave is transmitter
}

impl From<Direction> for bool {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Write => false,
            Direction::Read => true,
        }
    }
}

impl From<bool> for Direction {
    fn from(raw: bool) -> Self {
        if raw {
            Direction::Read
        } else {
            Direction::Write
        }
    }
}

const BUFFER_SIZE: usize = 32;
//
pub struct I2CSlave<I2C: Instance, SDA, SCL> {
    i2c: I2C,
    transfer_buffer: [u8; BUFFER_SIZE],
    transfer_len: usize,
    buffer_index: usize,
    register: u8,
    transfer_state: TransferState,
    state: Option<State>,
    _sda: SDA,
    _scl: SCL,
}
//
impl<I2C: Instance, SDA, SCL> I2CSlave<I2C, SDA, SCL>
where
    SDA: PinSda<I2C>,
    SCL: PinScl<I2C>,
{
    pub fn new(i2c: I2C, address: u8, sda: SDA, scl: SCL) -> Self {
        // RCC init taken from https://github.com/stm32-rs/stm32f4xx-hal/blob/bb160bbe8604bccf28557975131d0afdb2fa1014/src/i2c.rs#L723
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());

            // Enable and reset clock.
            I2C::enable_clock(&rcc);
        }

        i2c.cr1.modify(|_, w| w.pe().disabled());

        i2c.oar1
            .write(|w| w.addmode().add7().add().bits((address as u16) << 1));

        i2c.cr1.modify(
            |_, w| w.pe().enabled(), // enable peripheral
        );

        i2c.cr1.modify(|_, w| {
            w.nostretch()
                .enabled() // enable clock stretching
                .ack()
                .ack()
        });

        unsafe {
            i2c.cr2.modify(|_, w| {
                w.itbufen()
                    .enabled()
                    .itevten()
                    .enabled()
                    .iterren()
                    .enabled()
                    .freq()
                    .bits(42)
            })
        }

        I2CSlave {
            i2c,
            transfer_buffer: [0u8; BUFFER_SIZE],
            transfer_len: 0,
            buffer_index: 0,
            register: 0,
            transfer_state: TransferState::Idle,
            state: None,
            _sda: sda,
            _scl: scl,
        }
    }

    fn addressed(&self) -> Option<Direction> {
        if self.i2c.sr1.read().addr().bit_is_clear() {
            None
        } else {
            Some(self.i2c.sr2.read().tra().bit().into())
        }
    }

    fn rx_not_empty(&self) -> bool {
        self.i2c.sr1.read().rx_ne().bit_is_set()
    }

    fn tx_empty(&self) -> bool {
        self.i2c.sr1.read().tx_e().bit_is_set()
    }

    fn stopped(&self) -> bool {
        self.i2c.sr1.read().stopf().bit_is_set()
    }

    fn clear_stopped(&mut self) {
        self.i2c.cr1.modify(|_, w| w.pe().enabled());
    }

    pub fn read(&self) -> u8 {
        self.i2c.dr.read().dr().bits() as u8
    }

    pub fn write(&self, value: u8) {
        self.i2c.dr.write(|w| w.dr().bits(value));
    }

    pub fn event_interrupt(&mut self) {
        defmt::trace!("I2C event interrupt.");
        if self.transfer_state == TransferState::Idle {
            self.state = None;
        }

        if self.rx_not_empty() {
            if self.transfer_state == TransferState::Addressed {
                self.transfer_state = TransferState::RegisterSet;
                self.register = self.read();
                defmt::trace!("Register received: {:x}", self.register);
            } else if self.transfer_state == TransferState::RegisterSet {
                self.transfer_state = TransferState::Receiving;
            } else if self.transfer_state == TransferState::Receiving {
                // do not change state, just read
                self.transfer_buffer[self.buffer_index] = self.read();
                defmt::trace!("Byte read: {:x}", self.transfer_buffer[self.buffer_index]);
                self.buffer_index += 1;
            }
        }
        if self.stopped() {
            self.clear_stopped();
            // handle reception
            if self.transfer_state == TransferState::Receiving {
                self.state = Some(State::DataReceived(self.register));
            } else if self.transfer_state == TransferState::Transmitting {
                // data was transmitted, nothing else to do
                self.state = None;
            }
            self.transfer_state = TransferState::Idle;
        }

        let addressed = self.addressed();
        if let Some(Direction::Write) = addressed {
            defmt::error!("Slave is now Receiver.");
            self.transfer_state = TransferState::Addressed;
            return;
        }

        if let Some(Direction::Read) = addressed {
            defmt::error!("Slave is now Transmitter.");
            if self.transfer_state == TransferState::RegisterSet {
                self.transfer_state = TransferState::Transmitting;
                self.state = Some(State::DataRequested(self.register));
            }
            return;
        }

        if self.tx_empty() {
            defmt::error!("txe");
            // this may be true more times than actual data length, ignore then
            if self.transfer_state == TransferState::Transmitting {
                // state is not changed
                if self.buffer_index < self.transfer_len {
                    self.write(self.transfer_buffer[self.buffer_index]);
                    defmt::debug!("Transmitting: {}", self.buffer_index as u8);
                    self.buffer_index += 1;
                } else {
                    self.state = None;
                }
            }
        }
    }

    pub fn error_interrupt(&mut self) {
        defmt::trace!("I2C error interrupt.");
        self.check_and_clear_error_flags();
        self.handle_error();
    }

    fn handle_error(&mut self) {
        self.transfer_state = TransferState::Idle;
        self.state = None;
        self.transfer_len = 0;
        self.buffer_index = 0;
    }

    // https://github.com/stm32-rs/stm32f4xx-hal/blob/bb160bbe8604bccf28557975131d0afdb2fa1014/src/i2c.rs#L804
    fn check_and_clear_error_flags(&self) {
        // Note that flags should only be cleared once they have been registered. If flags are
        // cleared otherwise, there may be an inherent race condition and flags may be missed.
        let sr1 = self.i2c.sr1.read();

        if sr1.timeout().bit_is_set() {
            defmt::trace!("I2C clearing Timeout.");
            self.i2c.sr1.modify(|_, w| w.timeout().clear_bit());
        }

        if sr1.pecerr().bit_is_set() {
            defmt::trace!("I2C clearing PEC.");
            self.i2c.sr1.modify(|_, w| w.pecerr().clear_bit());
        }

        if sr1.ovr().bit_is_set() {
            defmt::trace!("I2C clearing Overrun.");
            self.i2c.sr1.modify(|_, w| w.ovr().clear_bit());
        }

        if sr1.af().bit_is_set() {
            defmt::trace!("I2C clearing AF.");
            self.i2c.sr1.modify(|_, w| w.af().clear_bit());
        }

        if sr1.arlo().bit_is_set() {
            defmt::trace!("I2C clearing ARLO.");
            self.i2c.sr1.modify(|_, w| w.arlo().clear_bit());
        }

        // The errata indicates that BERR may be incorrectly detected. It recommends ignoring and
        // clearing the BERR bit instead.
        if sr1.berr().bit_is_set() {
            defmt::trace!("I2C clearing BERR.");
            self.i2c.sr1.modify(|_, w| w.berr().clear_bit());
        }
    }

    pub fn set_transmit_buffer(&mut self, buffer: &[u8]) {
        for (index, item) in buffer.iter().enumerate() {
            self.transfer_buffer[index] = *item;
        }
        self.transfer_len = buffer.len();
        self.buffer_index = 0;
        self.state = None
    }

    pub fn get_received_data(&mut self) -> &[u8] {
        let data = &self.transfer_buffer[..self.buffer_index];
        self.state = None;
        self.buffer_index = 0;
        self.transfer_len = 0;
        data
    }

    pub fn get_state(&self) -> Option<State> {
        self.state
    }
}
