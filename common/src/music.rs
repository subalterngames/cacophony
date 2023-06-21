use super::midi_track::MidiTrack;
use serde::{Deserialize, Serialize};

/// The default music name.
pub const DEFAULT_MUSIC_NAME: &str = "My Music";

/// Tracks, notes, and metadata.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Music {
    /// The name of the music.
    pub name: String,
    /// The music tracks.
    pub midi_tracks: Vec<MidiTrack>,
    /// The index of the selected track.
    pub selected: Option<usize>,
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
}

impl Default for Music {
    fn default() -> Self {
        Self {
            name: DEFAULT_MUSIC_NAME.to_string(),
            midi_tracks: vec![],
            selected: None,
        }
    }
}
