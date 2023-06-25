use common::Note;

/// A MIDI note contains a note and some other useful information.
pub(super) struct MidiNote {
    /// The `Note`.
    pub(super) note: Note,
    /// The channel of the note's track.
    pub(super) channel: u8,
}

impl MidiNote {
    pub(super) fn new(note: &Note, channel: u8) -> Self {
        Self {
            note: *note,
            channel,
        }
    }
}
