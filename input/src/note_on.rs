/// A note-on event. When `off` is true, the event is done.
pub(crate) struct NoteOn {
    /// The note MIDI information.
    pub(super) note: [u8; 3],
    /// If true, the note-off event occurred.
    pub(super) off: bool,
}

impl NoteOn {
    pub(crate) fn new(note: &[u8; 3]) -> Self {
        Self {
            note: *note,
            off: false,
        }
    }
}
