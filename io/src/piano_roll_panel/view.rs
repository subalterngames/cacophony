use crate::get_tooltip_with_values;
use crate::panel::*;
use common::config::parse;
use common::ini::Ini;
use common::Zero;
use common::{fraction_from_str, EditMode, Fraction, EDIT_MODES, MAX_NOTE, MIN_NOTE};

/// The piano roll view sub-pane
pub struct View {
    /// Multiply the beat by this factor to get the quick time.
    quick_time_factor: u32,
    /// In precise mode, move the view left and right by this beat length.
    precise_time: Fraction,
    /// In quick mode, move the viewport up and down by this many half-steps.
    quick_note: u8,
    /// In precise mode, move the view up and down by this many half-steps.
    precise_note: u8,
}

impl View {
    pub fn new(config: &Ini) -> Self {
        let section = config.section(Some("VIEW")).unwrap();
        let quick_time_factor: u32 = parse(section, "quick_time_factor");
        let precise_time: Fraction = fraction_from_str(section.get("precise_time").unwrap());
        let quick_note: u8 = parse(section, "quick_note");
        let precise_note: u8 = parse(section, "precise_note");
        Self {
            quick_time_factor,
            precise_time,
            quick_note,
            precise_note,
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

    /// Returns the delta for moving the viewport left or right.
    fn get_input_time_delta(&self, state: &State) -> Fraction {
        match EDIT_MODES[state.view.mode.get()] {
            EditMode::Normal => state.input.beat,
            EditMode::Quick => state.input.beat * self.quick_time_factor,
            EditMode::Precise => self.precise_time,
        }
    }

    /// Returns the delta for moving the viewport up or down.
    fn get_input_note_delta(&self, state: &State) -> u8 {
        match EDIT_MODES[state.view.mode.get()] {
            EditMode::Normal => 1,
            EditMode::Quick => self.quick_note,
            EditMode::Precise => self.precise_note,
        }
    }
}

impl Panel for View {
    fn update(
        &mut self,
        state: &mut State,
        _: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &Text,
    ) -> Option<UndoRedoState> {
        // Do nothing if there is no track.
        if let None = state.music.selected {
            None
        }
        // Cycle the mode.
        else if input.happened(&InputEvent::ViewCycleMode) {
            let s0 = state.clone();
            state.view.mode.increment(true);
            Some(UndoRedoState::from((s0, state)))
        }
        // Sub-panel TTS.
        else if input.happened(&InputEvent::SubPanelTTS) {
            let mode = EDIT_MODES[state.view.mode.get()];
            let bpm = state.music.bpm;
            let s = get_tooltip_with_values(
                "VIEW_TTS",
                &[
                    InputEvent::ViewUp,
                    InputEvent::ViewDown,
                    InputEvent::ViewLeft,
                    InputEvent::ViewRight,
                    InputEvent::ViewStart,
                    InputEvent::ViewEnd,
                    InputEvent::ViewCycleMode,
                ],
                &[
                    &text.get_time(&state.view.dt[0], bpm),
                    &text.get_time(&state.view.dt[1], bpm),
                    &state.view.dn[0].to_string(),
                    &state.view.dn[1].to_string(),
                    &text.get_edit_mode(&mode),
                ],
                input,
                text,
            );
            tts.say(&s);
            None
        }
        // Move the view to t0.
        else if input.happened(&InputEvent::ViewStart) {
            let s0 = state.clone();
            let dt = View::get_dt(state);
            state.view.dt = [Fraction::zero(), dt];
            Some(UndoRedoState::from((s0, state)))
        }
        // Move the view to t1.
        else if input.happened(&InputEvent::ViewEnd) {
            let dt = View::get_dt(state);
            let track = state.music.get_selected_track().unwrap();
            match track.notes.iter().map(|n| n.start + n.duration).max() {
                // Move the view to the end.
                Some(max) => {
                    let s0 = state.clone();
                    state.view.dt = [max, max + dt];
                    Some(UndoRedoState::from((s0, state)))
                }
                // Move the view one viewport's dt rightwards.
                None => {
                    let s0 = state.clone();
                    state.view.dt = [dt, dt * 2];
                    Some(UndoRedoState::from((s0, state)))
                }
            }
        }
        // Move the view leftwards.
        else if input.happened(&InputEvent::ViewLeft) {
            let s0 = state.clone();
            let dt = self.get_input_time_delta(state);
            let t0 = state.view.dt[0] - dt;
            // Don't go past t=0.
            if t0.is_zero() || t0.is_sign_positive() {
                let t1 = state.view.dt[1] - dt;
                state.view.dt = [t0, t1];
                Some(UndoRedoState::from((s0, state)))
            }
            // Snap to t=0.
            else {
                state.view.dt = [Fraction::zero(), dt];
                Some(UndoRedoState::from((s0, state)))
            }
        }
        // Move the view rightwards.
        else if input.happened(&InputEvent::ViewRight) {
            let s0 = state.clone();
            let dt = self.get_input_time_delta(state);
            let t0 = state.view.dt[0] + dt;
            let t1 = state.view.dt[1] + dt;
            state.view.dt = [t0, t1];
            Some(UndoRedoState::from((s0, state)))
        }
        // Move the view upwards.
        else if input.happened(&InputEvent::ViewUp) {
            let s0 = state.clone();
            let dn = self.get_input_note_delta(state);
            // Don't go past n=1.
            if state.view.dn[0] + dn <= MAX_NOTE {
                let n0 = state.view.dn[0] + dn;
                let n1 = state.view.dn[1] + dn;
                state.view.dn = [n0, n1];
                Some(UndoRedoState::from((s0, state)))
            }
            // Snap to n=1.
            else {
                let dn = View::get_dn(state);
                state.view.dn = [MAX_NOTE, MAX_NOTE - dn];
                Some(UndoRedoState::from((s0, state)))
            }
        }
        // Move the view downwards.
        else if input.happened(&InputEvent::ViewDown) {
            let s0 = state.clone();
            let dn = self.get_input_note_delta(state);
            // Don't go past n=0.
            if state.view.dn[1] - dn >= MIN_NOTE {
                let n0 = state.view.dn[0] - dn;
                let n1 = state.view.dn[1] - dn;
                state.view.dn = [n0, n1];
                Some(UndoRedoState::from((s0, state)))
            }
            // Snap to n=0.
            else {
                let dn = View::get_dn(state);
                state.view.dn = [MIN_NOTE + dn, MIN_NOTE];
                Some(UndoRedoState::from((s0, state)))
            }
        } else {
            None
        }
    }
}
