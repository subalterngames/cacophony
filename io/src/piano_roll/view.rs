use super::{
    get_cycle_edit_mode_input_tts, get_edit_mode_status_tts, EditModeDeltas, PianoRollSubPanel,
};
use crate::panel::*;
use common::ini::Ini;
use common::Zero;
use common::{Fraction, EDIT_MODES, MAX_NOTE, MIN_NOTE};

/// The piano roll view sub-pane
pub(super) struct View {
    /// Time values and deltas.
    deltas: EditModeDeltas,
}

impl View {
    pub fn new(config: &Ini) -> Self {
        Self {
            deltas: EditModeDeltas::new(config),
        }
    }

    /// Returns the delta from the viewport's t1 to its t0.
    fn get_dt(state: &State) -> Fraction {
        state.view.dt[1] - state.view.dt[0]
    }

    /// Returns the delta from the viewport's n1 to its n0.
    fn get_dn(state: &State) -> u8 {
        state.view.dn[0] - state.view.dn[1]
    }
}

impl Panel for View {
    fn update(
        &mut self,
        state: &mut State,
        _: &mut Conn,
        input: &Input,
        _: &mut TTS,
        _: &Text,
    ) -> Option<Snapshot> {
        // Do nothing if there is no track.
        if state.music.selected.is_none() {
            None
        }
        // Cycle the mode.
        else if input.happened(&InputEvent::PianoRollCycleMode) {
            let s0 = state.clone();
            state.view.mode.increment(true);
            Some(Snapshot::from_states(s0, state))
        }
        // Move the view to t0.
        else if input.happened(&InputEvent::ViewStart) {
            let s0 = state.clone();
            let dt = View::get_dt(state);
            state.view.dt = [Fraction::zero(), dt];
            Some(Snapshot::from_states(s0, state))
        }
        // Move the view to t1.
        else if input.happened(&InputEvent::ViewEnd) {
            let dt = View::get_dt(state);
            let track = state.music.get_selected_track().unwrap();
            match track.get_end() {
                // Move the view to the end.
                Some(max) => {
                    let s0 = state.clone();
                    state.view.dt = [max, max + dt];
                    Some(Snapshot::from_states(s0, state))
                }
                // Move the view one viewport's dt rightwards.
                None => {
                    let s0 = state.clone();
                    state.view.dt = [dt, dt * 2];
                    Some(Snapshot::from_states(s0, state))
                }
            }
        }
        // Move the view leftwards.
        else if input.happened(&InputEvent::ViewLeft) {
            let mode = EDIT_MODES[state.time.mode.get()];
            let s0 = state.clone();
            let dt = self.deltas.get_dt(&mode, &state.input);
            let t0 = state.view.dt[0] - dt;
            // Don't go past t=0.
            if t0.is_zero() || t0.is_sign_positive() {
                let t1 = state.view.dt[1] - dt;
                state.view.dt = [t0, t1];
                Some(Snapshot::from_states(s0, state))
            }
            // Snap to t=0.
            else {
                state.view.dt = [Fraction::zero(), state.view.dt[1] - state.view.dt[0]];
                Some(Snapshot::from_states(s0, state))
            }
        }
        // Move the view rightwards.
        else if input.happened(&InputEvent::ViewRight) {
            let mode = EDIT_MODES[state.time.mode.get()];
            let s0 = state.clone();
            let dt = self.deltas.get_dt(&mode, &state.input);
            let t0 = state.view.dt[0] + dt;
            let t1 = state.view.dt[1] + dt;
            state.view.dt = [t0, t1];
            Some(Snapshot::from_states(s0, state))
        }
        // Move the view upwards.
        else if input.happened(&InputEvent::ViewUp) {
            let mode = EDIT_MODES[state.time.mode.get()];
            let s0 = state.clone();
            let dn = self.deltas.get_dn(&mode);
            // Don't go past n=1.
            if state.view.dn[0] + dn <= MAX_NOTE {
                let n0 = state.view.dn[0] + dn;
                let n1 = state.view.dn[1] + dn;
                state.view.dn = [n0, n1];
                Some(Snapshot::from_states(s0, state))
            }
            // Snap to n=1.
            else {
                let dn = View::get_dn(state);
                state.view.dn = [MAX_NOTE, MAX_NOTE - dn];
                Some(Snapshot::from_states(s0, state))
            }
        }
        // Move the view downwards.
        else if input.happened(&InputEvent::ViewDown) {
            let mode = EDIT_MODES[state.time.mode.get()];
            let s0 = state.clone();
            let dn = self.deltas.get_dn(&mode);
            // Don't go past n=0.
            if state.view.dn[1] - dn >= MIN_NOTE {
                let n0 = state.view.dn[0] - dn;
                let n1 = state.view.dn[1] - dn;
                state.view.dn = [n0, n1];
                Some(Snapshot::from_states(s0, state))
            }
            // Snap to n=0.
            else {
                let dn = View::get_dn(state);
                state.view.dn = [MIN_NOTE + dn, MIN_NOTE];
                Some(Snapshot::from_states(s0, state))
            }
        } else {
            None
        }
    }
}

impl PianoRollSubPanel for View {
    fn get_status_tts(&self, state: &State, text: &Text) -> String {
        let mut s = text.get_with_values(
            "PIANO_ROLL_PANEL_STATUS_TTS_VIEW",
            &[
                &text.get_fraction_tts(&state.view.dt[0]),
                &text.get_fraction_tts(&state.view.dt[1]),
                &text.get_note_name(state.view.dn[0]),
                &text.get_note_name(state.view.dn[1]),
            ],
        );
        s.push(' ');
        s.push_str(&get_edit_mode_status_tts(
            &EDIT_MODES[state.view.mode.get()],
            text,
        ));
        s
    }

    fn get_input_tts(&self, state: &State, input: &Input, text: &Text) -> String {
        // PIANO_ROLL_PANEL_INPUT_TTS_VIEW,"\0, \1, \2, and \3 to move the view. \4 and \5 to set the view to the start and end."
        let mut s = get_tooltip(
            "PIANO_ROLL_PANEL_INPUT_TTS_VIEW",
            &[
                InputEvent::ViewUp,
                InputEvent::ViewDown,
                InputEvent::ViewLeft,
                InputEvent::ViewRight,
                InputEvent::ViewStart,
                InputEvent::ViewEnd,
            ],
            input,
            text,
        );
        s.push(' ');
        s.push_str(&get_cycle_edit_mode_input_tts(
            &state.view.mode,
            input,
            text,
        ));
        s
    }
}
