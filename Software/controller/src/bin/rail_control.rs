use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use parking_lot::Mutex;
use sm4_controller::canopen_backend::AxisState;
use sm4_controller::canopen_backend::ENCODER_RESOLUTION;
use sm4_controller::tui::SystemEvent;
use sm4_controller::tui::SystemEvents;
use sm4_shared::prelude::Position;
use std::io::Write;
use std::sync::Arc;
use std::time::Duration;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame, Terminal,
};

const MAX_POSITION: Position<ENCODER_RESOLUTION> = Position::<ENCODER_RESOLUTION>::new(10, 0);

fn draw<B: Backend>(state: &AxisState, frame: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(35),
                Constraint::Percentage(5),
            ]
            .as_ref(),
        )
        .split(frame.size());
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(Span::styled(
                    "Position",
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(
            (state.actual_position.get_increments() as f32 / MAX_POSITION.get_increments() as f32
                * 100.0) as u16,
        );
    frame.render_widget(gauge, chunks[1]);
    draw_axis_block(state, frame, chunks[3]);
    frame.render_widget(
        Paragraph::new("q - quit, left arrow - move left, right arrow - move right"),
        chunks[4],
    );
}

fn draw_axis_block<B: Backend>(state: &AxisState, frame: &mut Frame<B>, target: Rect) {
    let axis1_data = vec![
        Spans::from(format!("enabled: {}", state.enabled)),
        Spans::from(format!("mode: {}", state.mode())),
        Spans::from(format!("target vel: {}", state.target_velocity)),
        Spans::from(format!("actual vel: {}", state.actual_velocity)),
        Spans::from(format!(
            "target pos: {} - {}",
            state.target_position.get_revolutions(),
            state.target_position.get_angle(),
        )),
        Spans::from(format!(
            "actual pos: {} - {}",
            state.actual_position.get_revolutions(),
            state.actual_position.get_angle(),
        )),
    ];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Axis parameters",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(axis1_data)
        .block(block)
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, target);
}

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let system_events = SystemEvents::new(Duration::from_millis(10));

    let axis_state = Arc::new(Mutex::new(AxisState::default()));

    let mut running = true;
    while running {
        terminal.draw(|frame| {
            draw(&axis_state.lock(), frame);
        })?;
        let position_increment = Position::<ENCODER_RESOLUTION>::new(0, 100);
        match system_events.recv() {
            SystemEvent::Input(key) => match key.code {
                KeyCode::Char('q') => running = false,
                KeyCode::Left => {
                    let mut state = axis_state.lock();
                    state.target_position = state.target_position - &position_increment;
                }
                KeyCode::Right => {
                    let mut state = axis_state.lock();
                    state.target_position = state.target_position + &position_increment;
                }
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
