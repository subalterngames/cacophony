use super::{
    get_cycle_edit_mode_input_tts, get_edit_mode_status_tts, get_no_selection_status_tts,
    EditModeDeltas, PianoRollSubPanel,
};
use crate::panel::*;
use common::{MAX_NOTE, MAX_VOLUME, MIN_NOTE};
use ini::Ini;

/// Edit selected notes.
pub(super) struct Edit {
    /// The edit mode deltas.
    deltas: EditModeDeltas,
    tooltips: Tooltips,
}

impl Edit {
    pub fn new(config: &Ini) -> Self {
        Self {
            deltas: EditModeDeltas::new(config),
            tooltips: Tooltips::default(),
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
        _: &mut PathsState,
    ) -> Option<Snapshot> {
        // Do nothing if there is no track.
        if state.music.selected.is_none() {
            None
        }
        // Cycle the mode.
        else if input.happened(&InputEvent::PianoRollCycleMode) {
            Some(Snapshot::from_state(
                |s| s.edit_mode.index.increment(true),
                state,
            ))
        } else {
            let mode = state.edit_mode.get_ref();
            let s0 = state.clone();
            // Is anything selected?
            match state.selection.get_selection_mut(&mut state.music) {
                Some((mut notes, mut effects)) => {
                    // Move the selection left.
                    if input.happened(&InputEvent::EditStartLeft) {
                        let dt = self.deltas.get_dt(mode, &state.input);
                        let mut snapshot = false;
                        // Don't let any notes or events go to t=0.
                        if !notes.iter().any(|n| n.start.checked_sub(dt).is_none()) {
                            notes.iter_mut().for_each(|n| n.set_t0_by(dt, false));
                            snapshot = true;
                        }
                        if !effects.iter().any(|e| e.time.checked_sub(dt).is_none()) {
                            effects.iter_mut().for_each(|e| e.time -= dt);
                            snapshot = true;
                        }
                        if snapshot {
                            Some(Snapshot::from_states(s0, state))
                        } else {
                            None
                        }
                    }
                    // Move the selection right.
                    else if input.happened(&InputEvent::EditStartRight) {
                        let dt = self.deltas.get_dt(mode, &state.input);
                        notes.iter_mut().for_each(|n| n.set_t0_by(dt, true));
                        effects.iter_mut().for_each(|e| e.time += dt);
                        Some(Snapshot::from_states(s0, state))
                    }
                    // Shorten the duration.
                    else if input.happened(&InputEvent::EditDurationLeft) {
                        let dt = self.deltas.get_dt(mode, &state.input);
                        // Don't let any notes go to dt<=0.
                        if notes
                            .iter()
                            .all(|n| n.get_duration().checked_sub(dt).is_some())
                        {
                            notes.iter_mut().for_each(|n| n.end -= dt);
                            Some(Snapshot::from_states(s0, state))
                        } else {
                            None
                        }
                    }
                    // Lengthen the notes.
                    else if input.happened(&InputEvent::EditDurationRight) {
                        let dt = self.deltas.get_dt(mode, &state.input);
                        notes.iter_mut().for_each(|n| n.end += dt);
                        Some(Snapshot::from_states(s0, state))
                    }
                    // Move the notes up.
                    else if input.happened(&InputEvent::EditPitchUp) {
                        let dn = self.deltas.get_dn(mode);
                        // Don't let any notes go to dn>=max.
                        if notes.iter().all(|n| (n.note + dn) <= MAX_NOTE) {
                            notes.iter_mut().for_each(|n| n.note += dn);
                            Some(Snapshot::from_states(s0, state))
                        } else {
                            None
                        }
                    }
                    // Move the notes down.
                    else if input.happened(&InputEvent::EditPitchDown) {
                        let dn = self.deltas.get_dn(mode);
                        // Don't let any notes go to dn<=0.
                        if notes.iter().all(|n| (n.note - dn) >= MIN_NOTE) {
                            notes.iter_mut().for_each(|n| n.note -= dn);
                            Some(Snapshot::from_states(s0, state))
                        } else {
                            None
                        }
                    }
                    // Increase the volume.
                    else if input.happened(&InputEvent::EditVolumeUp) {
                        let dv = self.deltas.get_dv(mode);
                        // Don't let any notes go to dv>=max.
                        if notes.iter().all(|n| (n.velocity + dv) <= MAX_VOLUME) {
                            notes.iter_mut().for_each(|n| n.velocity += dv);
                            Some(Snapshot::from_states(s0, state))
                        } else {
                            None
                        }
                    }
                    // Decrease the volume.
                    else if input.happened(&InputEvent::EditVolumeDown) {
                        let dv = self.deltas.get_dv(mode);
                        // Don't let any notes go to dv<=0.
                        if notes.iter().all(|n| (n.velocity as i8 - dv as i8) >= 0) {
                            notes.iter_mut().for_each(|n| n.velocity -= dv);
                            Some(Snapshot::from_states(s0, state))
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

impl PianoRollSubPanel for Edit {
    fn get_status_tts(&mut self, state: &State, text: &Text) -> Vec<TtsString> {
        vec![get_edit_mode_status_tts(state.edit_mode.get_ref(), text)]
    }

    fn get_input_tts(&mut self, state: &State, input: &Input, text: &Text) -> Vec<TtsString> {
        let mut tts_strings = vec![];
        match state.selection.get_selection(&state.music) {
            Some((notes, effects)) => {
                // E1dit pitch.
                let notes_empty = notes.is_empty();
                if !notes_empty {
                    tts_strings.push(self.tooltips.get_tooltip(
                        "PIANO_ROLL_PANEL_INPUT_TTS_EDIT_0",
                        &[InputEvent::EditPitchUp, InputEvent::EditPitchDown],
                        input,
                        text,
                    ));
                }
                // Set start time.
                if !notes_empty || !effects.is_empty() {
                    self.tooltips.get_tooltip(
                        "PIANO_ROLL_PANEL_INPUT_TTS_EDIT_1",
                        &[InputEvent::EditStartLeft, InputEvent::EditStartRight],
                        input,
                        text,
                    );
                }
                // Set the duration and volume.
                if !notes_empty {
                    tts_strings.push(self.tooltips.get_tooltip(
                        "PIANO_ROLL_PANEL_INPUT_TTS_EDIT_2",
                        &[InputEvent::EditDurationLeft, InputEvent::EditDurationRight],
                        input,
                        text,
                    ));
                    tts_strings.push(self.tooltips.get_tooltip(
                        "PIANO_ROLL_PANEL_INPUT_TTS_EDIT_3",
                        &[InputEvent::EditVolumeUp, InputEvent::EditVolumeDown],
                        input,
                        text,
                    ));
                }
            }
            None => tts_strings.push(get_no_selection_status_tts(text)),
        }
        tts_strings.push(get_cycle_edit_mode_input_tts(
            &mut self.tooltips,
            &state.edit_mode,
            input,
            text,
        ));
        tts_strings
    }
}
