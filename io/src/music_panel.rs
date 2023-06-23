use crate::panel::*;
use common::music_panel_field::*;
use common::{U64orF32, DEFAULT_BPM, DEFAULT_MUSIC_NAME, MAX_VOLUME};

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

    /// Increment the current music panel field. Returns a new undo state.
    fn set_field(state: &mut State, up: bool) -> Option<Snapshot> {
        let s0 = state.clone();
        state.music_panel_field.increment(up);
        Some(Snapshot::from_states(s0, state))
    }

    fn enable_alphanumeric_input(state: &mut State) -> Option<Snapshot> {
        let s0 = state.clone();
        state.input.alphanumeric_input = true;
        Some(Snapshot::from_states(s0, state))
    }

    fn get_music_panel_field(state: &State) -> MusicPanelField {
        MUSIC_PANEL_FIELDS[state.music_panel_field.get()]
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
            MusicPanel::set_field(state, true)
        } else if input.happened(&InputEvent::PreviousMusicPanelField) {
            MusicPanel::set_field(state, false)
        }
        // Panel TTS.
        else if input.happened(&InputEvent::StatusTTS) {
            tts.say(&text.get_with_values(
                "MUSIC_PANEL_STATUS_TTS",
                &[
                    &state.music.name,
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
            let tts_text = match Self::get_music_panel_field(state) {
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
            match Self::get_music_panel_field(state) {
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
                        return Self::enable_alphanumeric_input(state);
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
                    if state.input.alphanumeric_input {
                        // Stop editing the name.
                        if input.happened(&InputEvent::ToggleAlphanumericInput) {
                            let s0 = state.clone();
                            if state.music.name.is_empty() {
                                state.music.name = DEFAULT_MUSIC_NAME.to_string();
                            }
                            state.input.alphanumeric_input = false;
                            return Some(Snapshot::from_states(s0, state));
                        } else {
                            let mut name = state.music.name.clone();
                            if input.modify_string_abc123(&mut name) {
                                let s0 = state.clone();
                                state.music.name = name;
                                return Some(Snapshot::from_states(s0, state));
                            }
                        }
                    }
                    // Enable input.
                    else if input.happened(&InputEvent::ToggleAlphanumericInput) {
                        return Self::enable_alphanumeric_input(state);
                    }
                }
            }
            None
        }
    }
}
