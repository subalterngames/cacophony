/// Bindings for MIDI input.
#[derive(Clone)]
pub struct MidiBinding {
    /// The two bytes defining the MIDI input device.
    pub bytes: [u8; 2],
    /// If true, this is a positive delta. If false, this is a negative delta.
    positive: bool,
    /// A value that controls the sensitivity of the events. Check for events every `nth` consecutive inputs.
    sensitivity: u8,
    /// A counter for tracking sensitivity.
    counter: u8,
}

impl MidiBinding {
    pub(crate) fn new(bytes: [u8; 2], positive: bool, sensitivity: u8) -> Self {
        Self {
            bytes,
            positive,
            sensitivity,
            counter: 0,
        }
    }

    /// Update the event state. Returns true if the event happened.
    pub(crate) fn update(&mut self, buffer: &Vec<[u8; 3]>) -> bool {
        // Search for my event in the buffer.
        for b in buffer
            .iter()
            .filter(|b| b[0] == self.bytes[0] && b[1] == self.bytes[1])
        {
            if self.counter == 255 {
                self.counter = 0;
            } else {
                self.counter += 1;
            }
            // Did this trigger the event?
            let is_event = if (self.positive && b[2] != 127) || (!self.positive && b[2] == 127) {
                self.counter % self.sensitivity == 0
            } else {
                false
            };
            return is_event;
        }
        false
    }
}
