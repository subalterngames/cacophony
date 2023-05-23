use crate::get_tooltip_with_values;
use crate::panel::*;
use common::time::Time;
use common::{Note, SelectMode};

/// Select notes.
pub(super) struct Select {}

impl Select {
    /// Returns the index of the note closest (and before) the cursor.
    fn get_note_index_closest_to_before_cursor(notes: &[Note], time: &Time) -> Option<usize> {
        notes
            .iter()
            .enumerate()
            .filter(|n| n.1.start + n.1.duration < time.cursor)
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|max| max.0)
    }

    /// Returns the index of the note closest (and after) the cursor.
    fn get_note_index_closest_to_after_cursor(notes: &[Note], time: &Time) -> Option<usize> {
        notes
            .iter()
            .enumerate()
            .filter(|n| n.1.start + n.1.duration >= time.cursor)
            .min_by(|a, b| a.1.cmp(b.1))
            .map(|max| max.0)
    }
}

impl Panel for Select {
    fn update(
        &mut self,
        state: &mut State,
        _: &mut Conn,
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
            let mode = state.select_mode.clone();
            let s = match mode {
                SelectMode::Single(index) => {
                    let selection = match index {
                        Some(index) => match state.music.get_selected_track() {
                            Some(track) => {
                                let note = &track.notes[index];
                                let time = text.get_time(&note.start, state.music.bpm);
                                text.get_with_values(
                                    "SELECT_TTS_SINGLE_SELECTION",
                                    &[&note.note.to_string(), &note.velocity.to_string(), &time],
                                )
                            }
                            None => text.get_with_values("ERROR", &["there is no track!"]),
                        },
                        None => text.get("SELECT_TTS_SINGLE_NO_SELECTION"),
                    };
                    get_tooltip_with_values(
                        "SELECT_TTS_SINGLE",
                        &[
                            InputEvent::SelectStartLeft,
                            InputEvent::SelectStartRight,
                            InputEvent::SelectNone,
                            InputEvent::SelectAll,
                            InputEvent::PianoRollCycleMode,
                        ],
                        &[&selection],
                        input,
                        text,
                    )
                }
                SelectMode::Many(indices) => {
                    let selection = match indices {
                        Some(indices) => match state.music.get_selected_track() {
                            Some(track) => {
                                let notes: Vec<&Note> = track
                                    .notes
                                    .iter()
                                    .enumerate()
                                    .filter(|n| indices.contains(&n.0))
                                    .map(|n| n.1)
                                    .collect();
                                match notes.iter().min() {
                                    Some(min) => match notes.iter().max() {
                                        Some(max) => {
                                            let t0 = text.get_time(&min.start, state.music.bpm);
                                            let t1 = text.get_time(&max.start, state.music.bpm);
                                            text.get_with_values(
                                                "SELECT_TTS_MANY_SELECTION",
                                                &[&t0, &t1],
                                            )
                                        }
                                        None => text
                                            .get_with_values("ERROR", &["there is no end time!"]),
                                    },
                                    None => {
                                        text.get_with_values("ERROR", &["there is no start time!"])
                                    }
                                }
                            }
                            None => text.get_with_values("ERROR", &["there is no track!"]),
                        },
                        None => text.get("SELECT_TTS_MANY_NO_SELECTION"),
                    };
                    get_tooltip_with_values(
                        "SELECT_TTS_MANY",
                        &[
                            InputEvent::SelectStartLeft,
                            InputEvent::SelectStartRight,
                            InputEvent::SelectEndLeft,
                            InputEvent::SelectEndRight,
                            InputEvent::SelectNone,
                            InputEvent::SelectAll,
                            InputEvent::PianoRollCycleMode,
                        ],
                        &[&selection],
                        input,
                        text,
                    )
                }
            };
            tts.say(&s);
            None
        }
        // Move the selection start leftwards.
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
                            if let Some(index) = Select::get_note_index_closest_to_before_cursor(
                                &track.notes,
                                &state.time,
                            ) {
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
                            if let Some(index) = Select::get_note_index_closest_to_before_cursor(
                                &track.notes,
                                &state.time,
                            ) {
                                state.select_mode = SelectMode::Many(Some(vec![index]));
                                return Some(UndoRedoState::from((s0, state)));
                            }
                        }
                    }
                },
            }
            return None;
        }
        // Move the selection start rightwards.
        else if input.happened(&InputEvent::SelectStartRight) {
            let s0 = state.clone();
            let mode = state.select_mode.clone();
            match mode {
                SelectMode::Single(index) => match index {
                    Some(index) => {
                        if let Some(track) = state.music.get_selected_track() {
                            if let Some(max) = track
                                .notes
                                .iter()
                                .enumerate()
                                .filter(|n| n.1.gt(&track.notes[index]))
                                .max_by(|a, b| a.1.cmp(b.1))
                            {
                                state.select_mode = SelectMode::Single(Some(max.0));
                                return Some(UndoRedoState::from((s0, state)));
                            }
                        }
                    }
                    None => {
                        if let Some(track) = state.music.get_selected_track() {
                            if let Some(index) = Select::get_note_index_closest_to_before_cursor(
                                &track.notes,
                                &state.time,
                            ) {
                                state.select_mode = SelectMode::Single(Some(index));
                                return Some(UndoRedoState::from((s0, state)));
                            }
                        }
                    }
                },
                SelectMode::Many(indices) => match indices {
                    Some(indices) => match indices.len() > 1 {
                        // Remove an index.
                        true => {
                            let indices = indices.as_slice()[0..indices.len() - 1].to_vec();
                            state.select_mode = SelectMode::Many(Some(indices));
                            return Some(UndoRedoState::from((s0, state)));
                        }
                        // There are no indices.
                        false => {
                            state.select_mode = SelectMode::Many(None);
                            return Some(UndoRedoState::from((s0, state)));
                        }
                    },
                    None => {
                        if let Some(track) = state.music.get_selected_track() {
                            if let Some(index) = Select::get_note_index_closest_to_before_cursor(
                                &track.notes,
                                &state.time,
                            ) {
                                state.select_mode = SelectMode::Many(Some(vec![index]));
                                return Some(UndoRedoState::from((s0, state)));
                            }
                        }
                    }
                },
            }
            return None;
        }
        // Deselect.
        else if input.happened(&InputEvent::SelectNone) {
            let s0 = state.clone();
            let mode = state.select_mode.clone();
            state.select_mode = match mode {
                SelectMode::Single(_) => SelectMode::Single(None),
                SelectMode::Many(_) => SelectMode::Many(None),
            };
            Some(UndoRedoState::from((s0, state)))
        }
        // Select all.
        else if input.happened(&InputEvent::SelectAll) {
            match state.music.get_selected_track() {
                Some(track) => {
                    let indices = track.notes.iter().enumerate().map(|n| n.0).collect();
                    let s0 = state.clone();
                    state.select_mode = SelectMode::Many(Some(indices));
                    Some(UndoRedoState::from((s0, state)))
                }
                None => None,
            }
        }
        // Adjust the end of the selection.
        else if let SelectMode::Many(indices) = &state.select_mode {
            // Remove a note at the end.
            if input.happened(&InputEvent::SelectEndLeft) {
                match indices {
                    Some(indices) => match indices.len() > 1 {
                        true => {
                            let s0 = state.clone();
                            let indices = indices.as_slice()[0..indices.len() - 1].to_vec();
                            state.select_mode = SelectMode::Many(Some(indices));
                            Some(UndoRedoState::from((s0, state)))
                        }
                        false => {
                            let s0 = state.clone();
                            state.select_mode = SelectMode::Many(None);
                            Some(UndoRedoState::from((s0, state)))
                        }
                    },
                    None => {
                        if let Some(track) = state.music.get_selected_track() {
                            if let Some(index) = Select::get_note_index_closest_to_after_cursor(
                                &track.notes,
                                &state.time,
                            ) {
                                let s0 = state.clone();
                                state.select_mode = SelectMode::Many(Some(vec![index]));
                                return Some(UndoRedoState::from((s0, state)));
                            }
                        }
                        None
                    }
                }
            }
            // Add a note at the end.
            else if input.happened(&InputEvent::SelectEndRight) {
                match indices {
                    Some(indices) => match indices.iter().max() {
                        Some(max) => {
                            if let Some(track) = state.music.get_selected_track() {
                                if let Some(max) = track
                                    .notes
                                    .iter()
                                    .enumerate()
                                    .filter(|n| n.1.gt(&track.notes[*max]))
                                    .max_by(|a, b| a.1.cmp(b.1))
                                {
                                    let mut indices = indices.clone();
                                    indices.push(max.0);
                                    let s0 = state.clone();
                                    state.select_mode = SelectMode::Many(Some(indices));
                                    return Some(UndoRedoState::from((s0, state)));
                                }
                            }
                        }
                        None => {
                            if let Some(track) = state.music.get_selected_track() {
                                if let Some(index) = Select::get_note_index_closest_to_after_cursor(
                                    &track.notes,
                                    &state.time,
                                ) {
                                    let s0 = state.clone();
                                    state.select_mode = SelectMode::Many(Some(vec![index]));
                                    return Some(UndoRedoState::from((s0, state)));
                                }
                            }
                        }
                    },
                    None => {
                        if let Some(track) = state.music.get_selected_track() {
                            if let Some(index) = Select::get_note_index_closest_to_after_cursor(
                                &track.notes,
                                &state.time,
                            ) {
                                let s0 = state.clone();
                                state.select_mode = SelectMode::Many(Some(vec![index]));
                                return Some(UndoRedoState::from((s0, state)));
                            }
                        }
                    }
                }
                None
            } else {
                None
            }
        } else {
            None
        }
    }
}
