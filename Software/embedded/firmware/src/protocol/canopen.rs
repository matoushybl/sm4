use crate::can::{CANOpen, CANOpenMessage};
use crate::prelude::LEDs;
use crate::state::DriverState;
use bxcan::Frame;
use core::convert::{TryFrom, TryInto};
use sm4_shared::prelude::*;

pub fn sync<OD, const R: u32>(bus: &mut CANOpen, state: &mut DriverState<OD, R>, leds: &mut LEDs)
where
    OD: ObjectDictionary<R>,
{
    let pdo = TxPDO1 {
        battery_voltage: (state.object_dictionary().battery_voltage() * 1000.0) as u16,
        temperature: (state.object_dictionary().temperature() * 10.0) as u16,
    };
    bus.send(
        CANOpenMessage::TxPDO1,
        &pdo.to_raw().unwrap()[..TxPDO1::len()],
    )
    .on_error(|_| leds.signalize_can_error());

    let pdo = TxPDO2 {
        axis1_velocity: state
            .object_dictionary()
            .axis(Axis::Axis1)
            .actual_velocity()
            .get_rps(),
        axis2_velocity: state
            .object_dictionary()
            .axis(Axis::Axis2)
            .actual_velocity()
            .get_rps(),
    };
    bus.send(
        CANOpenMessage::TxPDO2,
        &pdo.to_raw().unwrap()[..TxPDO2::len()],
    )
    .on_error(|_| leds.signalize_can_error());

    let pdo = TxPDO3 {
        revolutions: state
            .object_dictionary()
            .axis(Axis::Axis1)
            .actual_position()
            .get_revolutions(),
        angle: state
            .object_dictionary()
            .axis(Axis::Axis1)
            .actual_position()
            .get_angle() as u32,
    };
    bus.send(
        CANOpenMessage::TxPDO3,
        &pdo.to_raw().unwrap()[..TxPDO3::len()],
    )
    .on_error(|_| leds.signalize_can_error());

    let pdo = TxPDO4 {
        revolutions: state
            .object_dictionary()
            .axis(Axis::Axis2)
            .actual_position()
            .get_revolutions(),
        angle: state
            .object_dictionary()
            .axis(Axis::Axis2)
            .actual_position()
            .get_angle() as u32,
    };
    bus.send(
        CANOpenMessage::TxPDO4,
        &pdo.to_raw().unwrap()[..TxPDO4::len()],
    )
    .on_error(|_| {
        defmt::error!("Failed to send.");
        leds.signalize_can_error();
    });
}

pub fn nmt_received<OD, const R: u32>(id: u8, frame: &Frame, state: &mut DriverState<OD, R>)
where
    OD: ObjectDictionary<R>,
{
    if frame.dlc() != 2 {
        defmt::error!("Malformed NMT node control data received.");
        return;
    }
    if frame.data().unwrap()[1] != id {
        return;
    }
    match NMTRequestedState::try_from(frame.data().unwrap()[0]) {
        Ok(nmt_state) => match nmt_state {
            NMTRequestedState::Operational => {
                state.go_to_operational();
            }
            NMTRequestedState::Stopped => {
                state.go_to_stopped();
            }
            NMTRequestedState::PreOperational => {
                state.go_to_preoperational();
            }
            NMTRequestedState::ResetNode => {
                unimplemented!()
            }
            NMTRequestedState::ResetCommunication => {
                unimplemented!();
            }
        },
        Err(_) => {
            defmt::error!("Invalid NMT requested state received.");
        }
    }
}

pub fn rx_pdo1<OD, const R: u32>(frame: &Frame, state: &mut DriverState<OD, R>)
where
    OD: ObjectDictionary<R>,
{
    if frame.data().is_none() {
        defmt::warn!("Invalid RxPDO1 received.");
        return;
    }
    if let Ok(pdo) = RxPDO1::try_from(frame.data().unwrap().as_ref()) {
        state
            .object_dictionary()
            .axis_mut(Axis::Axis1)
            .set_mode(pdo.axis1_mode);
        state
            .object_dictionary()
            .axis_mut(Axis::Axis2)
            .set_mode(pdo.axis2_mode);
        state
            .object_dictionary()
            .axis_mut(Axis::Axis1)
            .set_enabled(pdo.axis1_enabled);
        state
            .object_dictionary()
            .axis_mut(Axis::Axis2)
            .set_enabled(pdo.axis2_enabled);
    } else {
        defmt::warn!("Malformed RxPDO1 received.");
    }
}

pub fn rx_pdo2<OD, const R: u32>(frame: &Frame, state: &mut DriverState<OD, R>)
where
    OD: ObjectDictionary<R>,
{
    if frame.data().is_none() {
        defmt::warn!("Invalid RxPDO2 received.");
        return;
    }
    if let Ok(pdo) = RxPDO2::try_from(frame.data().unwrap().as_ref()) {
        state
            .object_dictionary()
            .axis_mut(Axis::Axis1)
            .set_target_velocity(Velocity::new(pdo.axis1_velocity));
        state
            .object_dictionary()
            .axis_mut(Axis::Axis2)
            .set_target_velocity(Velocity::new(pdo.axis2_velocity));

        state.invalidate_last_received_speed_command_counter();
    } else {
        defmt::warn!("Malformed RxPDO2 received.");
    }
}

pub fn rx_pdo3<OD, const R: u32>(frame: &Frame, state: &mut DriverState<OD, R>)
where
    OD: ObjectDictionary<R>,
{
    if frame.data().is_none() {
        defmt::warn!("Invalid RxPDO1 received.");
        return;
    }
    if let Ok(pdo) = RxPDO3::try_from(frame.data().unwrap().as_ref()) {
        state
            .object_dictionary()
            .axis_mut(Axis::Axis1)
            .set_target_position(Position::new(pdo.revolutions, pdo.angle));
    } else {
        defmt::warn!("Malformed RxPDO3 received.");
    }
}

pub fn rx_pdo4<OD, const R: u32>(frame: &Frame, state: &mut DriverState<OD, R>)
where
    OD: ObjectDictionary<R>,
{
    if frame.data().is_none() {
        defmt::warn!("Invalid RxPDO4 received.");
        return;
    }
    if let Ok(pdo) = RxPDO4::try_from(frame.data().unwrap().as_ref()) {
        state
            .object_dictionary()
            .axis_mut(Axis::Axis2)
            .set_target_position(Position::new(pdo.revolutions, pdo.angle));
    } else {
        defmt::warn!("Malformed RxPDO4 received.");
    }
}

pub fn update_object_dictionary<const R: u32>(
    index: u16,
    subindex: u8,
    data: &[u8],
    object_dictionary: &mut dyn ObjectDictionary<R>,
) {
    if let Some(key) = Key::parse(index, subindex) {
        match key {
            Key::BatteryVoltage => {
                defmt::error!("Battery voltage shall not be changed by the higher level systems.");
            }
            Key::Temperature => {
                defmt::error!("Temperature shall not be changed by the higher level systems.");
            }
            Key::Axis1(key) => {
                update_axis_dictionary(key, data, object_dictionary.axis_mut(Axis::Axis1))
            }
            Key::Axis2(key) => {
                update_axis_dictionary(key, data, object_dictionary.axis_mut(Axis::Axis2))
            }
        }
    }
}

fn update_axis_dictionary<const R: u32>(
    key: AxisKey,
    data: &[u8],
    dictionary: &mut dyn AxisDictionary<R>,
) {
    fn parse_f32<F: FnOnce(f32)>(data: &[u8], f: F) {
        let raw: Result<[u8; 4], _> = data.try_into();
        if let Ok(raw) = raw {
            f(f32::from_le_bytes(raw))
        } else {
            defmt::error!("Failed to parse f32 from SDO data.");
        }
    }
    match key {
        AxisKey::Mode => dictionary.set_mode(AxisMode::from(data[0])),
        AxisKey::Enabled => dictionary.set_enabled(data[0] > 0),
        AxisKey::TargetVelocity => {
            parse_f32(data, |v| dictionary.set_target_velocity(Velocity::new(v)))
        }
        AxisKey::ActualVelocity => {
            defmt::error!("Writing to actual velocity is forbidden.")
        }
        AxisKey::TargetPositionRevolutions => {
            let raw: Result<[u8; 4], _> = data.try_into();
            if let Ok(raw) = raw {
                let position = dictionary.target_position();
                dictionary.set_target_position(Position::new(
                    i32::from_le_bytes(raw),
                    position.get_angle(),
                ));
            } else {
                defmt::error!("Failed to parse f32 from SDO data.");
            }
        }
        AxisKey::TargetPositionAngle => {
            let raw: Result<[u8; 4], _> = data.try_into();
            if let Ok(raw) = raw {
                let position = dictionary.target_position();
                dictionary.set_target_position(Position::new(
                    position.get_revolutions(),
                    u32::from_le_bytes(raw),
                ));
            } else {
                defmt::error!("Failed to parse f32 from SDO data.");
            }
        }
        AxisKey::ActualPositionRevolutions => {
            defmt::error!("Writing to actual velocity is forbidden.")
        }
        AxisKey::ActualPositionAngle => {
            defmt::error!("Writing to actual velocity is forbidden.")
        }
        AxisKey::Acceleration => parse_f32(data, |v| dictionary.set_acceleration(v)),
        AxisKey::VelocityFeedbackControlEnabled => {
            dictionary.set_velocity_feedback_control_enabled(data[0] > 0)
        }
        AxisKey::AcceleratingCurrent => parse_f32(data, |v| dictionary.set_accelerating_current(v)),
        AxisKey::StandStillCurrent => parse_f32(data, |v| dictionary.set_standstill_current(v)),
        AxisKey::ConstantVelocityCurrent => {
            parse_f32(data, |v| dictionary.set_constant_velocity_current(v))
        }
        AxisKey::VelocityP => parse_f32(data, |v| dictionary.set_velocity_controller_p(v)),
        AxisKey::VelocityS => parse_f32(data, |v| dictionary.set_velocity_controller_s(v)),
        AxisKey::VelocityD => parse_f32(data, |v| dictionary.set_velocity_controller_d(v)),
        AxisKey::VelocityMaxAction => {
            parse_f32(data, |v| dictionary.set_velocity_controller_max_output(v))
        }
        AxisKey::PositionP => parse_f32(data, |v| dictionary.set_position_controller_p(v)),
        AxisKey::PositionS => parse_f32(data, |v| dictionary.set_position_controller_s(v)),
        AxisKey::PositionD => parse_f32(data, |v| dictionary.set_position_controller_d(v)),
        AxisKey::PositionMaxAction => {
            parse_f32(data, |v| dictionary.set_position_controller_max_output(v))
        }
    }
}

pub fn read_object_dictionary<const R: u32>(
    index: u16,
    subindex: u8,
    dictionary: &dyn ObjectDictionary<R>,
) -> ([u8; 4], usize) {
    if let Some(key) = Key::parse(index, subindex) {
        match key {
            Key::BatteryVoltage => (dictionary.battery_voltage().to_le_bytes(), 4),
            Key::Temperature => (dictionary.temperature().to_le_bytes(), 4),
            Key::Axis1(key) => read_axis_dictionary(key, dictionary.axis(Axis::Axis1)),
            Key::Axis2(key) => read_axis_dictionary(key, dictionary.axis(Axis::Axis2)),
        }
    } else {
        ([0u8; 4], 4)
    }
}

fn read_axis_dictionary<const R: u32>(
    key: AxisKey,
    dictionary: &dyn AxisDictionary<R>,
) -> ([u8; 4], usize) {
    match key {
        AxisKey::Mode => ([dictionary.mode().into(), 0, 0, 0], 1),
        AxisKey::Enabled => ([dictionary.enabled() as u8, 0, 0, 0], 1),
        AxisKey::TargetVelocity => (dictionary.target_velocity().get_rps().to_le_bytes(), 4),
        AxisKey::ActualVelocity => (dictionary.actual_velocity().get_rps().to_le_bytes(), 4),
        AxisKey::TargetPositionRevolutions => (
            dictionary.target_position().get_revolutions().to_le_bytes(),
            4,
        ),
        AxisKey::TargetPositionAngle => (dictionary.target_position().get_angle().to_le_bytes(), 4),
        AxisKey::ActualPositionRevolutions => (
            dictionary.actual_position().get_revolutions().to_le_bytes(),
            4,
        ),
        AxisKey::ActualPositionAngle => (dictionary.actual_position().get_angle().to_le_bytes(), 4),
        AxisKey::Acceleration => (dictionary.acceleration().to_le_bytes(), 4),
        AxisKey::VelocityFeedbackControlEnabled => (
            [
                dictionary.velocity_feedback_control_enabled() as u8,
                0,
                0,
                0,
            ],
            1,
        ),
        AxisKey::AcceleratingCurrent => {
            (dictionary.current().accelerating_current().to_le_bytes(), 4)
        }
        AxisKey::StandStillCurrent => (dictionary.current().standstill_current().to_le_bytes(), 4),
        AxisKey::ConstantVelocityCurrent => (
            dictionary
                .current()
                .constant_velocity_current()
                .to_le_bytes(),
            4,
        ),
        AxisKey::VelocityP => (
            dictionary
                .velocity_controller_settings()
                .proportional()
                .to_le_bytes(),
            4,
        ),
        AxisKey::VelocityS => (
            dictionary
                .velocity_controller_settings()
                .integral()
                .to_le_bytes(),
            4,
        ),
        AxisKey::VelocityD => (
            dictionary
                .velocity_controller_settings()
                .derivative()
                .to_le_bytes(),
            4,
        ),
        AxisKey::VelocityMaxAction => (
            dictionary
                .velocity_controller_settings()
                .max_output_amplitude()
                .to_le_bytes(),
            4,
        ),
        AxisKey::PositionP => (
            dictionary
                .position_controller_settings()
                .proportional()
                .to_le_bytes(),
            4,
        ),
        AxisKey::PositionS => (
            dictionary
                .position_controller_settings()
                .integral()
                .to_le_bytes(),
            4,
        ),
        AxisKey::PositionD => (
            dictionary
                .position_controller_settings()
                .derivative()
                .to_le_bytes(),
            4,
        ),
        AxisKey::PositionMaxAction => (
            dictionary
                .position_controller_settings()
                .max_output_amplitude()
                .to_le_bytes(),
            4,
        ),
    }
}
