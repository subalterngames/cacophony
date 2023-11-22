use crate::event_type::EventType;
use std::cmp::Ordering;

/// An event with a start time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct TimedEvent {
    /// The event time in number of samples.
    pub(crate) time: u64,
    /// The event.
    pub(crate) event: EventType,
}

impl Ord for TimedEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.time.cmp(&other.time) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match (&self.event, &other.event) {
                // Two note-on events are equal.
                (
                    EventType::NoteOn {
                        channel: _,
                        key: _,
                        vel: _,
                    },
                    EventType::NoteOn {
                        channel: _,
                        key: _,
                        vel: _,
                    },
                ) => Ordering::Equal,
                // Two note-off events are equal.
                (
                    EventType::NoteOff { channel: _, key: _ },
                    EventType::NoteOff { channel: _, key: _ },
                ) => Ordering::Equal,
                // Note-off events are always before all other events.
                (EventType::NoteOff { channel: _, key: _ }, _) => Ordering::Less,
                // Note-on events are always after note-offs.
                (
                    EventType::NoteOn {
                        channel: _,
                        key: _,
                        vel: _,
                    },
                    EventType::NoteOff { channel: _, key: _ },
                ) => Ordering::Greater,
                // Note-on events are always before all other events except note-offs.
                (
                    EventType::NoteOn {
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

impl PartialOrd for TimedEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
