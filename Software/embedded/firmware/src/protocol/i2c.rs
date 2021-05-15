use core::convert::{TryFrom, TryInto};
use sm4_shared::prelude::{Axis, AxisMode, ObjectDictionary, Position, Velocity};

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum I2CRegister {
    AxisSettings,
    Axis1Velocity,
    Axis2Velocity,
    BothAxesVelocity,
    Axis1Position,
    Axis2Position,
    BothAxesPosition,
}

impl I2CRegister {
    pub fn readable(&self) -> bool {
        self != &Self::BothAxesVelocity
    }

    pub fn writeable(&self) -> bool {
        self != &Self::BothAxesPosition
    }
}

impl TryFrom<u8> for I2CRegister {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x10 => Ok(Self::AxisSettings),
            0x21 => Ok(Self::Axis1Velocity),
            0x22 => Ok(Self::Axis2Velocity),
            0x31 => Ok(Self::Axis1Position),
            0x32 => Ok(Self::Axis2Position),
            0x40 => Ok(Self::BothAxesVelocity),
            0x50 => Ok(Self::BothAxesPosition),
            _ => Err(()),
        }
    }
}

pub fn axis_settings<const R: u32>(dictionary: &dyn ObjectDictionary<R>) -> [u8; 2] {
    let mut buffer: [u8; 2] = [0; 2];

    buffer[0] = u8::from(dictionary.axis(Axis::Axis1).mode()) << 4
        | u8::from(dictionary.axis(Axis::Axis2).mode());
    buffer[1] = u8::from(dictionary.axis(Axis::Axis1).enabled()) << 4
        | u8::from(dictionary.axis(Axis::Axis2).enabled());
    buffer
}

pub fn set_axis_settings<const R: u32>(raw: &[u8], dictionary: &mut dyn ObjectDictionary<R>) {
    if raw.len() < 2 {
        defmt::error!("Too few raw data to parse axis settings.");
        return;
    }

    dictionary
        .axis_mut(Axis::Axis1)
        .set_mode(AxisMode::from(raw[0] & 0x0f));
    dictionary
        .axis_mut(Axis::Axis2)
        .set_mode(AxisMode::from((raw[0] & 0xf0) >> 4));

    dictionary
        .axis_mut(Axis::Axis1)
        .set_enabled((raw[1] & 0x0f) > 0);
    dictionary
        .axis_mut(Axis::Axis2)
        .set_enabled(((raw[1] & 0xf0) >> 4) > 0);
}

pub fn parse_velocity(raw: &[u8]) -> Velocity {
    if raw.len() < 4 {
        defmt::error!("Too few raw data to parse axis velocity.");
        return Velocity::zero();
    }

    Velocity::new(f32::from_le_bytes(raw[..4].try_into().unwrap()))
}

pub fn parse_both_axes_velocities(raw: &[u8]) -> (Velocity, Velocity) {
    if raw.len() < 8 {
        defmt::error!("Too few raw data to parse both axes velocity.");
        return (Velocity::zero(), Velocity::zero());
    }
    (parse_velocity(&raw[..4]), parse_velocity(&raw[4..]))
}

pub fn both_axes_velocity<const R: u32>(dictionary: &dyn ObjectDictionary<R>) -> [u8; 8] {
    let mut buffer: [u8; 8] = [0; 8];
    buffer[..4].copy_from_slice(
        &dictionary
            .axis(Axis::Axis1)
            .actual_velocity()
            .get_rps()
            .to_le_bytes(),
    );
    buffer[4..].copy_from_slice(
        &dictionary
            .axis(Axis::Axis2)
            .actual_velocity()
            .get_rps()
            .to_le_bytes(),
    );
    buffer
}

pub fn position<const R: u32>(position: &Position<R>) -> [u8; 8] {
    let mut buffer: [u8; 8] = [0; 8];
    buffer[..4].copy_from_slice(&position.get_revolutions().to_le_bytes());
    buffer[4..].copy_from_slice(&position.get_angle().to_le_bytes());
    buffer
}

pub fn parse_position<const R: u32>(raw: &[u8]) -> Position<R> {
    if raw.len() < 8 {
        defmt::error!("Too few raw data to parse axis position.");
        return Position::zero();
    }

    Position::new(
        i32::from_le_bytes(raw[..4].try_into().unwrap()),
        u32::from_le_bytes(raw[4..].try_into().unwrap()),
    )
}

pub fn both_axes_position<const R: u32>(dictionary: &dyn ObjectDictionary<R>) -> [u8; 16] {
    let mut buffer: [u8; 16] = [0; 16];
    buffer[..4].copy_from_slice(&position(&dictionary.axis(Axis::Axis1).actual_position()));
    buffer[4..].copy_from_slice(&position(&dictionary.axis(Axis::Axis2).actual_position()));
    buffer
}
