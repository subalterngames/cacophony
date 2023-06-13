use super::{get_edit_mode_status_tts, EditModeDeltas, PianoRollSubPanel};
use crate::panel::*;
use common::ini::Ini;
use common::Zero;
use common::{Fraction, EDIT_MODES};

/// The piano roll time sub-panel.
pub(super) struct Time {
    /// Time values and deltas.
    deltas: EditModeDeltas,
}

impl Time {
    pub fn new(config: &Ini) -> Self {
        Self {
            deltas: EditModeDeltas::new(config),
        }
    }

    /// Get the end time for a cursor.
    fn get_end(state: &State) -> Fraction {
        match state.music.get_selected_track() {
            None => panic!("This should never happen!"),
            Some(track) => match track.get_end() {
                Some(t1) => t1,
                None => state.view.dt[1],
            },
        }
    }

    /// Move the time left.
    fn get_left(&self, t: &Fraction, state: &State) -> Fraction {
        let mode = EDIT_MODES[state.time.mode.get()];
        let dt = self.deltas.get_dt(&mode, &state.input);
        let t1 = t - dt;
        if t1.is_sign_negative() {
            Fraction::zero()
        } else {
            t1
        }
    }
}

impl Panel for Time {
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
            let s0 = state.clone();
            state.time.mode.increment(true);
            Some(Snapshot::from_states(s0, state))
        }
        // Move the cursor.
        else if input.happened(&InputEvent::TimeCursorStart) {
            let s0 = state.clone();
            state.time.cursor = Fraction::zero();
            Some(Snapshot::from_states(s0, state))
        } else if input.happened(&InputEvent::TimeCursorEnd) {
            let s0 = state.clone();
            state.time.cursor = Time::get_end(state);
            Some(Snapshot::from_states(s0, state))
        } else if input.happened(&InputEvent::TimeCursorLeft) {
            let s0 = state.clone();
            state.time.cursor = self.get_left(&state.time.cursor, state);
            Some(Snapshot::from_states(s0, state))
        } else if input.happened(&InputEvent::TimeCursorRight) {
            let mode = EDIT_MODES[state.time.mode.get()];
            let s0 = state.clone();
            let dt = self.deltas.get_dt(&mode, &state.input);
            state.time.cursor += dt;
            Some(Snapshot::from_states(s0, state))
        }
        // Move the playback.
        else if input.happened(&InputEvent::TimePlaybackStart) {
            let s0 = state.clone();
            state.time.playback = Fraction::zero();
            Some(Snapshot::from_states(s0, state))
        } else if input.happened(&InputEvent::TimePlaybackEnd) {
            let s0 = state.clone();
            state.time.playback = Time::get_end(state);
            Some(Snapshot::from_states(s0, state))
        } else if input.happened(&InputEvent::TimePlaybackLeft) {
            let s0 = state.clone();
            state.time.playback = self.get_left(&state.time.playback, state);
            Some(Snapshot::from_states(s0, state))
        } else if input.happened(&InputEvent::TimePlaybackRight) {
            let mode = EDIT_MODES[state.time.mode.get()];
            let s0 = state.clone();
            let dt = self.deltas.get_dt(&mode, &state.input);
            state.time.playback += dt;
            Some(Snapshot::from_states(s0, state))
        } else {
            None
        }
    }
}

impl PianoRollSubPanel for Time {
    fn get_status_tts(&self, state: &State, text: &Text) -> String {
        let mut s = get_edit_mode_status_tts(&EDIT_MODES[state.time.mode.get()], text);
        s.push(' ');
        s.push_str(&text.get_with_values(
            "PIANO_ROLL_PANEL_STATUS_TTS_TIME",
            &[
                &text.get_fraction_tts(&state.time.cursor),
                &text.get_fraction_tts(&state.time.playback),
            ],
        ));
        s
    }

    fn get_input_tts(&self, _: &State, input: &Input, text: &Text) -> String {
        get_tooltip(
            "PIANO_ROLL_PANEL_INPUT_TTS_TIME",
            &[
                InputEvent::TimeCursorLeft,
                InputEvent::TimeCursorRight,
                InputEvent::TimeCursorStart,
                InputEvent::TimeCursorEnd,
                InputEvent::TimePlaybackLeft,
                InputEvent::TimePlaybackRight,
                InputEvent::TimePlaybackStart,
                InputEvent::TimePlaybackEnd,
            ],
            input,
            text,
        )
    }
}
