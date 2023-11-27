use oxisynth::MidiEvent;
use std::cmp::Ordering;

/// A MIDI event with a start time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct TimedMidiEvent {
    /// The event time in number of samples.
    pub(crate) time: u64,
    /// The event.
    pub(crate) event: MidiEvent,
}

impl Ord for TimedMidiEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.time.cmp(&other.time) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match (&self.event, &other.event) {
                // Two note-on events are equal.
                (
                    MidiEvent::NoteOn {
                        channel: _,
                        key: _,
                        vel: _,
                    },
                    MidiEvent::NoteOn {
                        channel: _,
                        key: _,
                        vel: _,
                    },
                ) => Ordering::Equal,
                // Two note-off events are equal.
                (
                    MidiEvent::NoteOff { channel: _, key: _ },
                    MidiEvent::NoteOff { channel: _, key: _ },
                ) => Ordering::Equal,
                // Note-off events are always before all other events.
                (MidiEvent::NoteOff { channel: _, key: _ }, _) => Ordering::Less,
                // Note-on events are always after note-offs.
                (
                    MidiEvent::NoteOn {
                        channel: _,
                        key: _,
                        vel: _,
                    },
                    MidiEvent::NoteOff { channel: _, key: _ },
                ) => Ordering::Greater,
                // Note-on events are always before all other events except note-offs.
                (
                    MidiEvent::NoteOn {
                        channel: _,
                        key: _,
                        vel: _,
                    },
                    _,
                ) => Ordering::Less,
                // All other events are equal.
                _ => Ordering::Equal,
            },
        }
    }
}

impl PartialOrd for TimedMidiEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
