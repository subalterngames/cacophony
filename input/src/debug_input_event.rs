use crate::InputEvent;

#[cfg(debug_assertions)]
pub enum DebugInputEvent {
    InputEvent(InputEvent),
    Alphanumeric(char),
}
