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
}

impl MidiTrack {
    pub fn new(channel: u8) -> Self {
        Self {
            channel,
            gain: MAX_VOLUME,
            notes: vec![],
            mute: false,
            solo: false,
        }
    }

    /// Returns the end time of the track in PPQ.
    pub fn get_end(&self) -> Option<u64> {
        self.notes.iter().map(|n| n.end).max()
    }

    /// Returns all notes in the track that can be played (they are after t0).
    pub fn get_playback_notes(&self, start: u64) -> Vec<Note> {
        let gain = self.gain as f64 / MAX_VOLUME as f64;
        let mut notes = vec![];
        for note in self.notes.iter().filter(|n| n.start >= start) {
            let mut n1 = *note;
            n1.velocity = (n1.velocity as f64 * gain) as u8;
            notes.push(n1);
        }
        notes.sort();
        notes
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
