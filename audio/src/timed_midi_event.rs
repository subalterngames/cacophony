use oxisynth::MidiEvent;

/// A MIDI event with a start time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct TimedMidiEvent {
    /// The event time in number of samples.
    pub(crate) time: u64,
    /// The event.
    pub(crate) event: MidiEvent,
}