use std::sync::Arc;

use crossterm::event::Event;

///
#[derive(Clone, Copy, Debug)]
pub enum InputState {
    Paused,
    Polling,
}

///
#[derive(Clone, Copy, Debug)]
pub enum InputEvent {
    Input(Event),
    State(InputState),
}

///
pub struct Input {
    desired_state: Arc<NotifiableMutex>,
}
