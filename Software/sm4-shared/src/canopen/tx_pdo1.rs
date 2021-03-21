#[derive(Copy, Clone, Default, Debug)]
pub struct TxPDO1 {
    pub battery_voltage: u16, // millivolts
    pub temperature: u16,     // 0.1 deg C
}

impl TxPDO1 {
    const SIZE: usize = 4;

    pub fn to_raw(&self, raw: &mut [u8]) -> Result<usize, ()> {
        if raw.len() < Self::SIZE {
            return Err(());
        }

        raw[..2].clone_from_slice(&self.battery_voltage.to_le_bytes());
        raw[2..4].clone_from_slice(&self.temperature.to_le_bytes());

        Ok(Self::SIZE)
    }
}
