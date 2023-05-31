use super::*;
use crate::panel::*;
use common::config::parse_fractions;
use common::ini::Ini;
use common::{Fraction, Index, PianoRollMode, Note};

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
    /// A buffer of copied notes.
    copied_notes: Vec<Note>
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
            copied_notes: vec![],
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

    /// Returns the text-to-speech string that will be said if there is no valid track.
    fn tts_no_track(text: &Text) -> String {
        text.get("PIANO_ROLL_PANEL_TTS_NO_TRACK")
    }

    /// Returns the sub-panel corresponding to the current piano roll mode.
    fn get_sub_panel<'a>(&'a self, state: &State) -> &'a dyn PianoRollSubPanel {
        match state.piano_roll_mode {
            PianoRollMode::Edit => &self.edit,
            PianoRollMode::Select => &self.select,
            PianoRollMode::Time => &self.time,
            PianoRollMode::View => &self.view,
        }
    }

    /// Copy the selected notes to the copy buffer.
    fn copy_notes(&mut self, state: &State) {
        if let Some(notes) = state.select_mode.get_notes(&state.music) {
            self.copied_notes = notes.iter().map(|n| *n.clone()).collect()
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
            let s = match state.music.get_selected_track() {
                Some(track) => match conn.state.programs.get(&track.channel) {
                    Some(_) => {
                        // The piano roll mode.
                        let mut s = text.get_with_values(
                            "PIANO_ROLL_PANEL_STATUS_TTS_PIANO_ROLL_MODE",
                            &[&text.get_piano_roll_mode(&state.piano_roll_mode)],
                        );
                        s.push(' ');
                        match state.input.armed {
                            // The beat and the volume.
                            true => {
                                let beat = text.get_fraction_tts(&state.input.beat);
                                let v = state.input.volume.get().to_string();
                                let volume = if state.input.use_volume {
                                    v
                                } else {
                                    text.get_with_values(
                                        "PIANO_ROLL_PANEL_STATUS_TTS_VOLUME",
                                        &[&v],
                                    )
                                };
                                s.push_str(&text.get_with_values(
                                    "PIANO_ROLL_PANEL_STATUS_TTS_ARMED",
                                    &[&beat, &volume],
                                ));
                            }
                            // Not armed.
                            false => s.push_str(&text.get("PIANO_ROLL_PANEL_STATUS_TTS_NOT_ARMED")),
                        }
                        // Piano role mode.
                        s.push(' ');
                        s.push_str(&text.get_with_values(
                            "PIANO_ROLL_PANEL_STATUS_TTS_PIANO_ROLL_MODE",
                            &[&text.get_piano_roll_mode(&state.piano_roll_mode)],
                        ));
                        // Panel-specific status.
                        s.push(' ');
                        s.push_str(&self.get_sub_panel(state).get_status_tts(state, text));
                        s
                    }
                    None => PianoRollPanel::tts_no_track(text),
                },
                None => PianoRollPanel::tts_no_track(text),
            };
            tts.say(&s);
            None
        }
        // Copy notes.
        else if input.happened(&InputEvent::CopyNotes) {
            self.copy_notes(state);
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
