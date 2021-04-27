use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use sm4_controller::canopen_backend::{CANOpenBackend, ENCODER_RESOLUTION};
use sm4_controller::draw;
use sm4_controller::tui::{SystemEvent, SystemEvents};
use sm4_shared::prelude::Position;
use std::io::Write;
use std::time::Duration;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let system_events = SystemEvents::new(Duration::from_millis(10));
    let backend = CANOpenBackend::new("can0", 0x01, 50000);

    let mut running = true;
    while running {
        terminal.draw(|frame| {
            draw(&backend.get_state(), frame);
        })?;
        const INCREMENT: f32 = 0.05;
        let position_increment = Position::<ENCODER_RESOLUTION>::new(0, 100);
        match system_events.recv() {
            SystemEvent::Input(key) => match key.code {
                KeyCode::Char('o') => {
                    let state = backend.get_state();
                    match state.axis1.mode {
                        sm4_shared::prelude::AxisMode::Velocity => {
                            backend
                                .set_axis1_target_velocity(state.axis1.target_velocity + INCREMENT);
                        }
                        sm4_shared::prelude::AxisMode::Position => backend
                            .set_axis1_target_position(
                                state.axis1.target_position + &position_increment,
                            ),
                    }
                }
                KeyCode::Char('p') => {
                    let state = backend.get_state();
                    match state.axis2.mode {
                        sm4_shared::prelude::AxisMode::Velocity => {
                            backend
                                .set_axis2_target_velocity(state.axis2.target_velocity + INCREMENT);
                        }
                        sm4_shared::prelude::AxisMode::Position => backend
                            .set_axis2_target_position(
                                state.axis2.target_position + &position_increment,
                            ),
                    }
                }
                KeyCode::Char('k') => {
                    let state = backend.get_state();
                    match state.axis1.mode {
                        sm4_shared::prelude::AxisMode::Velocity => {
                            backend
                                .set_axis1_target_velocity(state.axis1.target_velocity - INCREMENT);
                        }
                        sm4_shared::prelude::AxisMode::Position => {
                            backend.set_axis1_target_position(
                                state.axis1.target_position - &position_increment,
                            );
                        }
                    }
                }
                KeyCode::Char('l') => {
                    let state = backend.get_state();
                    match state.axis2.mode {
                        sm4_shared::prelude::AxisMode::Velocity => {
                            backend
                                .set_axis2_target_velocity(state.axis2.target_velocity - INCREMENT);
                        }
                        sm4_shared::prelude::AxisMode::Position => backend
                            .set_axis2_target_position(
                                state.axis2.target_position - &position_increment,
                            ),
                    }
                }
                KeyCode::Char('n') => backend.toggle_axis1_mode(),
                KeyCode::Char('m') => backend.toggle_axis2_mode(),
                KeyCode::Char('e') => {
                    backend.set_axis1_enabled(!backend.axis1_enabled());
                    backend.set_axis2_enabled(!backend.axis2_enabled());
                }
                KeyCode::Char('q') => running = false,
                _ => {}
            },
            SystemEvent::Tick => {}
        }
    }

    terminal.clear()?;
    disable_raw_mode()?;

    let mut stdout = std::io::stdout();
    execute!(stdout, LeaveAlternateScreen)?;

    Ok(())
}
