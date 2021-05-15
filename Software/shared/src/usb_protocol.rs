use core::convert::TryInto;
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum USBMessage {
    Request(u16, u8),
    Transfer(u16, u8, u8, [u8; 4]),
}

pub trait USBProtocolConsumer {
    fn buffer(&self) -> &[u8];
    fn buffer_mut(&mut self) -> &mut [u8];
    fn buffer_index(&self) -> usize;
    fn buffer_index_mut(&mut self) -> &mut usize;
    fn max_length() -> usize;

    fn process_data(&mut self) -> Option<USBMessage> {
        if self.buffer()[0] != 0x55 {
            let mut start_byte_index = None;
            for (index, byte) in self.buffer().iter().enumerate() {
                if byte == &0x55 {
                    start_byte_index = Some(index);
                }
            }
            self.buffer_mut().fill(0);
            if let Some(start) = start_byte_index {
                self.buffer_mut().copy_within(start.., 0);
                *self.buffer_index_mut() -= start;
            } else {
                *self.buffer_index_mut() = 0;
            }
        }
        // there is more than 6 bytes of the message in the buffer
        if self.buffer_index() > 5 {
            let index = u16::from_le_bytes(self.buffer()[2..4].try_into().unwrap());
            let subindex = self.buffer()[4];
            let mut crc = crc_all::Crc::<u8>::new(0x31, 8, 0xff, 0x00, false);
            // request
            if self.buffer()[1] == 0x20 {
                crc.update(&self.buffer()[..5]);
                if self.buffer()[5] == crc.finish() {
                    self.buffer_mut().fill(0);
                    *self.buffer_index_mut() = 0;

                    return Some(USBMessage::Request(index, subindex));
                }
            } else if self.buffer()[1] == 0x21 {
                // transfer
                let length = self.buffer()[5] as usize;
                let total_length = 6 + length + 1;
                if self.buffer_index() >= total_length {
                    // there is enough data
                    crc.update(&self.buffer()[..(total_length - 1)]);
                    if self.buffer()[total_length - 1] == crc.finish() {
                        let mut output_buffer = [0u8; 4];
                        output_buffer[..length].copy_from_slice(&self.buffer()[6..][..length]);
                        self.buffer_mut().fill(0);
                        *self.buffer_index_mut() = 0;

                        return Some(USBMessage::Transfer(
                            index,
                            subindex,
                            length as u8,
                            output_buffer,
                        ));
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_within() {
        let mut buffer = [0, 0, 0, 1, 2, 3];
        buffer.copy_within(3.., 0);
        assert_eq!(buffer, [1, 2, 3, 1, 2, 3]);
    }

    const B_LEN: usize = 10;
    struct Consumer {
        buffer: [u8; B_LEN],
        index: usize,
    }

    impl USBProtocolConsumer for Consumer {
        fn buffer(&self) -> &[u8] {
            &self.buffer
        }

        fn buffer_mut(&mut self) -> &mut [u8] {
            &mut self.buffer
        }

        fn buffer_index(&self) -> usize {
            self.index
        }

        fn buffer_index_mut(&mut self) -> &mut usize {
            &mut self.index
        }

        fn max_length() -> usize {
            B_LEN
        }
    }

    #[test]
    fn not_enough_data() {
        let mut consumer = Consumer {
            buffer: [0x55, 0x20, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            index: 4,
        };

        assert!(consumer.process_data() == None)
    }

    #[test]
    fn request() {
        let mut crc = crc_all::Crc::<u8>::new(0x31, 8, 0xff, 0x00, false);
        let mut buffer = [0x55, 0x20, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        crc.update(&buffer[..5]);
        buffer[5] = crc.finish();
        let mut consumer = Consumer { buffer, index: 6 };

        assert!(consumer.process_data() == Some(USBMessage::Request(0x2000, 0x00)))
    }

    #[test]
    fn transfer() {
        let mut crc = crc_all::Crc::<u8>::new(0x31, 8, 0xff, 0x00, false);
        let mut buffer = [0x55, 0x21, 0x00, 0x20, 0x00, 0x02, 0x01, 0x02, 0x00, 0x00];
        crc.update(&buffer[..8]);
        buffer[8] = crc.finish();
        let mut consumer = Consumer { buffer, index: 9 };

        assert!(
            consumer.process_data()
                == Some(USBMessage::Transfer(0x2000, 0x00, 2, [0x01, 0x02, 0, 0]))
        )
    }
}
