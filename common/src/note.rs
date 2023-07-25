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
/// The name of each note, in order.
/// Question: Why not just calculate the name from the MIDI value?
/// Answer: 1) Because I couldn't find an accurate formula. 2) This is probably slightly faster.
pub const NOTE_NAMES: [&str; 115] = [
    "G9", "F#9", "F9", "E9", "D#9", "D9", "C#9", "C9", "B9", "A#9", "A9", "G#9", "G8", "F#8", "F8",
    "E8", "D#8", "D8", "C#8", "C8", "B8", "A#8", "A8", "G#8", "G7", "F#7", "F7", "E7", "D#7", "D7",
    "C#7", "C7", "B7", "A#7", "A7", "G#7", "G6", "F#6", "F6", "E6", "D#6", "D6", "C#6", "C6", "B6",
    "A#6", "A6", "G#6", "G5", "F#5", "F5", "E5", "D#5", "D5", "C#5", "C5", "B5", "A#5", "A5",
    "G#5", "G4", "F#4", "F4", "E4", "D#4", "D4", "C#4", "C4", "B4", "A#4", "A4", "G#4", "G3",
    "F#3", "F3", "E3", "D#3", "D3", "C#3", "C3", "B3", "A#3", "A3", "G#3", "G2", "F#2", "F2", "E2",
    "D#2", "D2", "C#2", "C2", "B2", "A#2", "A2", "G#2", "G1", "F#1", "F1", "E1", "D#1", "D1",
    "C#1", "C1", "B1", "A#1", "A1", "G#1", "G0", "F#0", "F0", "E0", "D#0", "D0", "C#0",
];

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

    /// Returns the name of the note.
    pub fn get_name(&self) -> &str {
        NOTE_NAMES[self.note as usize]
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
