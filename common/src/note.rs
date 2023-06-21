use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Result};

/// The MIDI value of the highest-frequency note.
pub const MAX_NOTE: u8 = 127;
/// The MIDI value of the lowest-frequency note.
pub const MIN_NOTE: u8 = 12;
/// The MIDI value for C4.
pub const MIDDLE_C: u8 = 60;

/// A MIDI note with a start bar time and a duration bar time.
#[derive(Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Note {
    /// The MIDI note value.
    pub note: u8,
    /// The velocity value.
    pub velocity: u8,
    /// The start time in PPQ (pulses per quarter note).
    pub start: u64,
    /// The end time in PPQ.
    pub end: u64,
}

impl Ord for Note {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.start, self.end, self.note).cmp(&(other.start, other.end, other.note))
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for Note {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Note {} {} {} {}",
            self.note, self.velocity, self.start, self.end
        )
    }
}
