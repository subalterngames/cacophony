use super::{
    get_cycle_edit_mode_input_tts, get_edit_mode_status_tts, EditModeDeltas, PianoRollSubPanel,
};
use crate::panel::*;
use common::sizes::get_viewport_size;
use ini::Ini;
use text::Tooltips;

/// The piano roll view sub-panel.
pub(super) struct View {
    /// Time values and deltas.
    deltas: EditModeDeltas,
    /// The default viewport dt.
    dt_0: u64,
    tooltips: Tooltips,
}

impl View {
    pub fn new(config: &Ini) -> Self {
        let viewport_size = get_viewport_size(config);
        let dt_0 = viewport_size[0] as u64;
        Self {
            deltas: EditModeDeltas::new(config),
            dt_0,
            tooltips: Tooltips::default(),
        }
    }

    /// Increment or decrement the top note of the view.
    fn set_top_note_by(&self, state: &mut State, add: bool) -> Option<Snapshot> {
        let mode = state.view.mode.get();
        Some(Snapshot::from_state(
            |s| s.view.set_top_note_by(self.deltas.get_dn(&mode), add),
            state,
        ))
    }

    /// Increment or decrement the start time.
    fn set_start_time_by(&self, state: &mut State, add: bool) -> Option<Snapshot> {
        let s0 = state.clone();
        state.view.set_start_time_by(
            self.deltas.get_dt(state.view.mode.get_ref(), &state.input),
            add,
        );
        Some(Snapshot::from_states(s0, state))
    }

    /// Zoom in or out.
    fn zoom(&self, state: &mut State, zoom_in: bool) -> Option<Snapshot> {
        let s0 = state.clone();
        state.view.zoom(zoom_in);
        Some(Snapshot::from_states(s0, state))
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
        _: &mut PathsState,
    ) -> Option<Snapshot> {
        // Do nothing if there is no track.
        if state.music.selected.is_none() {
            None
        }
        // Cycle the mode.
        else if input.happened(&InputEvent::PianoRollCycleMode) {
            Some(Snapshot::from_state(
                |s| s.view.mode.index.increment(true),
                state,
            ))
        }
        // Move the view to t0.
        else if input.happened(&InputEvent::ViewStart) {
            Some(Snapshot::from_state_value(
                |s| &mut s.view.dt,
                [0, state.view.get_dt()],
                state,
            ))
        }
        // Move the view to t1.
        else if input.happened(&InputEvent::ViewEnd) {
            let dt = state.view.get_dt();
            let track = state.music.get_selected_track().unwrap();
            match track.get_end() {
                // Move the view to the end.
                Some(max) => Some(Snapshot::from_state_value(
                    |s| &mut s.view.dt,
                    [max, max + dt],
                    state,
                )),
                // Move the view one viewport's dt rightwards.
                None => Some(Snapshot::from_state_value(
                    |s| &mut s.view.dt,
                    [dt, dt * 2],
                    state,
                )),
            }
        }
        // Move the view leftwards.
        else if input.happened(&InputEvent::ViewLeft) {
            self.set_start_time_by(state, false)
        }
        // Move the view rightwards.
        else if input.happened(&InputEvent::ViewRight) {
            self.set_start_time_by(state, true)
        }
        // Move the view upwards.
        else if state.view.single_track && input.happened(&InputEvent::ViewUp) {
            self.set_top_note_by(state, true)
        }
        // Move the view downwards.
        else if state.view.single_track && input.happened(&InputEvent::ViewDown) {
            self.set_top_note_by(state, false)
        }
        // Zoom in.
        else if input.happened(&InputEvent::ViewZoomIn) {
            self.zoom(state, true)
        }
        // Zoom out.
        else if input.happened(&InputEvent::ViewZoomOut) {
            self.zoom(state, false)
        }
        // Zoom default.
        else if input.happened(&InputEvent::ViewZoomDefault) {
            Some(Snapshot::from_state_value(
                |s| &mut s.view.dt,
                [state.view.dt[0], state.view.dt[0] + self.dt_0],
                state,
            ))
        } else {
            None
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

impl PianoRollSubPanel for View {
    fn get_status_tts(&mut self, state: &State, text: &Text) -> Vec<TtsString> {
        let mut s = vec![TtsString::from(text.get_with_values(
            "PIANO_ROLL_PANEL_STATUS_TTS_VIEW",
            &[
                &text.get_ppq_tts(&state.view.dt[0]),
                &text.get_ppq_tts(&state.view.dt[1]),
                text.get_note_name(state.view.dn[0]),
                text.get_note_name(state.view.dn[1]),
            ],
        ))];
        s.push(get_edit_mode_status_tts(state.view.mode.get_ref(), text));
        s
    }

    fn get_input_tts(&mut self, state: &State, input: &Input, text: &Text) -> Vec<TtsString> {
        let mut s = if state.view.single_track {
            vec![self.tooltips.get_tooltip(
                "PIANO_ROLL_PANEL_INPUT_TTS_VIEW_SINGLE_TRACK_0",
                &[
                    InputEvent::ViewUp,
                    InputEvent::ViewDown,
                    InputEvent::ViewLeft,
                    InputEvent::ViewRight,
                ],
                input,
                text,
            )]
        } else {
            vec![
                self.tooltips.get_tooltip(
                    "PIANO_ROLL_PANEL_INPUT_TTS_VIEW_MULTI_TRACK_0",
                    &[InputEvent::ViewLeft, InputEvent::ViewRight],
                    input,
                    text,
                ),
                self.tooltips.get_tooltip(
                    "PIANO_ROLL_PANEL_INPUT_TTS_VIEW_MULTI_TRACK_1",
                    &[InputEvent::ViewStart, InputEvent::ViewEnd],
                    input,
                    text,
                ),
            ]
        };
        s.append(&mut vec![
            self.tooltips.get_tooltip(
                "PIANO_ROLL_PANEL_INPUT_TTS_VIEW_SINGLE_TRACK_1",
                &[InputEvent::ViewStart, InputEvent::ViewEnd],
                input,
                text,
            ),
            self.tooltips.get_tooltip(
                "PIANO_ROLL_PANEL_INPUT_TTS_VIEW_SINGLE_TRACK_2",
                &[InputEvent::ViewZoomIn, InputEvent::ViewZoomOut],
                input,
                text,
            ),
            self.tooltips.get_tooltip(
                "PIANO_ROLL_PANEL_INPUT_TTS_VIEW_SINGLE_TRACK_3",
                &[InputEvent::ViewZoomDefault],
                input,
                text,
            ),
        ]);
        s.push(get_cycle_edit_mode_input_tts(
            &mut self.tooltips,
            &state.view.mode,
            input,
            text,
        ));
        s
    }
}
