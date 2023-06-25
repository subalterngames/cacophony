use super::{get_edit_mode_status_tts, EditModeDeltas, PianoRollSubPanel};
use crate::panel::*;
use common::ini::Ini;
use common::*;

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

    /// Get the end time in PPQ for a cursor.
    fn get_end(state: &State) -> u64 {
        match state.music.get_selected_track() {
            None => panic!("This should never happen!"),
            Some(track) => match track.get_end() {
                Some(t1) => t1,
                None => state.view.dt[1],
            },
        }
    }

    /// Move the cursor time.
    fn set_cursor(&self, state: &mut State, add: bool) -> Option<Snapshot> {
        let s0 = state.clone();
        let mode = EDIT_MODES[state.time.mode.get()];
        let dt = self.deltas.get_dt(&mode, &state.input);
        if add {
            state.time.cursor += dt;
        } else {
            state.time.cursor = state.time.cursor.saturating_sub(dt);
        }
        Some(Snapshot::from_states(s0, state))
    }

    /// Move the playback time.
    fn set_playback(&self, state: &mut State, add: bool) -> Option<Snapshot> {
        let s0 = state.clone();
        let mode = EDIT_MODES[state.time.mode.get()];
        let dt = self.deltas.get_dt(&mode, &state.input);
        if add {
            state.time.playback += dt;
        } else {
            state.time.playback = state.time.cursor.saturating_sub(dt);
        }
        Some(Snapshot::from_states(s0, state))
    }

    /// Round a time off to the nearest beat.
    fn get_nearest_beat(t: u64, state: &State) -> u64 {
        ((t as f32 / state.input.beat.get_f()).ceil() * state.input.beat.get_f()) as u64
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
        _: &mut Exporter,
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
            state.time.cursor = 0;
            Some(Snapshot::from_states(s0, state))
        } else if input.happened(&InputEvent::TimeCursorEnd) {
            let s0 = state.clone();
            state.time.cursor = Time::get_end(state);
            Some(Snapshot::from_states(s0, state))
        } else if input.happened(&InputEvent::TimeCursorLeft) {
            self.set_cursor(state, false)
        } else if input.happened(&InputEvent::TimeCursorRight) {
            self.set_cursor(state, true)
        } else if input.happened(&InputEvent::TimeCursorPlayback) {
            let s0 = state.clone();
            state.time.cursor = state.time.playback;
            Some(Snapshot::from_states(s0, state))
        } else if input.happened(&InputEvent::TimeCursorBeat) {
            let s0 = state.clone();
            state.time.cursor = Time::get_nearest_beat(state.time.cursor, state);
            Some(Snapshot::from_states(s0, state))
        }
        // Move the playback.
        else if input.happened(&InputEvent::TimePlaybackStart) {
            let s0 = state.clone();
            state.time.playback = 0;
            Some(Snapshot::from_states(s0, state))
        } else if input.happened(&InputEvent::TimePlaybackEnd) {
            let s0 = state.clone();
            state.time.playback = Time::get_end(state);
            Some(Snapshot::from_states(s0, state))
        } else if input.happened(&InputEvent::TimePlaybackLeft) {
            self.set_playback(state, false)
        } else if input.happened(&InputEvent::TimePlaybackRight) {
            self.set_playback(state, true)
        } else if input.happened(&InputEvent::TimePlaybackCursor) {
            let s0 = state.clone();
            state.time.playback = state.time.cursor;
            Some(Snapshot::from_states(s0, state))
        } else if input.happened(&InputEvent::TimePlaybackBeat) {
            let s0 = state.clone();
            state.time.playback = Time::get_nearest_beat(state.time.playback, state);
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
                &text.get_ppq_tts(&state.time.cursor),
                &text.get_ppq_tts(&state.time.playback),
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
                InputEvent::TimeCursorBeat,
                InputEvent::TimeCursorPlayback,
                InputEvent::TimePlaybackLeft,
                InputEvent::TimePlaybackRight,
                InputEvent::TimePlaybackStart,
                InputEvent::TimePlaybackEnd,
                InputEvent::TimePlaybackBeat,
                InputEvent::TimePlaybackCursor,
            ],
            input,
            text,
        )
    }
}
