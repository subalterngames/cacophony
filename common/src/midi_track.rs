use crate::{Note, MAX_VOLUME};
use serde::{Deserialize, Serialize};

/// A MIDI track has some notes.
#[derive(Debug, Deserialize, Serialize)]
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
    /// The name of the track. 
    /// This is used for imported MIDI tracks. 
    /// The SoundFont preset name supersedes it.
    pub name: Option<String>,
}

impl MidiTrack {
    pub fn new(channel: u8) -> Self {
        Self {
            channel,
            gain: MAX_VOLUME,
            notes: vec![],
            mute: false,
            solo: false,
            name: None
        }
    }

    /// Returns the end time of the track in PPQ.
    pub fn get_end(&self) -> Option<u64> {
        self.notes.iter().map(|n| n.end).max()
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
            name: self.name.clone()
        }
    }
}
