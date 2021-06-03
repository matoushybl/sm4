use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use device_query::keymap::Keycode::Comma;
use glib::bitflags::_core::sync::atomic::AtomicBool;
use sm4_controller::canopen_backend::{CANOpenBackend, ENCODER_RESOLUTION};
use sm4_controller::draw;
use sm4_controller::tui::{SystemEvent, SystemEvents};
use sm4_shared::prelude::Position;
use std::io::Write;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use tui::backend::CrosstermBackend;
use tui::Terminal;

enum Command {
    Forward,
    Rotate,
    Rotate2,
    Backward,
}

fn main() -> anyhow::Result<()> {
    let backend = CANOpenBackend::new("can0", 0x01, 50000);
    backend.set_axis1_enabled(true);
    backend.set_axis2_enabled(true);

    let mut running = Arc::new(AtomicBool::new(true));

    ctrlc::set_handler({
        let running = running.clone();
        move || running.store(false, Ordering::SeqCst)
    });

    let speed = 0.5;

    let sequence = [
        Command::Forward,
        Command::Backward,
        Command::Rotate,
        Command::Forward,
        Command::Backward,
        Command::Rotate,
        Command::Forward,
        Command::Backward,
        Command::Rotate,
        Command::Forward,
        Command::Backward,
        Command::Rotate,
        Command::Forward,
        Command::Backward,
        Command::Rotate2,
        Command::Forward,
        Command::Backward,
        Command::Rotate2,
        Command::Forward,
        Command::Backward,
        Command::Rotate2,
        Command::Forward,
        Command::Backward,
        Command::Rotate2,
    ];

    while running.load(Ordering::SeqCst) {
        for seq in sequence.iter() {
            let (s1, s2) = match *seq {
                Command::Forward => (speed, speed),
                Command::Rotate => (0.5 * speed, -0.5 * speed),
                Command::Backward => (-speed, -speed),
                Command::Rotate2 => (-0.5 * speed, 0.5 * speed),
            };

            backend.set_axis1_target_velocity(-s1);
            backend.set_axis2_target_velocity(s2);

            std::thread::sleep(Duration::from_millis(1000));
        }
    }

    Ok(())
}
