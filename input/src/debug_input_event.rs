use crate::InputEvent;

pub(crate) enum DebugInputEvent {
    InputEvent(InputEvent),
    Alphanumeric(char),
}
