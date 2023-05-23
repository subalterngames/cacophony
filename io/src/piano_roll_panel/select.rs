use super::EditModeDeltas;
use crate::get_tooltip_with_values;
use crate::panel::*;
use common::ini::Ini;
use common::time::Time;
use common::Zero;
use common::{Fraction, Note, SelectMode, EDIT_MODES, MAX_NOTE, MAX_VOLUME, MIN_NOTE};

/// Select notes.
pub(super) struct Select {}

impl Select {
    /// Returns the index of the note closest (and before) the cursor.
    fn get_note_index_closest_to_cursor(notes: &[Note], time: &Time) -> Option<usize> {
        notes
            .iter()
            .enumerate()
            .filter(|n| n.1.start + n.1.duration < time.cursor)
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|max| max.0)
    }
}

impl Panel for Select {
    fn update(
        &mut self,
        state: &mut State,
        conn: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &Text,
    ) -> Option<UndoRedoState> {
        // Cycle the select mode.
        if input.happened(&InputEvent::PianoRollCycleMode) {
            let s0 = state.clone();
            let mode = state.select_mode.clone();
            state.select_mode = match mode {
                SelectMode::Single(index) => match index {
                    Some(index) => SelectMode::Many(Some(vec![index])),
                    None => SelectMode::Many(None),
                },
                SelectMode::Many(indices) => match indices {
                    Some(indices) => match indices.is_empty() {
                        true => SelectMode::Single(None),
                        false => SelectMode::Single(Some(indices[0])),
                    },
                    None => SelectMode::Single(None),
                },
            };
            Some(UndoRedoState::from((s0, state)))
        } else if input.happened(&InputEvent::SubPanelTTS) {
            panic!("TODO")
        }
        // Move the selection left.
        else if input.happened(&InputEvent::SelectStartLeft) {
            let s0 = state.clone();
            let mode = state.select_mode.clone();
            match mode {
                SelectMode::Single(index) => match index {
                    Some(index) => {
                        if index > 0 {
                            state.select_mode = SelectMode::Single(Some(index - 1));
                            return Some(UndoRedoState::from((s0, state)));
                        }
                    }
                    None => {
                        if let Some(track) = state.music.get_selected_track() {
                            if let Some(index) =
                                Select::get_note_index_closest_to_cursor(&track.notes, &state.time)
                            {
                                state.select_mode = SelectMode::Single(Some(index));
                                return Some(UndoRedoState::from((s0, state)));
                            }
                        }
                    }
                },
                // Are there selected indices?
                SelectMode::Many(indices) => match indices {
                    // Is there a max selected index?
                    Some(indices) => {
                        if let Some(max) = indices.iter().max() {
                            if let Some(track) = state.music.get_selected_track() {
                                // Does the track have a note after the max note?
                                if let Some(max_track) = track
                                    .notes
                                    .iter()
                                    .enumerate()
                                    .filter(|n| n.1.gt(&track.notes[*max]))
                                    .max_by(|a, b| a.1.cmp(b.1))
                                {
                                    let mut indices = indices.clone();
                                    indices.push(max_track.0);
                                    state.select_mode = SelectMode::Many(Some(indices));
                                    return Some(UndoRedoState::from((s0, state)));
                                }
                            }
                        }
                    }
                    // Is there a track?
                    None => {
                        if let Some(track) = state.music.get_selected_track() {
                            // Is there a note near the cursor?
                            if let Some(index) =
                                Select::get_note_index_closest_to_cursor(&track.notes, &state.time)
                            {
                                state.select_mode = SelectMode::Many(Some(vec![index]));
                                return Some(UndoRedoState::from((s0, state)));
                            }
                        }
                    }
                },
            }
            return None;
        } else {
            None
        }
    }
}
