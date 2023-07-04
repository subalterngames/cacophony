use crate::panel::*;
use crate::{edit_string, set_alphanumeric_input};
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
        // Get the state.
        let snapshot = Snapshot::from_commands(gain0, &gain1);
        // Send the commands.
        conn.send(gain1);
        // Return the state.
        Some(snapshot)
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
        exporter: &mut Exporter,
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
            tts.say(&text.get_with_values(
                "MUSIC_PANEL_STATUS_TTS",
                &[
                    &exporter.metadata.title,
                    &state.time.bpm.to_string(),
                    &conn.state.gain.to_string(),
                ],
            ));
            None
        }
        // Sub-panel TTS.
        else if input.happened(&InputEvent::InputTTS) {
            let scroll = get_tooltip(
                "MUSIC_PANEL_INPUT_TTS",
                &[
                    InputEvent::PreviousMusicPanelField,
                    InputEvent::NextMusicPanelField,
                ],
                input,
                text,
            );
            let tts_text = match state.music_panel_field.get_ref() {
                MusicPanelField::BPM => {
                    let key = if state.input.alphanumeric_input {
                        "MUSIC_PANEL_INPUT_TTS_BPM_ABC123"
                    } else {
                        "MUSIC_PANEL_INPUT_TTS_BPM_NO_ABC123"
                    };
                    let mut s =
                        get_tooltip(key, &[InputEvent::ToggleAlphanumericInput], input, text);
                    if !state.input.alphanumeric_input {
                        s.push(' ');
                        s.push_str(&scroll);
                    }
                    s
                }
                MusicPanelField::Gain => {
                    let mut s = get_tooltip(
                        "MUSIC_PANEL_INPUT_TTS_GAIN",
                        &[InputEvent::DecreaseMusicGain, InputEvent::IncreaseMusicGain],
                        input,
                        text,
                    );
                    s.push(' ');
                    s.push_str(&scroll);
                    s
                }
                MusicPanelField::Name => {
                    let key = if state.input.alphanumeric_input {
                        "MUSIC_PANEL_INPUT_TTS_NAME_ABC123"
                    } else {
                        "MUSIC_PANEL_INPUT_TTS_NAME_NO_ABC123"
                    };
                    let mut s =
                        get_tooltip(key, &[InputEvent::ToggleAlphanumericInput], input, text);
                    if !state.input.alphanumeric_input {
                        s.push(' ');
                        s.push_str(&scroll);
                    }
                    s
                }
            };
            tts.say(&tts_text);
            None
        } else {
            // Field-specific actions.
            match state.music_panel_field.get_ref() {
                // Modify the BPM.
                MusicPanelField::BPM => {
                    if state.input.alphanumeric_input {
                        // Stop editing the BPM.
                        if input.happened(&InputEvent::ToggleAlphanumericInput) {
                            let s0 = state.clone();
                            // Set a default BPM.
                            if state.time.bpm.get_u() == 0 {
                                state.time.bpm = U64orF32::from(DEFAULT_BPM)
                            }
                            // Toggle off alphanumeric input.
                            state.input.alphanumeric_input = false;
                            return Some(Snapshot::from_states(s0, state));
                        }
                        // Edit the BPM.
                        else {
                            let mut bpm = state.time.bpm.get_u();
                            if input.modify_u64(&mut bpm) {
                                let s0: State = state.clone();
                                state.time.bpm = U64orF32::from(bpm);
                                return Some(Snapshot::from_states(s0, state));
                            }
                        }
                    }
                    // Enable input.
                    else if input.happened(&InputEvent::ToggleAlphanumericInput) {
                        return set_alphanumeric_input(state, true);
                    }
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
                    return edit_string(|e| &mut e.metadata.title, input, conn, state, exporter);
                }
            }
            None
        }
    }
}
