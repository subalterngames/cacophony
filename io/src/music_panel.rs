use crate::abc123::{abc123_shared_exporter, abc123_state};
use crate::panel::*;
use common::music_panel_field::*;
use common::{U64orF32, DEFAULT_BPM, MAX_VOLUME};

const MAX_GAIN: usize = MAX_VOLUME as usize;

/// Set global music values.
pub(crate) struct MusicPanel {}

impl MusicPanel {
    /// Increment the current gain. Returns a new undo state.
    fn set_gain(conn: &mut Conn, up: bool) -> Option<Snapshot> {
        // Get undo commands.
        let gain0 = vec![Command::SetGain {
            gain: conn.state.gain,
        }];
        // Increment/decrement the gain.
        let mut index = Index::new(conn.state.gain as usize, MAX_GAIN);
        index.increment(up);
        let gain = index.get() as u8;
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
        text: &mut Text,
        _: &mut PathsState,
        exporter: &mut SharedExporter,
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
            let ex = exporter.lock();
            tts.enqueue(text.get_with_values(
                "MUSIC_PANEL_STATUS_TTS",
                &[
                    &ex.metadata.title,
                    &state.time.bpm.to_string(),
                    &conn.state.gain.to_string(),
                ],
            ));
            None
        }
        // Sub-panel TTS.
        else if input.happened(&InputEvent::InputTTS) {
            let scroll = text.get_tooltip(
                "MUSIC_PANEL_INPUT_TTS",
                &[
                    InputEvent::PreviousMusicPanelField,
                    InputEvent::NextMusicPanelField,
                ],
                input,
            );
            let tts_strings = match state.music_panel_field.get_ref() {
                MusicPanelField::BPM => {
                    let key = if state.input.alphanumeric_input {
                        "MUSIC_PANEL_INPUT_TTS_BPM_ABC123"
                    } else {
                        "MUSIC_PANEL_INPUT_TTS_BPM_NO_ABC123"
                    };
                    let mut tts_strings = vec![];
                    tts_strings.push(text.get_tooltip(
                        key,
                        &[InputEvent::ToggleAlphanumericInput],
                        input,
                    ));
                    if !state.input.alphanumeric_input {
                        tts_strings.push(scroll);
                    }
                    tts_strings
                }
                MusicPanelField::Gain => {
                    vec![
                        text.get_tooltip(
                            "MUSIC_PANEL_INPUT_TTS_GAIN",
                            &[InputEvent::DecreaseMusicGain, InputEvent::IncreaseMusicGain],
                            input,
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
                    let mut tts_strings =
                        vec![text.get_tooltip(key, &[InputEvent::ToggleAlphanumericInput], input)];
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
                MusicPanelField::BPM => {
                    return abc123_state(
                        |s| &mut s.time.bpm,
                        state,
                        input,
                        U64orF32::from(DEFAULT_BPM),
                    );
                }
                // Set the gain.
                MusicPanelField::Gain => {
                    if input.happened(&InputEvent::DecreaseMusicGain) {
                        return MusicPanel::set_gain(conn, false);
                    } else if input.happened(&InputEvent::IncreaseMusicGain) {
                        return MusicPanel::set_gain(conn, true);
                    }
                }
                // Modify the name.
                MusicPanelField::Name => {
                    return abc123_shared_exporter(
                        |e| &mut e.metadata.title,
                        state,
                        input,
                        exporter,
                        "My Music".to_string(),
                    );
                }
            }
            None
        }
    }
}
