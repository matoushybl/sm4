use crate::can::{CANOpen, CANOpenMessage};
use crate::prelude::LEDs;
use crate::state::DriverState;
use bxcan::Frame;
use core::convert::TryFrom;
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
            .axis(Axis::Axis1)
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
    .on_error(|_| leds.signalize_can_error());
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
