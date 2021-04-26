use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use sm4_controller::canopen_backend::CANOpenBackend;
use sm4_controller::tui::{SystemEvent, SystemEvents};
use sm4_shared::prelude::Position;
use std::io::Write;
use std::time::Duration;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};
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
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(35), Constraint::Percentage(60), Constraint::Percentage(5)].as_ref())
                .split(frame.size());
            let block = Block::default().borders(Borders::ALL).title(Span::styled(
                "SM4 velocity controller - common",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ));

            let received = vec![
                Spans::from(format!("NMT: {}", backend.get_state().nmt_state())),
                Spans::from(format!("temp: {}", backend.get_state().temperature)),
                Spans::from(format!("voltage: {}", backend.get_state().voltage)),
            ];

            let paragraph = Paragraph::new(received)
                .block(block)
                .wrap(Wrap { trim: true });
            frame.render_widget(paragraph, chunks[0]);


            let axis_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[1]);

            let axis1_data = vec![
                Spans::from(format!(
                    "enabled: {}",
                    backend.axis1_enabled()
                )),
                Spans::from(format!(
                    "mode: {}",
                    backend.get_state().axis1.mode()
                )),
                Spans::from(format!(
                    "target vel: {}",
                    backend.get_state().axis1.target_velocity
                )),
                Spans::from(format!(
                    "actual vel: {}",
                    backend.get_state().axis1.actual_velocity
                )),
                Spans::from(format!(
                    "target pos: {} - {}",
                    backend.get_state().axis1.target_position.get_revolutions(),
                    backend.get_state().axis1.target_position.get_angle(),
                )),
                Spans::from(format!(
                    "actual pos: {} - {}",
                    backend.get_state().axis1.actual_position.get_revolutions(),
                    backend.get_state().axis1.actual_position.get_angle(),
                )),
            ];
            let block = Block::default().borders(Borders::ALL).title(Span::styled(
                "Axis 1",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ));
            let paragraph = Paragraph::new(axis1_data)
                .block(block)
                .wrap(Wrap { trim: true });
            frame.render_widget(paragraph, axis_chunks[0]);


            let axis2_data = vec![
                Spans::from(format!(
                    "enabled: {}",
                    backend.axis2_enabled()
                )),
                Spans::from(format!(
                    "mode: {}",
                    backend.get_state().axis2.mode()
                )),
                Spans::from(format!(
                    "target vel: {}",
                    backend.get_state().axis2.target_velocity
                )),
                Spans::from(format!(
                    "actual vel: {}",
                    backend.get_state().axis2.actual_velocity
                )),
                Spans::from(format!(
                    "target pos: {} - {}",
                    backend.get_state().axis2.target_position.get_revolutions(),
                    backend.get_state().axis2.target_position.get_angle(),
                )),
                Spans::from(format!(
                    "actual pos: {} - {}",
                    backend.get_state().axis2.actual_position.get_revolutions(),
                    backend.get_state().axis2.actual_position.get_angle(),
                )),
            ];
            let block = Block::default().borders(Borders::ALL).title(Span::styled(
                "Axis 2",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ));
            let paragraph = Paragraph::new(axis2_data)
                .block(block)
                .wrap(Wrap { trim: true });
            frame.render_widget(paragraph, axis_chunks[1]);

            frame.render_widget(
                Paragraph::new(
                    "q - quit   e - enable   o - A1 up   k - A1 down   n - toggle A1 mode   p - A2 vel up   l - A2 vel down   m - toggle A2 mode",
                ),
                chunks[2],
            );
        })?;
        const INCREMENT: f32 = 0.05;
        let position_increment: Position = Position::new(16 * 200, 0, 100);
        match system_events.recv() {
            SystemEvent::Input(key) => match key.code {
                KeyCode::Char('o') => {
                    let state = backend.get_state();
                    match state.axis1.mode {
                        sm4_shared::prelude::AxisMode::Velocity => {
                            backend
                                .set_axis1_target_velocity(state.axis1.target_velocity + INCREMENT);
                        }
                        sm4_shared::prelude::AxisMode::Position => {
                            let mut position = state.axis1.target_position;
                            position += position_increment.get_increments();
                            backend.set_axis1_target_position(position)
                        }
                    }
                }
                KeyCode::Char('p') => {
                    let state = backend.get_state();
                    match state.axis2.mode {
                        sm4_shared::prelude::AxisMode::Velocity => {
                            backend
                                .set_axis2_target_velocity(state.axis2.target_velocity + INCREMENT);
                        }
                        sm4_shared::prelude::AxisMode::Position => {
                            let mut position = state.axis2.target_position;
                            position += position_increment.get_increments();
                            backend.set_axis2_target_position(position)
                        }
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
                            let mut position = state.axis1.target_position;
                            position -= position_increment.get_increments();
                            backend.set_axis1_target_position(position)
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
                        sm4_shared::prelude::AxisMode::Position => {
                            let mut position = state.axis2.target_position;
                            position -= position_increment.get_increments();
                            backend.set_axis2_target_position(position)
                        }
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

    // terminal.clear()?;
    disable_raw_mode()?;

    let mut stdout = std::io::stdout();
    execute!(stdout, LeaveAlternateScreen)?;

    Ok(())
}
