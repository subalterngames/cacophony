use crate::Fraction;
use super::note::{Note, SerializableNote};
use serde::{Deserialize, Serialize};

/// A MIDI track has some notes.
pub struct MidiTrack {
    /// The channel used for audio synthesis.
    pub channel: u8,
    /// A gain value (0-127) for this track.
    pub gain: u8,
    /// The notes in the track.
    pub notes: Vec<Note>,
    /// True if the track is muted.
    pub mute: bool,
    /// True if the track is soloed.
    pub solo: bool,
}

impl MidiTrack {
    pub fn new(channel: u8) -> Self {
        Self {
            channel,
            gain: 127,
            notes: vec![],
            mute: false,
            solo: false,
        }
    }

    /// Returns the end time of the track.
    pub fn get_end(&self) -> Option<Fraction> {
        self.notes.iter().map(|n| n.start + n.duration).max()
    }

    /// Serialize to a `SerializableTrack`.
    pub(crate) fn serialize(&self) -> SerializableTrack {
        SerializableTrack {
            channel: self.channel,
            gain: self.gain,
            notes: self.notes.iter().map(|n| n.serialize()).collect(),
            mute: self.mute,
            solo: self.solo,
        }
    }
}

impl Clone for MidiTrack {
    fn clone(&self) -> Self {
        Self {
            channel: self.channel,
            gain: self.gain,
            notes: self.notes.clone(),
            mute: self.mute,
            solo: self.solo,
        }
    }
}

/// A track that can be serialized.
#[derive(Serialize, Deserialize)]
pub(crate) struct SerializableTrack {
    /// The channel used for audio synthesis.
    channel: u8,
    /// A gain value (0-127) for this track.
    gain: u8,
    /// The notes in the track.
    notes: Vec<SerializableNote>,
    /// True if the track is muted.
    mute: bool,
    /// True if the track is soloed.
    solo: bool,
}

impl SerializableTrack {
    /// Deserialize to a `MidiTrack`.
    pub(crate) fn deserialize(&self) -> MidiTrack {
        MidiTrack {
            channel: self.channel,
            gain: self.gain,
            notes: self.notes.iter().map(|n| n.deserialize()).collect(),
            mute: self.mute,
            solo: self.solo,
        }
    }
}
