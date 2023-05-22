use crate::panel::*;

const MAX_GAIN: usize = 127;

pub(crate) struct MusicPanel {
    /// The text-to-speech string for the panel.
    panel_tts: String,
}

impl MusicPanel {
    pub fn new(text: &Text) -> Self {
        let panel_tts = text.get("MUSIC_PANEL_TTS");
        Self { panel_tts }
    }

    /// Increment the current gain. Returns a new undo state.
    fn set_gain(conn: &mut Conn, up: bool) -> UndoRedoState {
        // Get undo commands.
        let gain0 = vec![Command::SetGain {
            gain: conn.state.gain,
        }];
        // Increment/decrement the gain.
        let mut index = Index::new(conn.state.gain as usize, MAX_GAIN);
        index.increment(up);
        let gain = index.get() as u8;
        let gain1 = vec![Command::SetGain { gain }];
        // Get the state.
        let undo_redo = UndoRedoState::from((gain0, &gain1));
        // Send the commands.
        conn.send(gain1);
        // Return the state.
        undo_redo
    }

    /// Increment the current music panel field. Returns a new undo state.
    fn set_field(state: &mut State, up: bool) -> UndoRedoState {
        let s0 = state.clone();
        state.music_panel_field.increment(up);
        UndoRedoState::from((s0, state))
    }
}

impl Panel for MusicPanel {
    fn update(
        &mut self,
        state: &mut State,
        conn: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &Text,
    ) -> (Option<UndoRedoState>, IOCommands) {
        // Cycle fields.
        if input.happened(&InputEvent::NextMusicPanelField) {
            return (Some(MusicPanel::set_field(state, true)), None);
        } else if input.happened(&InputEvent::PreviousMusicPanelField) {
            return (Some(MusicPanel::set_field(state, false)), None);
        }
        // Panel TTS.
        else if input.happened(&InputEvent::PanelTTS) {
            tts.say(&self.panel_tts);
        }
        // Sub-panel TTS.
        else if input.happened(&InputEvent::SubPanelTTS) {
            let tts_text = match state.get_music_panel_field() {
                MusicPanelField::BPM => {
                    text.get_with_values("MUSIC_PANEL_TTS_BPM", &[&state.music.bpm.to_string()])
                }
                MusicPanelField::Gain => get_tooltip_with_values(
                    "MUSIC_PANEL_TTS_GAIN",
                    &[InputEvent::DecreaseMusicGain, InputEvent::IncreaseMusicGain],
                    &[&conn.state.gain.to_string()],
                    input,
                    text,
                ),
                MusicPanelField::Name => {
                    text.get_with_values("MUSIC_PANEL_TTS_NAME", &[&state.music.name])
                }
            };
            tts.say(&tts_text);
        }
        // Field-specific actions.
        match state.get_music_panel_field() {
            // Modify the BPM.
            MusicPanelField::BPM => {
                let mut bpm = state.music.bpm;
                if input.modify_u32(&mut bpm) {
                    let s0 = state.clone();
                    state.music.bpm = bpm;
                    return (Some(UndoRedoState::from((s0, state))), None);
                }
            }
            // Set the gain.
            MusicPanelField::Gain => {
                if input.happened(&InputEvent::DecreaseMusicGain) {
                    return (Some(MusicPanel::set_gain(conn, false)), None);
                } else if input.happened(&InputEvent::IncreaseMusicGain) {
                    return (Some(MusicPanel::set_gain(conn, true)), None);
                }
            }
            // Modify the name.
            MusicPanelField::Name => {
                let mut name = state.music.name.clone();
                if input.modify_string_abc123(&mut name) {
                    let s0 = state.clone();
                    state.music.name = name;
                    return (Some(UndoRedoState::from((s0, state))), None);
                }
            }
        }
        (None, None)
    }
}
