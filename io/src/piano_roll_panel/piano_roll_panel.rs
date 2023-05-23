use common::ini::Ini;
use common::{PianoRollMode};
use common::hashbrown::HashMap;
use super::*;
use crate::panel::*;

/// The piano roll.
/// This is divided into different "modes" for convenience, where each mode is actually a panel.
pub struct PianoRollPanel {
    edit: Edit,
    select: Select,
    time: Time,
    view: View,
}

impl PianoRollPanel {
    pub fn new(config: &Ini, text: &Text) -> Self {
        let edit = Edit::new(config, text);
        let select = Select{};
        let time = Time::new(config);
        let view = View::new(config);
        Self { edit, select, time, view }
    }

    /// Set the piano roll mode.
    fn set_mode(mode: PianoRollMode, state: &mut State) -> Option<UndoRedoState> {
        let s0 = state.clone();
        state.piano_roll_mode = mode;
        Some(UndoRedoState::from((s0, state)))
    }
}

impl Panel for PianoRollPanel {
    fn update(
            &mut self,
            state: &mut State,
            conn: &mut Conn,
            input: &Input,
            tts: &mut TTS,
            text: &Text,
        ) -> Option<UndoRedoState> {
        if input.happened(&InputEvent::PanelTTS) {
            panic!("TODO")
        }
        // Toggle arm.
        else if input.happened(&InputEvent::Arm) {
            let s0 = state.clone();
            state.input.armed = !state.input.armed;
            Some(UndoRedoState::from((s0, state)))
        }
        // Set the input beat.
        else if input.happened(&InputEvent::InputBeatLeft) {
            panic!("TODO")
        }
        // Set the mode.
        else if input.happened(&InputEvent::PianoRollSetEdit) {
            PianoRollPanel::set_mode(PianoRollMode::Edit, state)
        }
        else if input.happened(&InputEvent::PianoRollSetSelect) {
            PianoRollPanel::set_mode(PianoRollMode::Select, state)
        }
        else if input.happened(&InputEvent::PianoRollSetTime) {
            PianoRollPanel::set_mode(PianoRollMode::Time, state)
        }
        else if input.happened(&InputEvent::PianoRollSetView) {
            PianoRollPanel::set_mode(PianoRollMode::View, state)
        }
        // Sub-panel actions.
        else {
            let mode = state.piano_roll_mode;
            match mode {
                PianoRollMode::Edit => self.edit.update(state, conn, input, tts, text),
                PianoRollMode::Select => self.select.update(state, conn, input, tts, text),
                PianoRollMode::Time => self.time.update(state, conn, input, tts, text),
                PianoRollMode::View => self.view.update(state, conn, input, tts, text)
            }
        }
    }
}