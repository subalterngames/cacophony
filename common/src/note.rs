use crate::{deserialize_fraction, serialize_fraction, Fraction, SerializableFraction};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Result};

pub const MAX_NOTE: u8 = 127;
pub const MIN_NOTE: u8 = 21;
pub const MAX_VOLUME: u8 = 127;

/// A MIDI note with a start bar time and a duration bar time.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Note {
    /// The MIDI note value (0-127).
    pub note: u8,
    /// The velocity value (0-127).
    pub velocity: u8,
    /// The start bar time.
    pub start: Fraction,
    /// The duration bar time.
    pub duration: Fraction,
}

impl Note {
    /// Serialize to a `SerializableNote`.
    pub(crate) fn serialize(&self) -> SerializableNote {
        SerializableNote {
            n: self.note,
            v: self.velocity,
            s: serialize_fraction(&self.start),
            d: serialize_fraction(&self.duration),
        }
    }
}

impl Ord for Note {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.start, self.start + self.duration, self.note).cmp(&(
            other.start,
            other.start + other.duration,
            other.note,
        ))
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for Note {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let start = format!(
            "{}/{}",
            self.start.numer().unwrap(),
            self.start.denom().unwrap()
        );
        let duration = format!(
            "{}/{}",
            self.duration.numer().unwrap(),
            self.duration.denom().unwrap()
        );
        write!(
            f,
            "Note {} {} {} {}",
            self.note, self.velocity, start, duration
        )
    }
}

/// A serializable note, with reduced key names.
#[derive(Serialize, Deserialize, Copy, Clone)]
pub(crate) struct SerializableNote {
    /// The MIDI note value (0-127).
    n: u8,
    /// The velocity value (0-127).
    v: u8,
    /// The start bar time.
    s: SerializableFraction,
    /// The duration bar time.
    d: SerializableFraction,
}

impl SerializableNote {
    /// Deserialize to a `Note`.
    pub(crate) fn deserialize(&self) -> Note {
        Note {
            note: self.n,
            velocity: self.v,
            start: deserialize_fraction(&self.s),
            duration: deserialize_fraction(&self.d),
        }
    }
}
