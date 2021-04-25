use byteorder::{ByteOrder, LittleEndian};
use crossterm::event::{
    read, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent, KeyModifiers,
};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{cursor, event};
use device_query::{DeviceQuery, DeviceState, Keycode};
use sm4_controller::canopen_backend::CANOpenBackend;
use sm4_controller::tui::{SystemEvent, SystemEvents};
use socketcan::canopen::CANOpenNodeCommand::SendPDO;
use socketcan::canopen::{CANOpen, CANOpenNodeMessage, PDO};
use socketcan::{CANFrame, CANSocket};
use std::io::{stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::Terminal;

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
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
                .constraints([Constraint::Percentage(95), Constraint::Percentage(5)].as_ref())
                .split(frame.size());
            let block = Block::default().borders(Borders::ALL).title(Span::styled(
                "SM4 velocity controller",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ));

            let received = vec![
                Spans::from(format!("enabled: {}", backend.enabled())),
                Spans::from(format!("temp: {}", backend.get_state().temperature)),
                Spans::from(format!("voltage: {}", backend.get_state().voltage)),
                Spans::from(format!(
                    "A1: target vel: {}",
                    backend.get_state().axis1_target_velocity
                )),
                Spans::from(format!(
                    "A2: target vel: {}",
                    backend.get_state().axis2_target_velocity
                )),
                Spans::from(format!(
                    "A1: actual vel: {}",
                    backend.get_state().axis1_actual_velocity
                )),
                Spans::from(format!(
                    "A2: actual vel: {}",
                    backend.get_state().axis2_actual_velocity
                )),
                Spans::from(format!(
                    "A1: actual pos: {} - {}",
                    backend.get_state().axis1_actual_position.get_revolutions(),
                    backend.get_state().axis1_actual_position.get_angle(),
                )),
                Spans::from(format!(
                    "A2: actual pos: {} - {}",
                    backend.get_state().axis2_actual_position.get_revolutions(),
                    backend.get_state().axis2_actual_position.get_angle(),
                )),
            ];

            let paragraph = Paragraph::new(received)
                .block(block)
                .wrap(Wrap { trim: true });
            frame.render_widget(paragraph, chunks[0]);
            frame.render_widget(
                Paragraph::new(
                    "q - quit   e - enable   o - A1 vel up   k - A1 vel down   p - A2 vel up   l - A2 vel down",
                ),
                chunks[1],
            );
        })?;
        match system_events.recv() {
            SystemEvent::Input(key) => match key.code {
                KeyCode::Char('o') => {
                    let state = backend.get_state();
                    backend.set_axis1_target_velocity(state.axis1_target_velocity + 0.1);
                }
                KeyCode::Char('p') => {
                    let state = backend.get_state();
                    backend.set_axis2_target_velocity(state.axis2_target_velocity + 0.1);
                }
                KeyCode::Char('k') => {
                    let state = backend.get_state();
                    backend.set_axis1_target_velocity(state.axis1_target_velocity - 0.1);
                }
                KeyCode::Char('l') => {
                    let state = backend.get_state();
                    backend.set_axis2_target_velocity(state.axis2_target_velocity - 0.1);
                }
                KeyCode::Char('e') => backend.set_enabled(!backend.enabled()),
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
