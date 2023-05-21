use common::macroquad::input::KeyCode;

/// The keycodes for the mods.
pub(crate) const MODS: [KeyCode; 7] = [
    KeyCode::LeftControl,
    KeyCode::RightControl,
    KeyCode::LeftAlt,
    KeyCode::RightAlt,
    KeyCode::LeftShift,
    KeyCode::RightShift,
    KeyCode::CapsLock,
];
pub(crate) const ALPHANUMERIC_INPUT_MODS: [KeyCode; 2] = [KeyCode::Backspace, KeyCode::CapsLock];
