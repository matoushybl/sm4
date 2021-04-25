pub mod canopen_backend;
pub mod gui;
pub mod tui;

#[derive(Copy, Clone, PartialOrd, PartialEq, Default, Debug)]
pub struct State {
    pub voltage: f32,
    pub temperature: f32,
}
