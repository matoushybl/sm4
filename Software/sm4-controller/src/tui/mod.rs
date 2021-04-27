pub mod events;

use crate::canopen_backend::{AxisState, State};
pub use events::{SystemEvent, SystemEvents};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::Frame;

pub fn draw<B: Backend>(state: &State, frame: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(35),
                Constraint::Percentage(60),
                Constraint::Percentage(5),
            ]
            .as_ref(),
        )
        .split(frame.size());
    draw_header_block(state, frame, chunks[0]);

    let axis_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    draw_axis_block(&state.axis1, frame, axis_chunks[0]);
    draw_axis_block(&state.axis2, frame, axis_chunks[1]);

    frame.render_widget(
        Paragraph::new(
            "q - quit   e - enable   o - A1 up   k - A1 down   n - toggle A1 mode   p - A2 vel up   l - A2 vel down   m - toggle A2 mode",
        ),
        chunks[2],
    );
}

fn draw_header_block<B: Backend>(state: &State, frame: &mut Frame<B>, target: Rect) {
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "SM4 velocity controller - common",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));

    let received = vec![
        Spans::from(format!("NMT: {}", state.nmt_state())),
        Spans::from(format!("temp: {}", state.temperature)),
        Spans::from(format!("voltage: {}", state.voltage)),
    ];

    let paragraph = Paragraph::new(received)
        .block(block)
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, target);
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
        "Axis 1",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(axis1_data)
        .block(block)
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, target);
}
