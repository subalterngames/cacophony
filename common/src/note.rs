use serde::ser::SerializeSeq;
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
#[derive(Copy, Clone, PartialEq, Eq, Deserialize)]
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

impl Note {
    /// Returns the duration of the note in PPQ.
    pub fn get_duration(&self) -> u64 {
        self.end - self.start
    }

    /// Adjust the start and end times by a delta (`dt`).
    pub fn set_t0_by(&mut self, dt: u64, positive: bool) {
        if positive {
            self.start += dt;
            self.end += dt;
        } else {
            self.start -= dt;
            self.end -= dt;
        }
    }
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

impl Serialize for Note {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(4)).unwrap();
        seq.serialize_element(&self.note).unwrap();
        seq.serialize_element(&self.velocity).unwrap();
        seq.serialize_element(&self.start).unwrap();
        seq.serialize_element(&self.end).unwrap();
        seq.end()
    }
}

#[cfg(test)]
mod tests {
    use crate::note::MIDDLE_C;
    use crate::{Note, MAX_VOLUME, PPQ_U};
    use serde_json::{from_str, to_string};

    #[test]
    fn note_duration() {
        let note = get_note();
        assert_eq!(note.get_duration(), PPQ_U, "{}", note.get_duration());
    }

    #[test]
    fn note_serialization() {
        let note = get_note();
        let r = to_string(&note);
        assert!(r.is_ok(), "{:?}", note);
        let s = r.unwrap();
        assert_eq!(&s, "[60,127,0,192]", "{}", s);
        let r = from_str(&s);
        assert!(r.is_ok(), "{:?}", s);
        let note: Note = r.unwrap();
        assert_eq!(note.note, MIDDLE_C, "{:?}", note);
        assert_eq!(note.velocity, MAX_VOLUME, "{:?}", note);
        assert_eq!(note.start, 0, "{:?}", note);
        assert_eq!(note.end, PPQ_U, "{:?}", note);
    }

    fn get_note() -> Note {
        Note {
            note: MIDDLE_C,
            velocity: MAX_VOLUME,
            start: 0,
            end: PPQ_U,
        }
    }
}
