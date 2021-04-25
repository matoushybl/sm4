use crossterm::event;
use crossterm::event::KeyEvent;
use std::time::{Duration, Instant};

pub enum SystemEvent {
    Input(KeyEvent),
    Tick,
}
pub struct SystemEvents {
    receiver: crossbeam::channel::Receiver<SystemEvent>,
}

impl SystemEvents {
    pub fn new(tick_rate: Duration) -> SystemEvents {
        let (tx, rx) = crossbeam::channel::unbounded();

        // taken from https://github.com/fdehau/tui-rs/blob/master/examples/crossterm_demo.rs
        std::thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                // poll for tick rate duration, if no events, sent tick event.
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));
                if event::poll(timeout).unwrap() {
                    if let crossterm::event::Event::Key(key) = event::read().unwrap() {
                        tx.send(SystemEvent::Input(key)).unwrap();
                    }
                }
                if last_tick.elapsed() >= tick_rate {
                    tx.send(SystemEvent::Tick).unwrap();
                    last_tick = Instant::now();
                }
            }
        });

        Self { receiver: rx }
    }

    pub fn recv(&self) -> SystemEvent {
        self.receiver.recv().unwrap()
    }
}
