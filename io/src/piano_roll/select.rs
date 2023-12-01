use super::{get_no_selection_status_tts, PianoRollSubPanel};
use crate::panel::*;
use common::time::Time;
use common::{MidiTrack, Note, Selection};

/// Select notes.
#[derive(Default)]
pub(super) struct Select {
    tooltips: Tooltips,
}

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
                    state.selection.single = !state.selection.single;
                    if state.selection.single {
                        if state.selection.notes.len() > 1 {
                            state.selection.notes.drain(1..state.selection.notes.len());
                        }
                        if state.selection.effects.len() > 1 {
                            state
                                .selection
                                .effects
                                .drain(1..state.selection.notes.len());
                        }
                    }
                    Some(Snapshot::from_states(s0, state))
                }
                // Move the selection start leftwards.
                else if input.happened(&InputEvent::SelectStartLeft) {
                    let s0 = state.clone();
                    // Try to select a previous note or event.
                    if state
                        .selection
                        .select_previous(&state.music, state.time.playback)
                    {
                        Some(Snapshot::from_states(s0, state))
                    } else {
                        None
                    }
                }
                // Move the selection start rightwards.
                else if input.happened(&InputEvent::SelectStartRight) {
                    let s0 = state.clone();
                    if state
                        .selection
                        .select_next(&state.music, state.time.playback)
                    {
                        Some(Snapshot::from_states(s0, state))
                    } else {
                        None
                    }
                }
                // Deselect.
                else if input.happened(&InputEvent::SelectNone) {
                    let s0 = state.clone();
                    state.selection.deselect();
                    Some(Snapshot::from_states(s0, state))
                }
                // Select all.
                else if input.happened(&InputEvent::SelectAll) {
                    let s0 = state.clone();
                    state.selection.select_all(&state.music);
                    Some(Snapshot::from_states(s0, state))
                }
                // Adjust the end of the selection.
                else if !state.selection.single {
                    // Remove a note at the end.
                    if input.happened(&InputEvent::SelectEndLeft) {
                        let s0 = state.clone();
                        if state.selection.deselect_first(&state.music) {
                            Some(Snapshot::from_states(s0, state))
                        }
                        else {
                            None
                        }
                    }
                    // Add a note at the end.
                    else if input.happened(&InputEvent::SelectEndRight) {
                        let s0 = state.clone();
                        if state.selection.deselect_last(&state.music) {
                            Some(Snapshot::from_states(s0, state))
                        }
                        else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    fn on_disable_abc123(&mut self, _: &mut State, _: &mut Conn) {}

    fn update_abc123(
        &mut self,
        _: &mut State,
        _: &Input,
        _: &mut Conn,
    ) -> (Option<Snapshot>, bool) {
        (None, false)
    }

    fn allow_alphanumeric_input(&self, _: &State, _: &Conn) -> bool {
        false
    }

    fn allow_play_music(&self) -> bool {
        true
    }
}

impl PianoRollSubPanel for Select {
    fn get_status_tts(&mut self, state: &State, text: &Text) -> Vec<TtsString> {
        let tts_string = if state.selection.single {
            
        }
        let tts_string = match &state.select_mode {
            SelectMode::Single(index) => match index {
                Some(index) => match state.select_mode.get_notes(&state.music) {
                    Some(notes) => {
                        let note = notes[*index];
                        TtsString::from(text.get_with_values(
                            "PIANO_ROLL_PANEL_STATUS_TTS_SELECTED_SINGLE",
                            &[&note.note.to_string(), &text.get_ppq_tts(&note.start)],
                        ))
                    }
                    None => TtsString::from(text.get_error("The selected note doesn't exist.")),
                },
                None => get_no_selection_status_tts(text),
            },
            SelectMode::Many(_) => match state.select_mode.get_notes(&state.music) {
                Some(notes) => match notes.iter().map(|n| n.start).min() {
                    Some(min) => match notes.iter().map(|n| n.end).max() {
                        Some(max) => TtsString::from(text.get_with_values(
                            "PIANO_ROLL_PANEL_STATUS_TTS_SELECTED_MANY",
                            &[&text.get_ppq_tts(&min), &text.get_ppq_tts(&max)],
                        )),
                        None => TtsString::from(
                            text.get_error("There is no end time to the selection."),
                        ),
                    },
                    None => {
                        TtsString::from(text.get_error("There is no start time to the selection."))
                    }
                },
                None => TtsString::from(text.get_error("The selected notes don't exist.")),
            },
        };
        vec![tts_string]
    }

    fn get_input_tts(&mut self, state: &State, input: &Input, text: &Text) -> Vec<TtsString> {
        let (mut tts_strings, selected) = match &state.select_mode {
            SelectMode::Single(index) => match index {
                Some(_) => (
                    vec![self
                        .tooltips
                        .get_tooltip(
                            "PIANO_ROLL_PANEL_INPUT_TTS_SELECT_SINGLE",
                            &[InputEvent::SelectStartLeft, InputEvent::SelectStartRight],
                            input,
                            text,
                        )
                        .clone()],
                    true,
                ),
                None => (vec![get_no_selection_status_tts(text)], false),
            },
            SelectMode::Many(indices) => match indices {
                Some(_) => (
                    vec![self
                        .tooltips
                        .get_tooltip(
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
                        )
                        .clone()],
                    true,
                ),
                None => (vec![get_no_selection_status_tts(text)], false),
            },
        };
        if state.select_mode.get_note_indices().is_some() {
            tts_strings.push(
                self.tooltips
                    .get_tooltip(
                        "PIANO_ROLL_PANEL_INPUT_TTS_SELECT_ALL",
                        &[InputEvent::SelectAll],
                        input,
                        text,
                    )
                    .clone(),
            );
        }
        if selected {
            tts_strings.push(
                self.tooltips
                    .get_tooltip(
                        "PIANO_ROLL_PANEL_INPUT_TTS_DESELECT",
                        &[InputEvent::SelectNone],
                        input,
                        text,
                    )
                    .clone(),
            );
        }
        let cycle_key = match state.select_mode {
            SelectMode::Single(_) => "PIANO_ROLL_PANEL_INPUT_TTS_SELECT_CYCLE_TO_MANY",
            SelectMode::Many(_) => "PIANO_ROLL_PANEL_INPUT_TTS_SELECT_CYCLE_TO_SINGLE",
        };
        tts_strings.push(
            self.tooltips
                .get_tooltip(cycle_key, &[InputEvent::PianoRollCycleMode], input, text)
                .clone(),
        );
        tts_strings
    }
}
