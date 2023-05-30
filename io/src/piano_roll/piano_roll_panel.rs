use super::*;
use crate::panel::*;
use common::config::parse_fractions;
use common::ini::Ini;
use common::{Fraction, Index, PianoRollMode};

/// The piano roll.
/// This is divided into different "modes" for convenience, where each mode is actually a panel.
pub struct PianoRollPanel {
    /// The edit mode.
    edit: Edit,
    /// The select mode.
    select: Select,
    /// The time mode.
    time: Time,
    /// The view mode.
    view: View,
    /// The beats that we can potentially input.
    beats: Vec<Fraction>,
    /// The index of the current beat.
    beat: Index,
}

impl PianoRollPanel {
    pub fn new(beat: &Fraction, config: &Ini) -> Self {
        let edit = Edit::new(config);
        let select = Select {};
        let time = Time::new(config);
        let view = View::new(config);
        // Load the beats.
        let section = config.section(Some("PIANO_ROLL")).unwrap();
        let mut beats = parse_fractions(section, "beats");
        // Is the input beat in the list?
        let beat_index = match beats.iter().position(|b| b == beat) {
            Some(position) => position,
            None => {
                beats.push(*beat);
                beats.len() - 1
            }
        };
        let beat = Index::new(beat_index, beats.len());
        Self {
            edit,
            select,
            time,
            view,
            beats,
            beat,
        }
    }

    /// Set the input beat.
    fn set_input_beat(&mut self, up: bool, state: &mut State) -> Option<UndoRedoState> {
        let s0 = state.clone();
        // Increment the beat.
        self.beat.increment(up);
        // Set the input beat.
        state.input.beat = self.beats[self.beat.get()];
        Some(UndoRedoState::from((s0, state)))
    }

    /// Set the piano roll mode.
    fn set_mode(mode: PianoRollMode, state: &mut State) -> Option<UndoRedoState> {
        let s0 = state.clone();
        state.piano_roll_mode = mode;
        Some(UndoRedoState::from((s0, state)))
    }

    /// Say this if there is no valid track.
    fn tts_no_track(tts: &mut TTS, text: &Text) {
        tts.say(&text.get("PIANO_ROLL_PANEL_TTS_NO_TRACK"))
    }

    fn get_sub_panel<'a>(&'a self, state: &State) -> &'a dyn PianoRollSubPanel {
        match state.piano_roll_mode {
            PianoRollMode::Edit => &self.edit,
            PianoRollMode::Select => &self.select,
            PianoRollMode::Time => &self.time,
            PianoRollMode::View => &self.view,
        }
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
        if state.music.selected.is_none() {
            None
        }
        // Status TTS.
        else if input.happened(&InputEvent::StatusTTS) {
            match state.music.get_selected_track() {
                Some(track) => match conn.state.programs.get(&track.channel) {
                    Some(program) => {
                        panic!("TODO")
                    },
                    None => PianoRollPanel::tts_no_track(tts, text)
                }
                None => PianoRollPanel::tts_no_track(tts, text)
            };
            None
        }
        // Toggle arm.
        else if input.happened(&InputEvent::Arm) {
            let s0 = state.clone();
            state.input.armed = !state.input.armed;
            Some(UndoRedoState::from((s0, state)))
        }
        // Set the input beat.
        else if input.happened(&InputEvent::InputBeatLeft) {
            self.set_input_beat(false, state)
        } else if input.happened(&InputEvent::InputBeatRight) {
            self.set_input_beat(true, state)
        }
        // Set the mode.
        else if input.happened(&InputEvent::PianoRollSetEdit) {
            PianoRollPanel::set_mode(PianoRollMode::Edit, state)
        } else if input.happened(&InputEvent::PianoRollSetSelect) {
            PianoRollPanel::set_mode(PianoRollMode::Select, state)
        } else if input.happened(&InputEvent::PianoRollSetTime) {
            PianoRollPanel::set_mode(PianoRollMode::Time, state)
        } else if input.happened(&InputEvent::PianoRollSetView) {
            PianoRollPanel::set_mode(PianoRollMode::View, state)
        }
        // Sub-panel actions.
        else {
            let mode = state.piano_roll_mode;
            match mode {
                PianoRollMode::Edit => self.edit.update(state, conn, input, tts, text),
                PianoRollMode::Select => self.select.update(state, conn, input, tts, text),
                PianoRollMode::Time => self.time.update(state, conn, input, tts, text),
                PianoRollMode::View => self.view.update(state, conn, input, tts, text),
            }
        }
    }
}
