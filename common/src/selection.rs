use crate::{Effect, Music, Note};
use serde::{Deserialize, Serialize};

enum Selectable<'track> {
    Effect {
        effect: &'track Effect,
        index: usize,
    },
    Note {
        note: &'track Note,
        index: usize,
    },
}

impl<'track> Selectable<'track> {
    fn get_start_time(&self) -> u64 {
        match self {
            Self::Effect { effect, index: _ } => effect.time,
            Self::Note { note, index: _ } => note.start,
        }
    }

    fn get_end_time(&self) -> u64 {
        match self {
            Self::Effect { effect, index: _ } => effect.time,
            Self::Note { note, index: _ } => note.end,
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
    pub fn get_selection<'a>(&self, music: &'a Music) -> Option<(Vec<&'a Note>, Vec<&'a Effect>)> {
        match music.get_selected_track() {
            None => None,
            Some(track) => Some((
                self.notes.iter().map(|i| &track.notes[*i]).collect(),
                self.effects.iter().map(|i| &track.effects[*i]).collect(),
            )),
        }
    }

    /// Returns mutable selected notes and effects.
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
                    .filter(|(i, _)| self.notes.contains(i))
                    .map(|(_, note)| note)
                    .collect(),
                track
                    .effects
                    .iter_mut()
                    .enumerate()
                    .filter(|(i, _)| self.effects.contains(i))
                    .map(|(_, effect)| effect)
                    .collect(),
            )),
        }
    }

    /// Select the note or event prior to the current selection.
    pub fn select_previous(&mut self, music: &Music, cursor_time: u64) -> bool {
        match Self::get_music_selectables(music) {
            Some(music_selectables) => {
                // The track has a program but no notes or events.
                if music_selectables.is_empty() {
                    false
                } else {
                    match self.get_selectables(music) {
                        Some(selectables) => {
                            // The selection is empty.
                            let s = if selectables.is_empty() {
                                music_selectables
                                    .iter()
                                    .filter(|s| s.get_start_time() < cursor_time)
                                    .last()
                            } else {
                                music_selectables
                                    .iter()
                                    .filter(|s| s.get_end_time() < selectables[0].get_start_time())
                                    .last()
                            };
                            match s {
                                Some(s) => {
                                    self.add_selectable(s);
                                    true
                                }
                                None => false,
                            }
                        }
                        None => false,
                    }
                }
            }
            None => false,
        }
    }

    /// Select the note or event after to the current selection.
    pub fn select_next(&mut self, music: &Music, cursor_time: u64) -> bool {
        match Self::get_music_selectables(music) {
            Some(music_selectables) => {
                // The track has a program but no notes or events.
                if music_selectables.is_empty() {
                    false
                } else {
                    match self.get_selectables(music) {
                        Some(selectables) => {
                            // The selection is empty.
                            let s = if selectables.is_empty() {
                                music_selectables
                                    .iter()
                                    .filter(|s| s.get_end_time() > cursor_time)
                                    .last()
                            } else {
                                music_selectables
                                    .iter()
                                    .filter(|s| {
                                        s.get_start_time()
                                            > selectables[selectables.len() - 1].get_end_time()
                                    })
                                    .last()
                            };
                            match s {
                                Some(s) => {
                                    self.add_selectable(s);
                                    true
                                }
                                None => false,
                            }
                        }
                        None => false,
                    }
                }
            }
            None => false,
        }
    }

    pub fn deselect_first(&mut self, music: &Music) -> bool {
        match self.get_selectables(music) {
            Some(selectables) => {
                // Try to remove the first selected note or event.
                if !selectables.is_empty() {
                    match selectables[0] {
                        Selectable::Effect { effect: _, index } => self.effects.remove(index),
                        Selectable::Note { note: _, index } => self.notes.remove(index),
                    };
                    true
                }
                else {
                    false
                }
            }
            None => false,
        }
    }

    pub fn deselect_last(&mut self, music: &Music) -> bool {
        match self.get_selectables(music) {
            Some(selectables) => {
                // Try to remove the first selected note or event.
                if !selectables.is_empty() {
                    match selectables.last().unwrap() {
                        Selectable::Effect { effect: _, index } => self.effects.remove(*index),
                        Selectable::Note { note: _, index } => self.notes.remove(*index),
                    };
                    true
                }
                else {
                    false
                }
            }
            None => false,
        }
    }

    /// Select all notes and effects.
    pub fn select_all(&mut self, music: &Music) {
        if let Some(music_selectables) = Self::get_music_selectables(music) {
            self.single = false;
            self.notes.clear();
            self.effects.clear();
            for s in music_selectables.iter() {
                self.add_selectable(s)
            }
        }
    }

    /// Returns a vec of selected notes and effects.
    fn get_selectables<'track>(&self, music: &'track Music) -> Option<Vec<Selectable<'track>>> {
        match self.get_selection(music) {
            Some((notes, effects)) => {
                let mut selectables = notes
                    .iter()
                    .enumerate()
                    .map(|(index, note)| Selectable::Note { note, index })
                    .collect::<Vec<Selectable>>();
                selectables.extend(
                    effects
                        .iter()
                        .enumerate()
                        .map(|(index, effect)| Selectable::Effect { effect, index }),
                );
                // Sort the selectables by start time.
                selectables.sort_by(|a, b| a.get_start_time().cmp(&b.get_start_time()));
                Some(selectables)
            }
            None => None,
        }
    }

    /// Convert the selected track's notes and effects into a vec of selectables.
    fn get_music_selectables(music: &Music) -> Option<Vec<Selectable<'_>>> {
        match music.get_selected_track() {
            Some(track) => {
                let mut selectables = track
                    .notes
                    .iter()
                    .enumerate()
                    .map(|(index, note)| Selectable::Note { note, index })
                    .collect::<Vec<Selectable>>();
                selectables.extend(
                    track
                        .effects
                        .iter()
                        .enumerate()
                        .map(|(index, effect)| Selectable::Effect { effect, index }),
                );
                selectables.sort_by(|a, b| a.get_start_time().cmp(&b.get_start_time()));
                Some(selectables)
            }
            None => None,
        }
    }

    /// Add a note or effect to the selection.
    fn add_selectable(&mut self, selectable: &Selectable) {
        match selectable {
            Selectable::Effect { effect: _, index } => {
                if self.effects.is_empty() || !self.single {
                    self.effects.push(*index)
                } else {
                    self.effects[0] = *index
                }
            }
            Selectable::Note { note: _, index } => {
                if self.notes.is_empty() || !self.single {
                    self.notes.push(*index)
                } else {
                    self.notes[0] = *index
                }
            }
        }
    }
}
