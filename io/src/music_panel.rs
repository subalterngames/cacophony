use crate::abc123::{on_disable_exporter, on_disable_state, update_exporter, update_state};
use crate::panel::*;
use common::music_panel_field::*;
use common::{U64orF32, DEFAULT_BPM, MAX_VOLUME};

/// Set global music values.
#[derive(Default)]
pub(crate) struct MusicPanel {
    tooltips: Tooltips,
}

impl MusicPanel {
    /// Increment the current gain. Returns a new undo state.
    fn set_gain(conn: &mut Conn, up: bool) -> Option<Snapshot> {
        // Get undo commands.
        let gain0 = vec![Command::SetGain {
            gain: conn.state.gain,
        }];
        // Increment/decrement the gain.
        let mut index = Index::new(conn.state.gain, MAX_VOLUME + 1);
        index.increment(up);
        let gain = index.get();
        let gain1 = vec![Command::SetGain { gain }];
        // Send the commands.
        Some(Snapshot::from_commands(gain0, gain1, conn))
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
        _: &mut PathsState,
    ) -> Option<Snapshot> {
        // Cycle fields.
        if input.happened(&InputEvent::NextMusicPanelField) {
            Some(Snapshot::from_state(
                |s| s.music_panel_field.index.increment(true),
                state,
            ))
        } else if input.happened(&InputEvent::PreviousMusicPanelField) {
            Some(Snapshot::from_state(
                |s| s.music_panel_field.index.increment(false),
                state,
            ))
        }
        // Panel TTS.
        else if input.happened(&InputEvent::StatusTTS) {
            tts.enqueue(text.get_with_values(
                "MUSIC_PANEL_STATUS_TTS",
                &[
                    &conn.exporter.metadata.title,
                    &state.time.bpm.to_string(),
                    &conn.state.gain.to_string(),
                ],
            ));
            None
        }
        // Sub-panel TTS.
        else if input.happened(&InputEvent::InputTTS) {
            let scroll = self.tooltips.get_tooltip(
                "MUSIC_PANEL_INPUT_TTS",
                &[
                    InputEvent::PreviousMusicPanelField,
                    InputEvent::NextMusicPanelField,
                ],
                input,
                text,
            );
            let tts_strings = match state.music_panel_field.get_ref() {
                MusicPanelField::BPM => {
                    let key = if state.input.alphanumeric_input {
                        "MUSIC_PANEL_INPUT_TTS_BPM_ABC123"
                    } else {
                        "MUSIC_PANEL_INPUT_TTS_BPM_NO_ABC123"
                    };
                    let mut tts_strings = vec![];
                    tts_strings.push(self.tooltips.get_tooltip(
                        key,
                        &[InputEvent::ToggleAlphanumericInput],
                        input,
                        text,
                    ));
                    if !state.input.alphanumeric_input {
                        tts_strings.push(scroll);
                    }
                    tts_strings
                }
                MusicPanelField::Gain => {
                    vec![
                        self.tooltips.get_tooltip(
                            "MUSIC_PANEL_INPUT_TTS_GAIN",
                            &[InputEvent::DecreaseMusicGain, InputEvent::IncreaseMusicGain],
                            input,
                            text,
                        ),
                        scroll,
                    ]
                }
                MusicPanelField::Name => {
                    let key = if state.input.alphanumeric_input {
                        "MUSIC_PANEL_INPUT_TTS_NAME_ABC123"
                    } else {
                        "MUSIC_PANEL_INPUT_TTS_NAME_NO_ABC123"
                    };
                    let mut tts_strings = vec![self.tooltips.get_tooltip(
                        key,
                        &[InputEvent::ToggleAlphanumericInput],
                        input,
                        text,
                    )];
                    if !state.input.alphanumeric_input {
                        tts_strings.push(scroll);
                    }
                    tts_strings
                }
            };
            tts.enqueue(tts_strings);
            None
        } else {
            // Field-specific actions.
            match state.music_panel_field.get_ref() {
                // Modify the BPM.
                MusicPanelField::BPM => None,
                // Set the gain.
                MusicPanelField::Gain => {
                    if input.happened(&InputEvent::DecreaseMusicGain) {
                        MusicPanel::set_gain(conn, false)
                    } else if input.happened(&InputEvent::IncreaseMusicGain) {
                        MusicPanel::set_gain(conn, true)
                    } else {
                        None
                    }
                }
                // Modify the name.
                MusicPanelField::Name => None,
            }
        }
    }

    fn update_abc123(
        &mut self,
        state: &mut State,
        input: &Input,
        conn: &mut Conn,
    ) -> (Option<Snapshot>, bool) {
        match state.music_panel_field.get_ref() {
            MusicPanelField::BPM => {
                let snapshot = update_state(|s| &mut s.time.bpm, state, input);
                let updated = snapshot.is_some();
                (snapshot, updated)
            }
            MusicPanelField::Gain => (None, false),
            MusicPanelField::Name => (
                None,
                update_exporter(|e| &mut e.metadata.title, input, &mut conn.exporter),
            ),
        }
    }

    fn on_disable_abc123(&mut self, state: &mut State, conn: &mut Conn) {
        match state.music_panel_field.get_ref() {
            MusicPanelField::BPM => {
                on_disable_state(|s| &mut s.time.bpm, state, U64orF32::from(DEFAULT_BPM))
            }
            MusicPanelField::Gain => (),
            MusicPanelField::Name => on_disable_exporter(
                |e| &mut e.metadata.title,
                &mut conn.exporter,
                "My Music".to_string(),
            ),
        }
    }

    fn allow_alphanumeric_input(&self, state: &State, _: &Conn) -> bool {
        match state.music_panel_field.get_ref() {
            MusicPanelField::BPM => true,
            MusicPanelField::Gain => false,
            MusicPanelField::Name => true,
        }
    }

    fn allow_play_music(&self) -> bool {
        true
    }
}
