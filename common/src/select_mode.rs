use crate::{Music, Note};
use serde::{Deserialize, Serialize};

/// The current mode for selecting notes.
#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub enum SelectMode {
    /// Select only one note. The value is an index in `track.notes`.
    Single(Option<usize>),
    /// Select many: A vec of indices.
    Many(Option<Vec<usize>>),
}

impl Clone for SelectMode {
    fn clone(&self) -> Self {
        match self {
            SelectMode::Single(index) => SelectMode::Single(*index),
            SelectMode::Many(indices) => SelectMode::Many(indices.clone()),
        }
    }
}

impl SelectMode {
    /// Converts and returns the selection as a list of indices in the selected track's `notes`.
    pub fn get_note_indices(&self) -> Option<Vec<usize>> {
        match self {
            // A single note is selected.
            SelectMode::Single(index) => index.as_ref().map(|index| vec![*index]),
            SelectMode::Many(indices) => indices.as_ref().cloned(),
        }
    }

    /// Converts and returns the selection as a list of cloned notes from the selected track's `notes`.
    ///
    /// - `music` The music.
    pub fn get_notes<'a>(&self, music: &'a Music) -> Option<Vec<&'a Note>> {
        match music.get_selected_track() {
            None => None,
            Some(track) => match self {
                SelectMode::Single(index) => index.as_ref().map(|index| vec![&track.notes[*index]]),
                SelectMode::Many(indices) => indices
                    .as_ref()
                    .map(|indices| indices.iter().map(|&i| &track.notes[i]).collect()),
            },
        }
    }

    /// Converts and returns the selection as a list of cloned notes from the selected track's `notes`.
    ///
    /// - `music` The music.
    pub fn get_notes_mut<'a>(&mut self, music: &'a mut Music) -> Option<Vec<&'a mut Note>> {
        match music.get_selected_track_mut() {
            None => None,
            Some(track) => match self {
                SelectMode::Single(index) => match index {
                    Some(index) => Some(vec![&mut track.notes[*index]]),
                    None => None,
                },
                SelectMode::Many(indices) => match indices {
                    Some(indices) => Some(
                        track
                            .notes
                            .iter_mut()
                            .enumerate()
                            .filter(|n| indices.contains(&n.0))
                            .map(|n| n.1)
                            .collect(),
                    ),
                    None => None,
                },
            },
        }
    }
}
