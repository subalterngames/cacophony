use super::{get_edit_mode_status_tts, EditModeDeltas, PianoRollSubPanel};
use crate::panel::*;
use ini::Ini;

/// The piano roll time sub-panel.
pub(super) struct Time {
    /// Time values and deltas.
    deltas: EditModeDeltas,
    tooltips: Tooltips,
}

impl Time {
    pub fn new(config: &Ini) -> Self {
        Self {
            deltas: EditModeDeltas::new(config),
            tooltips: Tooltips::default(),
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
        let mode = state.time.mode.get_ref();
        let dt = self.deltas.get_dt(mode, &state.input);
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
        let mode = state.time.mode.get_ref();
        let dt = self.deltas.get_dt(mode, &state.input);
        if add {
            state.time.playback += dt;
        } else {
            state.time.playback = state.time.playback.saturating_sub(dt);
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
        _: &mut SharedExporter,
    ) -> Option<Snapshot> {
        // Do nothing if there is no track.
        if state.music.selected.is_none() {
            None
        }
        // Cycle the mode.
        else if input.happened(&InputEvent::PianoRollCycleMode) {
            Some(Snapshot::from_state(
                |s| s.time.mode.index.increment(true),
                state,
            ))
        }
        // Move the cursor.
        else if input.happened(&InputEvent::TimeCursorStart) {
            Some(Snapshot::from_state_value(|s| &mut s.time.cursor, 0, state))
        } else if input.happened(&InputEvent::TimeCursorEnd) {
            Some(Snapshot::from_state_value(
                |s| &mut s.time.cursor,
                Time::get_end(state),
                state,
            ))
        } else if input.happened(&InputEvent::TimeCursorLeft) {
            self.set_cursor(state, false)
        } else if input.happened(&InputEvent::TimeCursorRight) {
            self.set_cursor(state, true)
        } else if input.happened(&InputEvent::TimeCursorPlayback) {
            Some(Snapshot::from_state_value(
                |s: &mut State| &mut s.time.cursor,
                state.time.playback,
                state,
            ))
        } else if input.happened(&InputEvent::TimeCursorBeat) {
            Some(Snapshot::from_state_value(
                |s: &mut State| &mut s.time.cursor,
                Time::get_nearest_beat(state.time.cursor, state),
                state,
            ))
        }
        // Move the playback.
        else if input.happened(&InputEvent::TimePlaybackStart) {
            Some(Snapshot::from_state_value(
                |s: &mut State| &mut s.time.playback,
                0,
                state,
            ))
        } else if input.happened(&InputEvent::TimePlaybackEnd) {
            Some(Snapshot::from_state_value(
                |s: &mut State| &mut s.time.playback,
                Time::get_end(state),
                state,
            ))
        } else if input.happened(&InputEvent::TimePlaybackLeft) {
            self.set_playback(state, false)
        } else if input.happened(&InputEvent::TimePlaybackRight) {
            self.set_playback(state, true)
        } else if input.happened(&InputEvent::TimePlaybackCursor) {
            Some(Snapshot::from_state_value(
                |s: &mut State| &mut s.time.playback,
                state.time.cursor,
                state,
            ))
        } else if input.happened(&InputEvent::TimePlaybackBeat) {
            Some(Snapshot::from_state_value(
                |s| &mut s.time.playback,
                Time::get_nearest_beat(state.time.playback, state),
                state,
            ))
        } else {
            None
        }
    }

    fn on_disable_abc123(&mut self, _: &mut State, _: &mut SharedExporter) {}

    fn update_abc123(
        &mut self,
        _: &mut State,
        _: &Input,
        _: &mut SharedExporter,
    ) -> (Option<Snapshot>, bool) {
        (None, false)
    }

    fn allow_alphanumeric_input(&self, _: &State, _: &SharedExporter) -> bool {
        false
    }

    fn allow_play_music(&self) -> bool {
        true
    }
}

impl PianoRollSubPanel for Time {
    fn get_status_tts(&mut self, state: &State, text: &Text) -> Vec<TtsString> {
        let mut s = vec![get_edit_mode_status_tts(state.time.mode.get_ref(), text)];
        s.push(TtsString::from(text.get_with_values(
            "PIANO_ROLL_PANEL_STATUS_TTS_TIME",
            &[
                &text.get_ppq_tts(&state.time.cursor),
                &text.get_ppq_tts(&state.time.playback),
            ],
        )));
        s
    }

    fn get_input_tts(&mut self, _: &State, input: &Input, text: &Text) -> Vec<TtsString> {
        vec![
            self.tooltips.get_tooltip(
                "PIANO_ROLL_PANEL_INPUT_TTS_TIME_0",
                &[InputEvent::TimeCursorLeft, InputEvent::TimeCursorRight],
                input,
                text,
            ),
            self.tooltips.get_tooltip(
                "PIANO_ROLL_PANEL_INPUT_TTS_TIME_1",
                &[InputEvent::TimeCursorStart, InputEvent::TimeCursorEnd],
                input,
                text,
            ),
            self.tooltips.get_tooltip(
                "PIANO_ROLL_PANEL_INPUT_TTS_TIME_2",
                &[InputEvent::TimeCursorBeat],
                input,
                text,
            ),
            self.tooltips.get_tooltip(
                "PIANO_ROLL_PANEL_INPUT_TTS_TIME_3",
                &[InputEvent::TimeCursorPlayback],
                input,
                text,
            ),
            self.tooltips.get_tooltip(
                "PIANO_ROLL_PANEL_INPUT_TTS_TIME_4",
                &[InputEvent::TimePlaybackLeft, InputEvent::TimePlaybackRight],
                input,
                text,
            ),
            self.tooltips.get_tooltip(
                "PIANO_ROLL_PANEL_INPUT_TTS_TIME_5",
                &[InputEvent::TimePlaybackStart, InputEvent::TimePlaybackEnd],
                input,
                text,
            ),
            self.tooltips.get_tooltip(
                "PIANO_ROLL_PANEL_INPUT_TTS_TIME_6",
                &[InputEvent::TimePlaybackBeat],
                input,
                text,
            ),
            self.tooltips.get_tooltip(
                "PIANO_ROLL_PANEL_INPUT_TTS_TIME_7",
                &[InputEvent::TimePlaybackCursor],
                input,
                text,
            ),
        ]
    }
}
