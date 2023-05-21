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
    fn set_gain(state: &State, conn: &mut Conn, up: bool) -> UndoState {
        let undo = UndoState {
            state: state.clone(),
            commands: vec![Command::SetGain {
                gain: conn.state.gain,
            }],
        };
        let mut index = Index::new(conn.state.gain as usize, MAX_GAIN);
        index.increment(up);
        let gain = index.get() as u8;
        conn.send(vec![Command::SetGain { gain }]);
        undo
    }
}

impl Panel for MusicPanel {
    fn update(
        &mut self,
        state: &State,
        conn: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &Text,
    ) -> Option<UndoState> {
        // Cycle fields.
        if input.happened(&InputEvent::NextMusicPanelField) {
            let mut state = state.clone();
            state.music_panel_field.increment(true);
            return Some(UndoState::from(state));
        } else if input.happened(&InputEvent::PreviousMusicPanelField) {
            let mut state = state.clone();
            state.music_panel_field.increment(false);
            return Some(UndoState::from(state));
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
                    let mut state = state.clone();
                    state.music.bpm = bpm;
                    return Some(UndoState::from(state));
                }
            }
            // Set the gain.
            MusicPanelField::Gain => {
                if input.happened(&InputEvent::DecreaseMusicGain) {
                    return Some(MusicPanel::set_gain(state, conn, false));
                } else if input.happened(&InputEvent::IncreaseMusicGain) {
                    return Some(MusicPanel::set_gain(state, conn, true));
                }
            }
            // Modify the name.
            MusicPanelField::Name => {
                let mut name = state.music.name.clone();
                if input.modify_string_abc123(&mut name) {
                    let mut state = state.clone();
                    state.music.name = name;
                    return Some(UndoState::from(state));
                }
            }
        }
        None
    }
}
