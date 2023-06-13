use super::{get_no_selection_status_tts, PianoRollSubPanel};
use crate::panel::*;
use common::time::Time;
use common::{MidiTrack, Note, SelectMode};

/// Select notes.
pub(super) struct Select {}

impl Select {
    /// Returns the index of the note closest (and before) the cursor.
    fn get_note_index_closest_to_before_cursor(notes: &[Note], time: &Time) -> Option<usize> {
        notes
            .iter()
            .enumerate()
            .filter(|n| n.1.start < time.cursor)
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|max| max.0)
    }

    /// Returns the index of the note closest (and after) the cursor.
    fn get_note_index_closest_to_after_cursor(notes: &[Note], time: &Time) -> Option<usize> {
        notes
            .iter()
            .enumerate()
            .filter(|n| n.1.start >= time.cursor)
            .min_by(|a, b| a.1.cmp(b.1))
            .map(|max| max.0)
    }

    /// Returns the first note in a selection defined by `indices`.
    fn get_first_selected_note<'a>(
        track: &'a MidiTrack,
        indices: &[usize],
    ) -> Option<(usize, &'a Note)> {
        track
            .notes
            .iter()
            .enumerate()
            .filter(|n| indices.contains(&n.0))
            .min_by(|a, b| a.1.cmp(b.1))
    }

    /// Returns the last note in a selection defined by `indices`.
    fn get_last_selected_note<'a>(
        track: &'a MidiTrack,
        indices: &[usize],
    ) -> Option<(usize, &'a Note)> {
        track
            .notes
            .iter()
            .enumerate()
            .filter(|n| indices.contains(&n.0))
            .max_by(|a, b| a.1.cmp(b.1))
    }
}

impl Panel for Select {
    fn update(
        &mut self,
        state: &mut State,
        _: &mut Conn,
        input: &Input,
        _: &mut TTS,
        _: &Text,
        _: &mut PathsState,
    ) -> Option<Snapshot> {
        match state.music.get_selected_track() {
            None => None,
            Some(track) => {
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
                    Some(Snapshot::from_states(s0, state))
                }
                // Move the selection start leftwards.
                else if input.happened(&InputEvent::SelectStartLeft) {
                    let s0 = state.clone();
                    match &mut state.select_mode {
                        SelectMode::Single(index) => match index {
                            // Get the prior note.
                            Some(index) => {
                                let note = &track.notes[*index];
                                if let Some(prior_note) = track
                                    .notes
                                    .iter()
                                    .enumerate()
                                    .filter(|n| n.1.lt(note))
                                    .max_by(|a, b| a.1.cmp(b.1))
                                {
                                    *index = prior_note.0;
                                    return Some(Snapshot::from_states(s0, state));
                                }
                            }
                            // Select the note closest to the cursor.
                            None => {
                                if let Some(index) = Select::get_note_index_closest_to_before_cursor(
                                    &track.notes,
                                    &state.time,
                                ) {
                                    state.select_mode = SelectMode::Single(Some(index));
                                    return Some(Snapshot::from_states(s0, state));
                                }
                            }
                        },
                        // Are there selected indices?
                        SelectMode::Many(indices) => match indices {
                            // Is there a max selected index?
                            Some(indices) => {
                                // There is a first selected note.
                                if let Some(first_selected_note) =
                                    Select::get_first_selected_note(track, indices)
                                {
                                    // There is a prior note.
                                    if let Some(prior_note) = track
                                        .notes
                                        .iter()
                                        .enumerate()
                                        .filter(|n| n.1.lt(first_selected_note.1))
                                        .max_by(|a, b| a.1.cmp(b.1))
                                    {
                                        // Add the prior note.
                                        indices.push(prior_note.0);
                                        return Some(Snapshot::from_states(s0, state));
                                    }
                                }
                            }
                            // Select the note closest to the cursor.
                            None => {
                                // Is there a note near the cursor?
                                if let Some(index) = Select::get_note_index_closest_to_before_cursor(
                                    &track.notes,
                                    &state.time,
                                ) {
                                    state.select_mode = SelectMode::Many(Some(vec![index]));
                                    return Some(Snapshot::from_states(s0, state));
                                }
                            }
                        },
                    }
                    return None;
                }
                // Move the selection start rightwards.
                else if input.happened(&InputEvent::SelectStartRight) {
                    let s0 = state.clone();
                    match &mut state.select_mode {
                        SelectMode::Single(index) => match index {
                            Some(index) => {
                                let note = &track.notes[*index];
                                // Get the next note.
                                if let Some(next_note) = track
                                    .notes
                                    .iter()
                                    .enumerate()
                                    .filter(|n| n.1.gt(note))
                                    .min_by(|a, b| a.1.cmp(b.1))
                                {
                                    *index = next_note.0;
                                    return Some(Snapshot::from_states(s0, state));
                                }
                            }
                            // Select the note closest to the cursor.
                            None => {
                                if let Some(index) = Select::get_note_index_closest_to_before_cursor(
                                    &track.notes,
                                    &state.time,
                                ) {
                                    state.select_mode = SelectMode::Single(Some(index));
                                    return Some(Snapshot::from_states(s0, state));
                                }
                            }
                        },
                        // Remove the first note.
                        SelectMode::Many(indices) => match indices {
                            Some(indices) => {
                                if indices.len() <= 1 {
                                    return None;
                                }
                                // There is a first selected note.
                                if let Some(first_selected_note) =
                                    Select::get_first_selected_note(track, indices)
                                {
                                    // Remove the note.
                                    indices.retain(|n| *n != first_selected_note.0);
                                    return Some(Snapshot::from_states(s0, state));
                                }
                            }
                            // Select the note closest to the cursor.
                            None => {
                                if let Some(index) = Select::get_note_index_closest_to_before_cursor(
                                    &track.notes,
                                    &state.time,
                                ) {
                                    state.select_mode = SelectMode::Many(Some(vec![index]));
                                    return Some(Snapshot::from_states(s0, state));
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
                    return Some(Snapshot::from_states(s0, state));
                }
                // Select all.
                else if input.happened(&InputEvent::SelectAll) {
                    let indices = track.notes.iter().enumerate().map(|n| n.0).collect();
                    let s0 = state.clone();
                    state.select_mode = SelectMode::Many(Some(indices));
                    return Some(Snapshot::from_states(s0, state));
                }
                // Adjust the end of the selection.
                else if let SelectMode::Many(indices) = &state.select_mode {
                    // Remove a note at the end.
                    if input.happened(&InputEvent::SelectEndLeft) {
                        match indices {
                            Some(indices) => {
                                if indices.len() <= 1 {
                                    return None;
                                }
                                match Select::get_last_selected_note(track, indices) {
                                    Some(last_selected_note) => {
                                        let s0 = state.clone();
                                        // Remove the note.
                                        let mut indices = indices.clone();
                                        indices.retain(|n| *n != last_selected_note.0);
                                        state.select_mode = SelectMode::Many(Some(indices));
                                        return Some(Snapshot::from_states(s0, state));
                                    }
                                    None => return None,
                                }
                            }
                            None => {
                                // Select the note closest to the cursor.
                                if let Some(index) = Select::get_note_index_closest_to_after_cursor(
                                    &track.notes,
                                    &state.time,
                                ) {
                                    let s0 = state.clone();
                                    state.select_mode = SelectMode::Many(Some(vec![index]));
                                    return Some(Snapshot::from_states(s0, state));
                                }
                                None
                            }
                        }
                    }
                    // Add a note at the end.
                    else if input.happened(&InputEvent::SelectEndRight) {
                        match indices {
                            Some(indices) => {
                                match Select::get_last_selected_note(track, indices) {
                                    // Get the next note.
                                    Some(last_selected_note) => match track
                                        .notes
                                        .iter()
                                        .enumerate()
                                        .filter(|n| n.1.gt(last_selected_note.1))
                                        .min_by(|a, b| a.1.cmp(b.1))
                                    {
                                        Some(next_note) => {
                                            let s0 = state.clone();
                                            // Remove the note.
                                            let mut indices = indices.clone();
                                            indices.push(next_note.0);
                                            state.select_mode = SelectMode::Many(Some(indices));
                                            return Some(Snapshot::from_states(s0, state));
                                        }
                                        None => return None,
                                    },
                                    None => return None,
                                }
                            }
                            None => {
                                if let Some(index) = Select::get_note_index_closest_to_after_cursor(
                                    &track.notes,
                                    &state.time,
                                ) {
                                    let s0 = state.clone();
                                    state.select_mode = SelectMode::Many(Some(vec![index]));
                                    return Some(Snapshot::from_states(s0, state));
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
    }
}

impl PianoRollSubPanel for Select {
    fn get_status_tts(&self, state: &State, text: &Text) -> String {
        match &state.select_mode {
            SelectMode::Single(index) => match index {
                Some(index) => match state.select_mode.get_notes(&state.music) {
                    Some(notes) => {
                        let note = notes[*index];
                        text.get_with_values(
                            "PIANO_ROLL_PANEL_STATUS_TTS_SELECTED_SINGLE",
                            &[&note.note.to_string(), &text.get_fraction_tts(&note.start)],
                        )
                    }
                    None => text.get_error("The selected note doesn't exist."),
                },
                None => get_no_selection_status_tts(text),
            },
            SelectMode::Many(_) => match state.select_mode.get_notes(&state.music) {
                Some(notes) => match notes.iter().map(|n| n.start).min() {
                    Some(min) => match notes.iter().map(|n| n.start + n.duration).max() {
                        Some(max) => text.get_with_values(
                            "PIANO_ROLL_PANEL_STATUS_TTS_SELECTED_MANY",
                            &[&text.get_fraction_tts(&min), &text.get_fraction_tts(&max)],
                        ),
                        None => text.get_error("There is no end time to the selection."),
                    },
                    None => text.get_error("There is no start time to the selection."),
                },
                None => text.get_error("The selected notes don't exist."),
            },
        }
    }

    fn get_input_tts(&self, state: &State, input: &Input, text: &Text) -> String {
        let (mut s, selected) = match &state.select_mode {
            SelectMode::Single(index) => match index {
                Some(_) => (
                    get_tooltip(
                        "PIANO_ROLL_PANEL_INPUT_TTS_SELECT_SINGLE",
                        &[InputEvent::SelectStartLeft, InputEvent::SelectStartRight],
                        input,
                        text,
                    ),
                    true,
                ),
                None => (get_no_selection_status_tts(text), false),
            },
            SelectMode::Many(indices) => match indices {
                Some(_) => (
                    get_tooltip(
                        "PIANO_ROLL_PANEL_INPUT_TTS_SELECT_MANY",
                        &[
                            InputEvent::SelectStartLeft,
                            InputEvent::SelectStartRight,
                            InputEvent::SelectEndLeft,
                            InputEvent::SelectEndRight,
                            InputEvent::SelectNone,
                            InputEvent::SelectAll,
                            InputEvent::PianoRollCycleMode,
                        ],
                        input,
                        text,
                    ),
                    true,
                ),
                None => (get_no_selection_status_tts(text), false),
            },
        };
        if state.select_mode.get_note_indices().is_some() {
            s.push(' ');
            s.push_str(&get_tooltip(
                "PIANO_ROLL_PANEL_INPUT_TTS_SELECT_ALL",
                &[InputEvent::SelectAll],
                input,
                text,
            ));
        }
        if selected {
            s.push(' ');
            s.push_str(&get_tooltip(
                "PIANO_ROLL_PANEL_INPUT_TTS_DESELECT",
                &[InputEvent::SelectNone],
                input,
                text,
            ));
        }
        s.push(' ');
        let cycle_key = match state.select_mode {
            SelectMode::Single(_) => "PIANO_ROLL_PANEL_INPUT_TTS_SELECT_CYCLE_TO_MANY",
            SelectMode::Many(_) => "PIANO_ROLL_PANEL_INPUT_TTS_SELECT_CYCLE_TO_SINGLE",
        };
        s.push_str(&get_tooltip(
            cycle_key,
            &[InputEvent::PianoRollCycleMode],
            input,
            text,
        ));
        s
    }
}
