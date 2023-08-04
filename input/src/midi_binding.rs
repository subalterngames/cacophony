use serde::Deserialize;

/// Bindings for MIDI input.
#[derive(Clone, Deserialize)]
pub struct MidiBinding {
    /// The two bytes defining the MIDI input device.
    pub bytes: [u8; 2],
    /// An alias name for the MIDI binding.
    #[serde(default)]
    pub alias: Option<String>,
    /// A value that controls the sensitivity of the events. Check for events every `nth` consecutive inputs. The sign defines positive or negative input.
    dt: i8,
}

impl MidiBinding {
    /// Update the event state. Returns true if the event happened.
    pub(crate) fn update(&mut self, buffer: &[[u8; 3]], counter: i8) -> bool {
        if let Some(b) = buffer
            .iter()
            .find(|b| b[0] == self.bytes[0] && b[1] == self.bytes[1])
        {
            // Did this trigger the event?
            if (self.dt > 0 && b[2] == 1) || (self.dt < 0 && b[2] == 127) {
                counter % self.dt.abs() == 0
            } else {
                false
            }
        } else {
            false
        }
    }
}
