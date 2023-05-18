use crate::{ALPHANUMERIC_INPUT_MODS, MODS};
use macroquad::input::KeyCode;

/// A list of qwerty keys plus mods that define a qwerty key binding.
#[derive(Clone)]
pub struct QwertyBinding {
    /// The keys that were pressed on this frame.
    pub keys: Vec<KeyCode>,
    /// The modifiers that are being held down, e.g. LCtrl.
    pub mods: Vec<KeyCode>,
    /// All mods that are *not* part of this qwerty binding.
    non_mods: Vec<KeyCode>,
    /// Wait this many frame for a repeat event.
    sensitivity: u64,
    /// The frame of the most recent press.
    frame: u64,
    /// If true, this event is pressed.
    pub pressed: bool,
    /// If true, this event is down.
    pub down: bool,
}

impl QwertyBinding {
    pub fn new(keys: Vec<KeyCode>, mods: Vec<KeyCode>, sensitivity: u64) -> Self {
        let non_mods = MODS
            .iter()
            .filter(|m| !mods.contains(m))
            .map(|m| *m)
            .collect();
        Self {
            keys,
            mods,
            non_mods,
            sensitivity,
            frame: 0,
            pressed: false,
            down: false,
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
    pub fn update(&mut self, pressed: &[KeyCode], down: &[KeyCode], alphanumeric: bool) {
        self.pressed = false;
        self.down = false;
        // Mods.
        if self.mods.iter().all(|m| down.contains(m))
            && !self.non_mods.iter().any(|m| down.contains(m))
        {
            // Pressed.
            if self.keys.iter().all(|k| {
                (!alphanumeric || !ALPHANUMERIC_INPUT_MODS.contains(&k)) && pressed.contains(k) }) {
                self.pressed = true;
                self.frame = 0;
            }
            // Down.
            if self.keys.iter().all(|k| {
                (!alphanumeric || !ALPHANUMERIC_INPUT_MODS.contains(&k)) && down.contains(k) }) {
                    if self.frame >= self.sensitivity {
                        self.frame = 0;
                        self.down = true;
                    }
                    else {
                        self.frame += 1;
                    }
            }
        }
    }
}
