use crate::{ALPHANUMERIC_INPUT_MODS, MODS};
use common::macroquad::input::KeyCode;
use serde::Deserialize;
use serde_json::{from_str, Error};

/// A list of qwerty keys plus mods that define a qwerty key binding.
#[derive(Clone)]
pub struct QwertyBinding {
    /// The keys that were pressed on this frame.
    pub keys: Vec<KeyCode>,
    /// The modifiers that are being held down.
    pub mods: Vec<KeyCode>,
    /// All mods that are *not* part of this qwerty binding.
    non_mods: Vec<KeyCode>,
    /// Wait this many frame for a repeat event.
    sensitivity: u64,
    /// The frame of the most recent press.
    frame: u64,
    /// If true, listen to down events.
    repeatable: bool,
    /// If true, this event is pressed.
    pub pressed: bool,
}

impl QwertyBinding {
    /// Deserialize a serializable version of this binding from a string.
    pub(crate) fn deserialize(string: &str) -> Self {
        let q: Result<SerializableQwertyBinding, Error> = from_str(string);
        match q {
            Ok(q) => {
                if q.keys.iter().any(|k| keycode_from_str(k).is_none()) || q.mods.iter().any(|k| keycode_from_str(k).is_none()) {
                    panic!("Invalid qwerty binding: {}", string)
                }
                let keys = q
                    .keys
                    .iter()
                    .map(|s| keycode_from_str(s))
                    .filter(|s| s.is_some())
                    .flatten()
                    .collect();
                let mods: Vec<KeyCode> = q
                    .mods
                    .iter()
                    .map(|s| keycode_from_str(s))
                    .filter(|s| s.is_some())
                    .flatten()
                    .collect();
                let sensitivity = q.dt;
                let non_mods = MODS.iter().filter(|m| !mods.contains(m)).copied().collect();
                let repeatable = sensitivity > 0;
                Self {
                    keys,
                    mods,
                    non_mods,
                    repeatable,
                    sensitivity,
                    frame: 0,
                    pressed: false,
                }
            }
            Err(error) => panic!(
                "Failed to deserialize {} into a QwertyBinding: {}",
                string, error
            ),
        }
    }

    /// Update the state of this key binding.
    ///
    /// The keys are pressed if:
    ///
    /// - All of the `mods` are down.
    /// - No other mods are down.
    /// - Either alphanumeric input is disabled or this key is allowed in the context of alphanumeric input.
    /// - All of the `keys` are pressed.
    ///
    /// They keys are down if:
    ///
    /// - All of the above is true.
    /// - A sufficient number of frames have elapsed.
    ///
    /// Parameters:
    ///
    /// - `pressed` The keys that were pressed on this frame.
    /// - `down` The keys that were held down on this frame.
    /// - `alphanumeric` If true, we're in alphanumeric input mode, which can affect whether we can listen for certain qwerty bindings.
    pub(crate) fn update(&mut self, pressed: &[KeyCode], down: &[KeyCode], alphanumeric: bool) {
        self.pressed = false;
        // Mods.
        if self.mods.iter().all(|m| down.contains(m))
            && !self.non_mods.iter().any(|m| down.contains(m))
        {
            // Pressed.
            if self.keys.iter().all(|k| {
                (!alphanumeric || !ALPHANUMERIC_INPUT_MODS.contains(k)) && pressed.contains(k)
            }) {
                self.pressed = true;
                self.frame = 0;
            }
            // Down.
            if self.repeatable
                && self.keys.iter().all(|k| {
                    (!alphanumeric || !ALPHANUMERIC_INPUT_MODS.contains(k)) && down.contains(k)
                })
            {
                if self.frame >= self.sensitivity {
                    self.frame = 0;
                    self.pressed = true;
                } else {
                    self.frame += 1;
                }
            }
        }
    }
}

/// A serializable version of a qwerty binding.
#[derive(Deserialize)]
struct SerializableQwertyBinding {
    /// The keys that were pressed on this frame.
    keys: Vec<String>,
    /// The modifiers that are being held down as strings.
    #[serde(default)]
    mods: Vec<String>,
    /// Wait this many frame for a repeat event.
    #[serde(default)]
    dt: u64,
}

fn keycode_from_str(s: &str) -> Option<KeyCode> {
    match s {
        "a" => Some(KeyCode::A),
        "A" => Some(KeyCode::A),
        "b" => Some(KeyCode::B),
        "B" => Some(KeyCode::B),
        "c" => Some(KeyCode::C),
        "C" => Some(KeyCode::C),
        "d" => Some(KeyCode::D),
        "D" => Some(KeyCode::D),
        "e" => Some(KeyCode::E),
        "E" => Some(KeyCode::E),
        "f" => Some(KeyCode::F),
        "F" => Some(KeyCode::F),
        "g" => Some(KeyCode::G),
        "G" => Some(KeyCode::G),
        "h" => Some(KeyCode::H),
        "H" => Some(KeyCode::H),
        "i" => Some(KeyCode::I),
        "I" => Some(KeyCode::I),
        "j" => Some(KeyCode::J),
        "J" => Some(KeyCode::J),
        "k" => Some(KeyCode::K),
        "K" => Some(KeyCode::K),
        "l" => Some(KeyCode::L),
        "L" => Some(KeyCode::L),
        "m" => Some(KeyCode::M),
        "M" => Some(KeyCode::M),
        "n" => Some(KeyCode::N),
        "N" => Some(KeyCode::N),
        "o" => Some(KeyCode::O),
        "O" => Some(KeyCode::O),
        "p" => Some(KeyCode::P),
        "P" => Some(KeyCode::P),
        "q" => Some(KeyCode::Q),
        "Q" => Some(KeyCode::Q),
        "r" => Some(KeyCode::R),
        "R" => Some(KeyCode::R),
        "s" => Some(KeyCode::S),
        "S" => Some(KeyCode::S),
        "t" => Some(KeyCode::T),
        "T" => Some(KeyCode::T),
        "u" => Some(KeyCode::U),
        "U" => Some(KeyCode::U),
        "v" => Some(KeyCode::V),
        "V" => Some(KeyCode::V),
        "w" => Some(KeyCode::W),
        "W" => Some(KeyCode::W),
        "x" => Some(KeyCode::X),
        "X" => Some(KeyCode::X),
        "y" => Some(KeyCode::Y),
        "Y" => Some(KeyCode::Y),
        "z" => Some(KeyCode::Z),
        "Z" => Some(KeyCode::Z),
        "0" => Some(KeyCode::Key0),
        "Kp0" => Some(KeyCode::Kp0),
        "1" => Some(KeyCode::Key1),
        "F1" => Some(KeyCode::F1),
        "Kp1" => Some(KeyCode::Kp1),
        "2" => Some(KeyCode::Key2),
        "F2" => Some(KeyCode::F2),
        "Kp2" => Some(KeyCode::Kp2),
        "3" => Some(KeyCode::Key3),
        "F3" => Some(KeyCode::F3),
        "Kp3" => Some(KeyCode::Kp3),
        "4" => Some(KeyCode::Key4),
        "F4" => Some(KeyCode::F4),
        "Kp4" => Some(KeyCode::Kp4),
        "5" => Some(KeyCode::Key5),
        "F5" => Some(KeyCode::F5),
        "Kp5" => Some(KeyCode::Kp5),
        "6" => Some(KeyCode::Key6),
        "F6" => Some(KeyCode::F6),
        "Kp6" => Some(KeyCode::Kp6),
        "7" => Some(KeyCode::Key7),
        "F7" => Some(KeyCode::F7),
        "Kp7" => Some(KeyCode::Kp7),
        "8" => Some(KeyCode::Key8),
        "F8" => Some(KeyCode::F8),
        "Kp8" => Some(KeyCode::Kp8),
        "9" => Some(KeyCode::Key9),
        "F9" => Some(KeyCode::F9),
        "Kp9" => Some(KeyCode::Kp9),
        "F10" => Some(KeyCode::F10),
        "F11" => Some(KeyCode::F11),
        "F12" => Some(KeyCode::F12),
        "`" => Some(KeyCode::GraveAccent),
        "-" => Some(KeyCode::Minus),
        "=" => Some(KeyCode::Equal),
        "[" => Some(KeyCode::LeftBracket),
        "]" => Some(KeyCode::RightBracket),
        "Backslash" => Some(KeyCode::Backslash),
        ";" => Some(KeyCode::Semicolon),
        "'" => Some(KeyCode::Apostrophe),
        "," => Some(KeyCode::Comma),
        "." => Some(KeyCode::Period),
        "/" => Some(KeyCode::Slash),
        "Space" => Some(KeyCode::Space),
        "Escape" => Some(KeyCode::Escape),
        "Tab" => Some(KeyCode::Tab),
        "Backspace" => Some(KeyCode::Backspace),
        "Insert" => Some(KeyCode::Insert),
        "Delete" => Some(KeyCode::Delete),
        "Right" => Some(KeyCode::Right),
        "Left" => Some(KeyCode::Left),
        "Up" => Some(KeyCode::Up),
        "Down" => Some(KeyCode::Down),
        "PageUp" => Some(KeyCode::PageUp),
        "PageDown" => Some(KeyCode::PageDown),
        "Home" => Some(KeyCode::Home),
        "End" => Some(KeyCode::End),
        "CapsLock" => Some(KeyCode::CapsLock),
        "ScrollLock" => Some(KeyCode::ScrollLock),
        "NumLock" => Some(KeyCode::NumLock),
        "PrintScreen" => Some(KeyCode::PrintScreen),
        "Pause" => Some(KeyCode::Pause),
        "KpDecimal" => Some(KeyCode::KpDecimal),
        "KpDivide" => Some(KeyCode::KpDivide),
        "KpMultiply" => Some(KeyCode::KpMultiply),
        "KpSubtract" => Some(KeyCode::KpSubtract),
        "KpAdd" => Some(KeyCode::KpAdd),
        "KpEnter" => Some(KeyCode::KpEnter),
        "KpEqual" => Some(KeyCode::KpEqual),
        "LeftShift" => Some(KeyCode::LeftShift),
        "LeftControl" => Some(KeyCode::LeftControl),
        "LeftAlt" => Some(KeyCode::LeftAlt),
        "LeftSuper" => Some(KeyCode::LeftSuper),
        "RightShift" => Some(KeyCode::RightShift),
        "RightControl" => Some(KeyCode::RightControl),
        "RightAlt" => Some(KeyCode::RightAlt),
        "RightSuper" => Some(KeyCode::RightSuper),
        "Return" => Some(KeyCode::Enter),
        other => {
            dbg!("Warning! Invalid key code: {}", other);
            None
        },
    }
}
