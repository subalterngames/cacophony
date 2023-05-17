use super::midi_track::{MidiTrack, SerializableTrack};
use crate::{serialize_fraction, SerializableFraction, Fraction};
use fraction::Zero;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Tracks, notes, and metadata.
pub struct Music {
    /// The name of the music.
    pub name: String,
    /// The file path. If None, we haven't saved this music yet.
    pub path: Option<PathBuf>,
    /// The music tracks.
    pub midi_tracks: Vec<MidiTrack>,
    /// The beats per minute.
    pub bpm: u32,
    /// Start audio playback at this time.
    pub playback_time: Fraction,
    /// The index of the selected track.
    pub selected: Option<usize>,
}

impl Music {
    pub fn new(name: String) -> Self {
        Self {
            name,
            midi_tracks: vec![],
            bpm: 120,
            playback_time: Fraction::zero(),
            selected: None,
            path: None,
        }
    }

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

    /// Serialize to a `SerializableMusic`.
    pub(crate) fn serialize(&self) -> SerializableMusic {
        SerializableMusic {
            name: self.name.clone(),
            midi_tracks: self.midi_tracks.iter().map(|m| m.serialize()).collect(),
            bpm: self.bpm,
            playback_time: serialize_fraction(&self.playback_time),
            selected: self.selected,
            path: self.path.clone(),
        }
    }
}

/// Music that can be serialized.
#[derive(Serialize, Deserialize)]
pub(crate) struct SerializableMusic {
    /// The name of the music.
    name: String,
    /// The file path. If None, we haven't saved this music yet.
    path: Option<PathBuf>,
    /// The serializable tracks.
    midi_tracks: Vec<SerializableTrack>,
    /// The beats per minute.
    bpm: u32,
    /// Start audio playback at this time.
    playback_time: SerializableFraction,
    /// The index of the selected track.
    selected: Option<usize>,
}

impl SerializableMusic {
    /// Deserialize to a `Music`.
    pub(crate) fn deserialize(&self) -> Music {
        Music {
            name: self.name.clone(),
            midi_tracks: self.midi_tracks.iter().map(|m| m.deserialize()).collect(),
            bpm: self.bpm,
            playback_time: Fraction::new(self.playback_time[0], self.playback_time[1]),
            selected: self.selected,
            path: self.path.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::note::Note;
    use super::{MidiTrack, Music, SerializableMusic, Fraction};
    use serde_json::{from_str, to_string};

    #[test]
    fn serialize() {
        let mut music = Music::new("my music".to_string());
        let mut track = MidiTrack::new(0);
        track.notes.push(Note {
            note: 60,
            velocity: 100,
            start: Fraction::new(1u8, 2u8),
            duration: Fraction::new(2u8, 1u8),
        });
        track.notes.push(Note {
            note: 61,
            velocity: 90,
            start: Fraction::new(3u8, 2u8),
            duration: Fraction::new(4u8, 7u8),
        });
        music.midi_tracks.push(track);
        let s = to_string(&music.serialize());
        assert!(s.is_ok());
        let serialized = s.unwrap();
        let m: Result<SerializableMusic, _> = from_str(&serialized);
        assert!(m.is_ok());
        let m1 = m.unwrap().deserialize();
        assert_eq!(m1.midi_tracks.len(), 1);
        assert_eq!(m1.midi_tracks[0].notes.len(), 2);
        assert_eq!(m1.midi_tracks[0].notes[0].note, 60);
        assert_eq!(m1.midi_tracks[0].notes[0].velocity, 100);
        assert_eq!(m1.midi_tracks[0].notes[1].note, 61);
        assert_eq!(m1.midi_tracks[0].notes[1].velocity, 90);
    }
}
