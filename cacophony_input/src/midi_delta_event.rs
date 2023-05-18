/// Handle MIDI event deltas i.e. events where we don't care what the value byte is, only what the delta is.
pub struct MidiDeltaEvent {
    /// The two bytes defining the MIDI input device.
    pub bytes: [u8; 2],
    /// If true, this is a positive delta. If false, this is a negative delta.
    positive: bool,
    /// A value that controls the sensitivity of the events. Check for events every `nth` consecutive inputs.
    nth: u8,
    /// A counter for tracking sensitivity.
    counter: u8,
}

impl MidiDeltaEvent {
    pub(crate) fn new(bytes: [u8; 2], positive: bool, nth: u8) -> Self {
        Self {
            bytes,
            positive,
            nth,
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
                self.counter % self.nth == 0
            } else {
                false
            };
            return is_event;
        }
        false
    }
}
