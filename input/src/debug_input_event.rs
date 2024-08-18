use crate::InputEvent;

#[cfg(debug_assertions)]
pub(crate) enum DebugInputEvent {
    InputEvent(InputEvent),
    Alphanumeric(char),
}
