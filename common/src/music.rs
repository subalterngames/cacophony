use super::midi_track::MidiTrack;
use serde::{Deserialize, Serialize};

/// Tracks, notes, and metadata.
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct Music {
    /// The music tracks.
    pub midi_tracks: Vec<MidiTrack>,
    /// The index of the selected track.
    pub selected: Option<usize>,
    /// If true, at least one note or effect has been changed.
    #[serde(skip_serializing, skip_deserializing)]
    pub dirty: bool,
}

impl Music {
    /// Returns the selected track, if any.
    pub fn get_selected_track(&self) -> Option<&MidiTrack> {
        match self.selected {
            Some(index) => Some(&self.midi_tracks[index]),
            None => None,
        }
    }

    /// Returns a mutable reference to the selected track, if any.
    pub fn get_selected_track_mut(&mut self) -> Option<&mut MidiTrack> {
        match self.selected {
            Some(index) => Some(&mut self.midi_tracks[index]),
            None => None,
        }
    }

    /// Returns all tracks that can be played.
    pub fn get_playable_tracks(&self) -> Vec<&MidiTrack> {
        // Get all tracks that can play music.
        let tracks = match self.midi_tracks.iter().find(|t| t.solo) {
            // Only include the solo track.
            Some(solo) => vec![solo],
            // Only include unmuted tracks.
            None => self.midi_tracks.iter().filter(|t| !t.mute).collect(),
        };
        tracks
    }
}
