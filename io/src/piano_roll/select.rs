use super::{get_no_selection_status_tts, PianoRollSubPanel};
use crate::panel::*;
use common::time::Time;
use common::{MidiTrack, Note};

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
                        } else {
                            None
                        }
                    }
                    // Add a note at the end.
                    else if input.happened(&InputEvent::SelectEndRight) {
                        let s0 = state.clone();
                        if state.selection.deselect_last(&state.music) {
                            Some(Snapshot::from_states(s0, state))
                        } else {
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
        vec![match state.selection.get_selection(&state.music) {
            Some((notes, effects)) => {
                // A single note or effect.
                if state.selection.single {
                    if !notes.is_empty() {
                        let note = notes[0];
                        TtsString::from(text.get_with_values(
                            "PIANO_ROLL_PANEL_STATUS_TTS_SELECTED_SINGLE_NOTE",
                            &[&note.note.to_string(), &text.get_ppq_tts(&note.start)],
                        ))
                    } else if !effects.is_empty() {
                        let effect = effects[0];
                        TtsString::from(text.get_with_values(
                            "PIANO_ROLL_PANEL_STATUS_TTS_SELECTED_SINGLE_EFFECT",
                            &[&text.get_ppq_tts(&effect.time)],
                        ))
                    } else {
                        get_no_selection_status_tts(text)
                    }
                }
                // Multiple notes and effects.
                else {
                    match state.selection.get_dt(&state.music) {
                        Some((min, max)) => TtsString::from(text.get_with_values(
                            "PIANO_ROLL_PANEL_STATUS_TTS_SELECTED_MANY",
                            &[&text.get_ppq_tts(&min), &text.get_ppq_tts(&max)],
                        )),
                        None => get_no_selection_status_tts(text),
                    }
                }
            }
            None => get_no_selection_status_tts(text),
        }]
    }

    fn get_input_tts(&mut self, state: &State, input: &Input, text: &Text) -> Vec<TtsString> {
        let mut tts_strings = match state.selection.get_selection(&state.music) {
            Some((notes, effects)) => {
                let mut tts_strings = vec![];
                let empty = notes.is_empty() && effects.is_empty();
                // There is no selection.
                if !empty {
                    if state.selection.single {
                        tts_strings.push(self.tooltips.get_tooltip(
                            "PIANO_ROLL_PANEL_INPUT_TTS_SELECT_SINGLE",
                            &[InputEvent::SelectStartLeft, InputEvent::SelectStartRight],
                            input,
                            text,
                        ));
                    } else {
                        tts_strings.push(self.tooltips.get_tooltip(
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
                        ));
                    }
                }
                // Select all.
                tts_strings.push(self.tooltips.get_tooltip(
                    "PIANO_ROLL_PANEL_INPUT_TTS_SELECT_ALL",
                    &[InputEvent::SelectAll],
                    input,
                    text,
                ));
                // Deselect.
                if !empty {
                    tts_strings.push(self.tooltips.get_tooltip(
                        "PIANO_ROLL_PANEL_INPUT_TTS_DESELECT",
                        &[InputEvent::SelectNone],
                        input,
                        text,
                    ));
                }
                tts_strings
            }
            None => vec![get_no_selection_status_tts(text)],
        };
        let cycle_key = if state.selection.single {
            "PIANO_ROLL_PANEL_INPUT_TTS_SELECT_CYCLE_TO_MANY"
        } else {
            "PIANO_ROLL_PANEL_INPUT_TTS_SELECT_CYCLE_TO_SINGLE"
        };
        tts_strings.push(
            self.tooltips
                .get_tooltip(cycle_key, &[InputEvent::PianoRollCycleMode], input, text)
                .clone(),
        );
        tts_strings
    }
}
