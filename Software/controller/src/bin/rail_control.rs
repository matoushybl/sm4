use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use i2cdev::core::I2CDevice;
use i2cdev::core::I2CMessage;
use i2cdev::core::I2CTransfer;
use i2cdev::linux::LinuxI2CDevice;
use i2cdev::linux::LinuxI2CError;
use parking_lot::Mutex;
use sm4_controller::canopen_backend::AxisState;
use sm4_controller::canopen_backend::ENCODER_RESOLUTION;
use sm4_controller::tui::SystemEvent;
use sm4_controller::tui::SystemEvents;
use sm4_shared::prelude::Axis;
use sm4_shared::prelude::Position;
use sm4_shared::OnError;
use std::io::Write;
use std::sync::Arc;
use std::time::Duration;
use std::vec;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame, Terminal,
};

trait I2CRegisterTransfers<Error> {
    fn write_register<R: Into<u8>>(&mut self, register: R, data: &[u8]) -> Result<(), Error>;
    fn read_register<R: Into<u8>>(&mut self, register: R, buffer: &mut [u8]) -> Result<u32, Error>;
}

impl I2CRegisterTransfers<LinuxI2CError> for LinuxI2CDevice {
    fn write_register<R: Into<u8>>(
        &mut self,
        register: R,
        data: &[u8],
    ) -> Result<(), LinuxI2CError> {
        let mut buffer = vec![register.into()];
        buffer.extend_from_slice(data);
        self.write(&buffer)
    }

    fn read_register<R: Into<u8>>(
        &mut self,
        register: R,
        buffer: &mut [u8],
    ) -> Result<u32, LinuxI2CError> {
        self.transfer(&mut [
            I2CMessage::write(&[register.into()]),
            I2CMessage::read(buffer),
        ])
    }
}

#[derive(Clone)]
struct I2CBackend {
    state: Arc<Mutex<AxisState>>,
}

impl I2CBackend {
    fn new(device: LinuxI2CDevice) -> Self {
        let s = Self {
            state: Arc::new(Mutex::new(AxisState::default())),
        };

        std::thread::spawn({
            let s = s.clone();
            let mut device = device;
            device
                .write_register(0x10, &[0x11, 0x0f])
                .on_error(|_| println!("Failed to set driver mode"));
            move || loop {
                let mut data = [0u8; 8];
                {
                    let position = s.state.lock().target_position;
                    data[..4].clone_from_slice(&position.get_revolutions().to_le_bytes());
                    data[4..].clone_from_slice(&position.get_angle().to_le_bytes());
                }
                device
                    .write_register(0x31, &data)
                    .on_error(|_| println!("Failed to write target position."));

                // let mut buffer = [0u8; 16];
                // device
                //     .read_register(0x50, &mut buffer)
                //     .on_error(|_| println!("Failed to read positions."));
                std::thread::sleep(Duration::from_millis(50));
            }
        });

        s
    }

    fn set_target_position(&self, position: Position<ENCODER_RESOLUTION>) {
        self.state.lock().target_position = position;
    }

    fn get_target_position(&self) -> Position<ENCODER_RESOLUTION> {
        self.state.lock().target_position
    }

    fn get_state(&self) -> AxisState {
        *self.state.lock()
    }
}

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

    let system_events = SystemEvents::new(Duration::from_millis(50));

    // let axis_state = Arc::new(Mutex::new(AxisState::default()));

    let i2c = LinuxI2CDevice::new("/dev/i2c-11", 0x55).unwrap();
    let backend = I2CBackend::new(i2c);

    let mut running = true;
    while running {
        terminal.draw(|frame| {
            draw(&backend.get_state(), frame);
        })?;
        let position_increment = Position::<ENCODER_RESOLUTION>::new(0, 500);
        match system_events.recv() {
            SystemEvent::Input(key) => match key.code {
                KeyCode::Char('q') => running = false,
                KeyCode::Left => {
                    backend
                        .set_target_position(backend.get_target_position() - &position_increment);
                }
                KeyCode::Right => {
                    backend
                        .set_target_position(backend.get_target_position() + &position_increment);
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
