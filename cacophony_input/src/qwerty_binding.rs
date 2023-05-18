use crate::{ALPHANUMERIC_INPUT_MODS, MODS};
use macroquad::input::KeyCode;

/// A list of qwerty keys plus mods that define a qwerty key binding.
pub struct QwertyBinding {
    /// The keys that were pressed on this frame.
    pub keys: Vec<KeyCode>,
    /// The modifiers that are being held down, e.g. LCtrl.
    pub mods: Vec<KeyCode>,
    /// All mods that are *not* part of this qwerty binding.
    non_mods: Vec<KeyCode>,
}

impl QwertyBinding {
    pub fn new(keys: Vec<KeyCode>, mods: Vec<KeyCode>) -> Self {
        let non_mods = MODS
            .iter()
            .filter(|m| !mods.contains(m))
            .map(|m| *m)
            .collect();
        Self {
            keys,
            mods,
            non_mods,
        }
    }

    /// Returns true if this qwerty binding is being pressed on this frame:
    ///
    /// - All of the `mods` are down.
    /// - No other mods are down.
    /// - Either alphanumeric input is disabled or this key is allowed in the context of alphanumeric input.
    /// - All of the `keys` are pressed.
    ///
    /// Parameters:
    ///
    /// - `pressed` The keys that were pressed on this frame.
    /// - `down` The keys that were held down on this frame.
    /// - `alphanumeric` If true, we're in alphanumeric input mode, which can affect whether we can listen for certain qwerty bindings.
    pub fn pressed(&self, pressed: &[KeyCode], down: &[KeyCode], alphanumeric: bool) -> bool {
        self.mods.iter().all(|m| down.contains(m))
            && !self.non_mods.iter().any(|m| down.contains(m))
            && self.keys.iter().all(|k| {
                (!alphanumeric || !ALPHANUMERIC_INPUT_MODS.contains(&k)) && pressed.contains(k)
            })
    }
}
