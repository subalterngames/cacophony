use super::{EditModeDeltas, PianoRollSubPanel, get_edit_mode_status_tts, get_no_selection_status_tts, get_cycle_edit_mode_input_tts};
use crate::panel::*;
use common::ini::Ini;
use common::{Fraction, EDIT_MODES, MAX_NOTE, MAX_VOLUME, MIN_NOTE, Zero};

/// Edit selected notes.
pub(super) struct Edit {
    /// The edit mode deltas.
    deltas: EditModeDeltas,
}

impl Edit {
    pub fn new(config: &Ini) -> Self {
        Self {
            deltas: EditModeDeltas::new(config),
        }
    }
}

impl Panel for Edit {
    fn update(
        &mut self,
        state: &mut State,
        _: &mut Conn,
        input: &Input,
        _: &mut TTS,
        _: &Text,
    ) -> Option<UndoRedoState> {
        // Do nothing if there is no track.
        if state.music.selected.is_none() {
            None
        }
        // Cycle the mode.
        else if input.happened(&InputEvent::PianoRollCycleMode) {
            let s0 = state.clone();
            state.edit_mode.increment(true);
            Some(UndoRedoState::from((s0, state)))
        } else {
            let mode = EDIT_MODES[state.edit_mode.get()];
            let s0 = state.clone();
            // Are there notes we can edit?
            match state.select_mode.get_notes_mut(&mut state.music) {
                Some(mut notes) => {
                    // Move the notes left.
                    if input.happened(&InputEvent::EditStartLeft) {
                        let dt = self.deltas.get_dt(&mode, &state.input);
                        // Don't let any notes go to t=0.
                        if !notes.iter().any(|n| (n.start - dt).is_sign_negative()) {
                            notes.iter_mut().for_each(|n| n.start -= dt);
                            Some(UndoRedoState::from((s0, state)))
                        } else {
                            None
                        }
                    }
                    // Move the notes right.
                    else if input.happened(&InputEvent::EditStartRight) {
                        let dt = self.deltas.get_dt(&mode, &state.input);
                        notes.iter_mut().for_each(|n| n.start -= dt);
                        Some(UndoRedoState::from((s0, state)))
                    }
                    // Shorten the duration.
                    else if input.happened(&InputEvent::EditDurationLeft) {
                        let dt = self.deltas.get_dt(&mode, &state.input);
                        // Don't let any notes go to dt<=0.
                        let zero = Fraction::zero();
                        if notes.iter().all(|n| (n.duration - dt) > zero) {
                            notes.iter_mut().for_each(|n| n.duration -= dt);
                            Some(UndoRedoState::from((s0, state)))
                        } else {
                            None
                        }
                    }
                    // Lengthen the notes.
                    else if input.happened(&InputEvent::EditDurationRight) {
                        let dt = self.deltas.get_dt(&mode, &state.input);
                        notes.iter_mut().for_each(|n| n.duration += dt);
                        Some(UndoRedoState::from((s0, state)))
                    }
                    // Move the notes up.
                    else if input.happened(&InputEvent::EditPitchUp) {
                        let dn = self.deltas.get_dn(&mode);
                        // Don't let any notes go to dn>=max.
                        if notes.iter().all(|n| (n.note + dn) <= MAX_NOTE) {
                            notes.iter_mut().for_each(|n| n.note += dn);
                            Some(UndoRedoState::from((s0, state)))
                        } else {
                            None
                        }
                    }
                    // Move the notes down.
                    else if input.happened(&InputEvent::EditPitchDown) {
                        let dn = self.deltas.get_dn(&mode);
                        // Don't let any notes go to dn<=0.
                        if notes.iter().all(|n| (n.note - dn) >= MIN_NOTE) {
                            notes.iter_mut().for_each(|n| n.note -= dn);
                            Some(UndoRedoState::from((s0, state)))
                        } else {
                            None
                        }
                    }
                    // Increase the volume.
                    else if input.happened(&InputEvent::EditVolumeUp) {
                        let dv = self.deltas.get_dv(&mode);
                        // Don't let any notes go to dv>=max.
                        if notes.iter().all(|n| (n.velocity + dv) <= MAX_VOLUME) {
                            notes.iter_mut().for_each(|n| n.velocity += dv);
                            Some(UndoRedoState::from((s0, state)))
                        } else {
                            None
                        }
                    }
                    // Decrease the volume.
                    else if input.happened(&InputEvent::EditVolumeDown) {
                        let dv = self.deltas.get_dv(&mode);
                        // Don't let any notes go to dv<=0.
                        if notes.iter().all(|n| (n.velocity as i8 - dv as i8) >= 0) {
                            notes.iter_mut().for_each(|n| n.velocity -= dv);
                            Some(UndoRedoState::from((s0, state)))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                None => None,
            }
        }
    }
}

impl PianoRollSubPanel for Edit {
    fn get_status_tts(&self, state: &State, text: &Text) -> String {
        get_edit_mode_status_tts(&EDIT_MODES[state.edit_mode.get()], text)
    }

    fn get_input_tts(&self, state: &State, input: &Input, text: &Text) -> String {
        let mut s = match state.select_mode.get_note_indices() {
            Some(_) => {
                get_tooltip("PIANO_ROLL_PANEL_INPUT_TTS_EDIT", &[
                    InputEvent::EditPitchUp,
                    InputEvent::EditPitchDown,
                    InputEvent::EditStartLeft,
                    InputEvent::EditStartRight,
                    InputEvent::EditDurationLeft,
                    InputEvent::EditDurationRight,
                    InputEvent::EditVolumeUp,
                    InputEvent::EditVolumeDown,
                ], input, text)
            }
            None => get_no_selection_status_tts(text)
        };
        s.push(' ');
        s.push_str(&get_cycle_edit_mode_input_tts(&state.edit_mode, input, text));
        s
    }
}