use super::EditModeDeltas;
use crate::get_tooltip_with_values;
use crate::panel::*;
use common::ini::Ini;
use common::Zero;
use common::{Fraction, EDIT_MODES};

/// The piano roll time sub-panel.
pub struct Time {
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
        let mode = EDIT_MODES[state.view.mode.get()];
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
        tts: &mut TTS,
        text: &Text,
    ) -> Option<UndoRedoState> {
        // Do nothing if there is no track.
        if state.music.selected.is_none() {
            None
        }
        // Cycle the mode.
        else if input.happened(&InputEvent::PianoRollCycleMode) {
            let s0 = state.clone();
            state.time.mode.increment(true);
            Some(UndoRedoState::from((s0, state)))
        }
        // TTS.
        else if input.happened(&InputEvent::SubPanelTTS) {
            let bpm = state.music.bpm;
            let cursor = text.get_time(&state.time.cursor, bpm);
            let playback = text.get_time(&state.time.playback, bpm);
            let mode = text.get_edit_mode(&EDIT_MODES[state.time.mode.get()]);
            let s = get_tooltip_with_values(
                "TIME_TTS",
                &[
                    InputEvent::TimeCursorLeft,
                    InputEvent::TimeCursorRight,
                    InputEvent::TimeCursorStart,
                    InputEvent::TimeCursorEnd,
                    InputEvent::TimePlaybackLeft,
                    InputEvent::TimePlaybackRight,
                    InputEvent::TimePlaybackStart,
                    InputEvent::TimePlaybackEnd,
                    InputEvent::PianoRollCycleMode,
                ],
                &[&cursor, &playback, &mode],
                input,
                text,
            );
            tts.say(&s);
            None
        }
        // Move the cursor.
        else if input.happened(&InputEvent::TimeCursorStart) {
            let s0 = state.clone();
            state.time.cursor = Fraction::zero();
            Some(UndoRedoState::from((s0, state)))
        } else if input.happened(&InputEvent::TimeCursorEnd) {
            let s0 = state.clone();
            state.time.cursor = Time::get_end(state);
            Some(UndoRedoState::from((s0, state)))
        } else if input.happened(&InputEvent::TimeCursorLeft) {
            let s0 = state.clone();
            state.time.cursor = self.get_left(&state.time.cursor, state);
            Some(UndoRedoState::from((s0, state)))
        } else if input.happened(&InputEvent::TimeCursorRight) {
            let mode = EDIT_MODES[state.view.mode.get()];
            let s0 = state.clone();
            let dt = self.deltas.get_dt(&mode, &state.input);
            state.time.cursor += dt;
            Some(UndoRedoState::from((s0, state)))
        }
        // Move the playback.
        else if input.happened(&InputEvent::TimePlaybackStart) {
            let s0 = state.clone();
            state.time.playback = Fraction::zero();
            Some(UndoRedoState::from((s0, state)))
        } else if input.happened(&InputEvent::TimePlaybackEnd) {
            let s0 = state.clone();
            state.time.playback = Time::get_end(state);
            Some(UndoRedoState::from((s0, state)))
        } else if input.happened(&InputEvent::TimePlaybackLeft) {
            let s0 = state.clone();
            state.time.playback = self.get_left(&state.time.playback, state);
            Some(UndoRedoState::from((s0, state)))
        } else if input.happened(&InputEvent::TimeCursorRight) {
            let mode = EDIT_MODES[state.view.mode.get()];
            let s0 = state.clone();
            let dt = self.deltas.get_dt(&mode, &state.input);
            state.time.playback += dt;
            Some(UndoRedoState::from((s0, state)))
        } else {
            None
        }
    }
}
