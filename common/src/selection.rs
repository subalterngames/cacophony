use crate::{Effect, Music, Note, MidiTrack, selectable};
use serde::{Deserialize, Serialize};

enum Selectable<'track> {
    Effect{ effect: &'track Effect, index: usize},
    Note{ note: &'track Note, index: usize}
}

impl<'track> Selectable<'track> {
    fn get_start_time(&self) -> u64 {
        match self {
            Self::Effect {effect, index: _} => effect.time,
            Self::Note{note, index: _} => note.start
        }
    }

    fn get_end_time(&self) -> u64 {
        match self {
            Self::Effect {effect, index: _} => effect.time,
            Self::Note{note, index: _} => note.end
        }
    }
}

/// A selection of notes an effects.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Selection {
    /// The selected notes.
    pub notes: Vec<usize>,
    /// The selected effects.
    pub effects: Vec<usize>,
    /// If true, only one note or effect may be selected at a time.
    pub single: bool,
}

impl Selection {
    /// Deselect all notes and effects.
    pub fn deselect(&mut self) {
        self.notes.clear();
        self.effects.clear();
    }

    /// Returns the selected notes and effects.
    ///
    /// - `music` The music.
    pub fn get_selection<'a>(&self, music: &'a Music) -> Option<(Vec<&'a Note>, Vec<&'a Effect>)> {
        match music.get_selected_track() {
            None => None,
            Some(track) => Some((
                self.notes.iter().map(|i| &track.notes[*i]).collect(),
                self.effects.iter().map(|i| &track.effects[*i]).collect(),
            )),
        }
    }

    pub fn get_selection_mut<'a>(
        &self,
        music: &'a mut Music,
    ) -> Option<(Vec<&'a mut Note>, Vec<&'a mut Effect>)> {
        match music.get_selected_track_mut() {
            None => None,
            Some(track) => Some((
                track
                    .notes
                    .iter_mut()
                    .enumerate()
                    .filter(|n| self.notes.contains(&n.0))
                    .map(|n| n.1)
                    .collect(),
                track
                    .effects
                    .iter_mut()
                    .enumerate()
                    .filter(|n| self.effects.contains(&n.0))
                    .map(|n| n.1)
                    .collect(),
            )),
        }
    }

    pub fn select_previous(&self, music: &Music) -> bool {
        match Self::get_music_selectables(music) {
            Some(music_selectables) => {
                let selectables = self.get_selectables(music).unwrap();
                // Nothing is selected.
                if selectables.is_empty() {
                    // There is nothing to select.
                    if music_selectables.is_empty() {
                        false
                    }
                    // Select the first available option.
                    else {

                    }
                }
                else {

                }
            }
            None => false
        }
        // Get all selected notes and effects.
        let selectables = self.get_selectables(music);
    }

    fn get_selectables<'track>(&self, music: &'track Music) -> Option<Vec<Selectable<'track>>> {
        match self.get_selection(music) {
            Some((notes, effects)) => {
                let mut selectables = notes.iter().enumerate().map(|(index, note)| Selectable::Note{note, index}).collect::<Vec<Selectable>>();
                selectables.extend(effects.iter().enumerate().map(|(index, effect)| Selectable::Effect {effect, index}));
                // Sort the selectables by start time.
                selectables.sort_by(|a, b| a.get_start_time().cmp(&b.get_start_time()));
                Some(selectables)
            }
            None => None
        }
    }

    fn get_music_selectables(music: &Music) -> Option<Vec<Selectable<'track>>> {
        match music.get_selected_track_mut() {
            Some(track) => {
                let mut selectables = track.notes.iter().enumerate().map(|(index, note)| Selectable::Note{note, index}).collect::<Vec<Selectable>>();
                selectables.extend(track.effects.iter().enumerate().map(|(index, effect)| Selectable::Effect {effect, index}));
                Some(selectables)
            }
            None => None
        }
    }

    fn add_selectable(&mut self, selectable: &Selectable) {
        match selectable {
            Selectable::Effect { effect, index } => {
                if self.effects.is_empty() || !self.single {
                    self.effects.push(*index)
                }
                else {
                    self.effects[0] = *index
                }
            }
            Selectable::Note { note, index } => {
                if self.notes.is_empty() || !self.single {
                    self.notes.push(*index)
                }
                else {
                    self.notes[0] = *index
                }
            }
        }
    }
}
