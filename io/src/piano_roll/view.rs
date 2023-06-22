use super::{
    get_cycle_edit_mode_input_tts, get_edit_mode_status_tts, EditModeDeltas, PianoRollSubPanel,
};
use crate::panel::*;
use common::config::parse_ppq;
use common::ini::Ini;
use common::sizes::get_viewport_size;
use common::{EditMode, EDIT_MODES, MAX_NOTE, MIN_NOTE};

/// The piano roll view sub-pane
pub(super) struct View {
    /// Time values and deltas.
    deltas: EditModeDeltas,
    /// The default viewport dt.
    dt_0: u64,
    /// The minimum viewport dt.
    min_dt: u64,
    /// The maximum viewport dt.
    max_dt: u64,
    /// In normal mode, zoom in by this factor.
    normal_zoom: f32,
    /// In quick mode, zoom in by this factor.
    quick_zoom: f32,
    /// In precise mode, zoom in by this factor.
    precise_zoom: f32,
}

impl View {
    pub fn new(config: &Ini) -> Self {
        let section = config.section(Some("PIANO_ROLL")).unwrap();
        let normal_zoom = parse_ppq(section, "normal_zoom") as f32;
        let quick_zoom = parse_ppq(section, "quick_zoom") as f32;
        let precise_zoom = parse_ppq(section, "precise_zoom") as f32;
        let viewport_size = get_viewport_size(config);
        let dt_0 = viewport_size[0] as u64;
        Self {
            deltas: EditModeDeltas::new(config),
            min_dt: 1,
            max_dt: 96000,
            normal_zoom,
            quick_zoom,
            precise_zoom,
            dt_0,
        }
    }

    /// Returns the delta from the viewport's t1 to its t0.
    fn get_dt(state: &State) -> u64 {
        state.view.dt[1] - state.view.dt[0]
    }

    /// Returns the delta from the viewport's n1 to its n0.
    fn get_dn(state: &State) -> u8 {
        state.view.dn[0] - state.view.dn[1]
    }

    /// Zoom in or out.
    fn zoom(&self, state: &mut State, zoom_in: bool) -> Option<Snapshot> {
        // Get the current dt.
        let dt = Self::get_dt(state) as f32;
        // Get the zoom factor.
        let dz = match &EDIT_MODES[state.view.mode.get()] {
            EditMode::Normal => self.normal_zoom,
            EditMode::Quick => self.quick_zoom,
            EditMode::Precise => self.precise_zoom,
        };
        // Apply the zoom factor.
        let dt = ((if zoom_in { dt * dz } else { dt / dz }).ceil() as u64)
            .clamp(self.min_dt, self.max_dt);
        // Set the viewport.
        let s0 = state.clone();
        state.view.dt = [state.view.dt[0], state.view.dt[0] + dt];
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
            let s0 = state.clone();
            state.view.mode.increment(true);
            Some(Snapshot::from_states(s0, state))
        }
        // Move the view to t0.
        else if input.happened(&InputEvent::ViewStart) {
            let s0 = state.clone();
            let dt = View::get_dt(state);
            state.view.dt = [0, dt];
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
            state
                .view
                .set_start_time_by(self.deltas.get_dt(&mode, &state.input), false);
            Some(Snapshot::from_states(s0, state))
        }
        // Move the view rightwards.
        else if input.happened(&InputEvent::ViewRight) {
            let mode = EDIT_MODES[state.time.mode.get()];
            let s0 = state.clone();
            state
                .view
                .set_start_time_by(self.deltas.get_dt(&mode, &state.input), true);
            Some(Snapshot::from_states(s0, state))
        }
        // Move the view upwards.
        else if state.view.single_track && input.happened(&InputEvent::ViewUp) {
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
        else if state.view.single_track && input.happened(&InputEvent::ViewDown) {
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
        }
        // Zoom in.
        else if state.view.single_track && input.happened(&InputEvent::ViewZoomIn) {
            self.zoom(state, true)
        }
        // Zoom out.
        else if state.view.single_track && input.happened(&InputEvent::ViewZoomOut) {
            self.zoom(state, false)
        }
        // Zoom default.
        else if state.view.single_track && input.happened(&InputEvent::ViewZoomDefault) {
            let s0 = state.clone();
            state.view.dt = [state.view.dt[0], state.view.dt[0] + self.dt_0];
            Some(Snapshot::from_states(s0, state))
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
                &text.get_ppq_tts(&state.view.dt[0]),
                &text.get_ppq_tts(&state.view.dt[1]),
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
        let mut s = if state.view.single_track {
            get_tooltip(
                "PIANO_ROLL_PANEL_INPUT_TTS_VIEW_SINGLE_TRACK",
                &[
                    InputEvent::ViewUp,
                    InputEvent::ViewDown,
                    InputEvent::ViewLeft,
                    InputEvent::ViewRight,
                    InputEvent::ViewStart,
                    InputEvent::ViewEnd,
                    InputEvent::ViewZoomIn,
                    InputEvent::ViewZoomOut,
                    InputEvent::ViewZoomDefault,
                ],
                input,
                text,
            )
        } else {
            get_tooltip(
                "PIANO_ROLL_PANEL_INPUT_TTS_VIEW_MULTI_TRACK",
                &[
                    InputEvent::ViewLeft,
                    InputEvent::ViewRight,
                    InputEvent::ViewStart,
                    InputEvent::ViewEnd,
                ],
                input,
                text,
            )
        };
        s.push(' ');
        s.push_str(&get_cycle_edit_mode_input_tts(
            &state.view.mode,
            input,
            text,
        ));
        s
    }
}
