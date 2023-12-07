use crate::{Effect, Event, Music, Note};
use serde::{Deserialize, Serialize};

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
        music.get_selected_track().map(|track| {
            (
                self.notes.iter().map(|i| &track.notes[*i]).collect(),
                self.effects.iter().map(|i| &track.effects[*i]).collect(),
            )
        })
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
        match Self::get_music_events(music) {
            Some(music_selectables) => {
                // The track has a program but no notes or events.
                if music_selectables.is_empty() {
                    false
                } else {
                    match self.get_events(music) {
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
                                    self.add_event(s);
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
        match Self::get_music_events(music) {
            Some(music_selectables) => {
                // The track has a program but no notes or events.
                if music_selectables.is_empty() {
                    false
                } else {
                    match self.get_events(music) {
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
                                    self.add_event(s);
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
        match self.get_events(music) {
            Some(events) => {
                // Try to remove the first selected note or event.
                if !events.is_empty() {
                    match events[0] {
                        Event::Effect { effect: _, index } => self.effects.remove(index),
                        Event::Note { note: _, index } => self.notes.remove(index),
                    };
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    pub fn deselect_last(&mut self, music: &Music) -> bool {
        match self.get_events(music) {
            Some(events) => {
                // Try to remove the first selected note or event.
                if !events.is_empty() {
                    match events.last().unwrap() {
                        Event::Effect { effect: _, index } => self.effects.remove(*index),
                        Event::Note { note: _, index } => self.notes.remove(*index),
                    };
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    /// Select all notes and effects.
    pub fn select_all(&mut self, music: &Music) {
        if let Some(music_events) = Self::get_music_events(music) {
            self.single = false;
            self.notes.clear();
            self.effects.clear();
            for s in music_events.iter() {
                self.add_event(s)
            }
        }
    }

    /// Returns the start and end time of the selection in PPQ.
    pub fn get_dt(&self, music: &Music) -> Option<(u64, u64)> {
        match self.get_events(music) {
            Some(events) => match events.iter().map(|s| s.get_start_time()).min() {
                Some(min) => events
                    .iter()
                    .map(|s| s.get_end_time())
                    .max()
                    .map(|max| (min, max)),
                None => None,
            },
            None => None,
        }
    }

    /// Returns a vec of selected notes and effects.
    pub fn get_events<'track>(&self, music: &'track Music) -> Option<Vec<Event<'track>>> {
        match self.get_selection(music) {
            Some((notes, effects)) => {
                let mut events = notes
                    .iter()
                    .enumerate()
                    .map(|(index, note)| Event::Note { note, index })
                    .collect::<Vec<Event>>();
                events.extend(
                    effects
                        .iter()
                        .enumerate()
                        .map(|(index, effect)| Event::Effect { effect, index }),
                );
                // Sort the selectables by start time.
                events.sort_by_key(|e| e.get_start_time());
                Some(events)
            }
            None => None,
        }
    }

    /// Convert the selected track's notes and effects into a vec of selectables.
    fn get_music_events(music: &Music) -> Option<Vec<Event<'_>>> {
        match music.get_selected_track() {
            Some(track) => {
                let mut events = track
                    .notes
                    .iter()
                    .enumerate()
                    .map(|(index, note)| Event::Note { note, index })
                    .collect::<Vec<Event>>();
                events.extend(
                    track
                        .effects
                        .iter()
                        .enumerate()
                        .map(|(index, effect)| Event::Effect { effect, index }),
                );
                events.sort_by_key(|e| e.get_start_time());
                Some(events)
            }
            None => None,
        }
    }

    /// Add a note or effect to the selection.
    fn add_event(&mut self, selectable: &Event) {
        match selectable {
            Event::Effect { effect: _, index } => {
                if self.effects.is_empty() || !self.single {
                    self.effects.push(*index)
                } else {
                    self.effects[0] = *index;
                    self.notes.clear();
                }
            }
            Event::Note { note: _, index } => {
                if self.notes.is_empty() || !self.single {
                    self.notes.push(*index)
                } else {
                    self.notes[0] = *index;
                    self.effects.clear();
                }
            }
        }
    }
}
