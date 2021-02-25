use core::convert::TryFrom;

#[repr(C, align(1))]
#[derive(Copy, Clone, Default, Debug)]
pub struct RxPDO1 {
    pub driver1_speed: f32,
    pub driver2_speed: f32,
}

impl TryFrom<&[u8]> for RxPDO1 {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != core::mem::size_of::<Self>() {
            return Err(());
        }

        Ok(unsafe { core::mem::transmute_copy(&value) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let valid_raw_data = [0u8; 8];
        assert!(RxPDO1::try_from(valid_raw_data.as_ref()).is_ok());

        let invalid_raw_data = [0u8; 4];
        assert!(RxPDO1::try_from(invalid_raw_data.as_ref()).is_err());
    }
}
