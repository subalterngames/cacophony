use crate::panel::*;
use common::MAX_VOLUME;

const MAX_GAIN: usize = MAX_VOLUME as usize;

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
}

impl Panel for MusicPanel {
    fn update(
        &mut self,
        state: &mut State,
        conn: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &Text,
        _: &Paths,
        _: &mut PathsState
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
                    &state.music.bpm.to_string(),
                    &conn.state.gain.to_string(),
                ],
            ));
            None
        }
        // Sub-panel TTS.
        else if input.happened(&InputEvent::InputTTS) {
            let field_key = match state.get_music_panel_field() {
                MusicPanelField::BPM => "BPM",
                MusicPanelField::Gain => "GAIN",
                MusicPanelField::Name => "NAME",
            };
            let field = text.get(field_key);
            let mut s = get_tooltip_with_values(
                "MUSIC_PANEL_INPUT_TTS",
                &[
                    InputEvent::PreviousMusicPanelField,
                    InputEvent::NextMusicPanelField,
                ],
                &[&field],
                input,
                text,
            );
            s.push(' ');
            let tts_text = match state.get_music_panel_field() {
                MusicPanelField::BPM => text.get("MUSIC_PANEL_INPUT_TTS_BPM"),
                MusicPanelField::Gain => get_tooltip(
                    "MUSIC_PANEL_INPUT_TTS_GAIN",
                    &[InputEvent::DecreaseMusicGain, InputEvent::IncreaseMusicGain],
                    input,
                    text,
                ),
                MusicPanelField::Name => text.get("MUSIC_PANEL_INPUT_TTS_NAME"),
            };
            s.push_str(&tts_text);
            tts.say(&s);
            None
        } else {
            // Field-specific actions.
            match state.get_music_panel_field() {
                // Modify the BPM.
                MusicPanelField::BPM => {
                    let mut bpm = state.music.bpm;
                    if input.modify_u32(&mut bpm) {
                        let s0 = state.clone();
                        state.music.bpm = bpm;
                        return Some(Snapshot::from_states(s0, state));
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
                    let mut name = state.music.name.clone();
                    if input.modify_string_abc123(&mut name) {
                        let s0 = state.clone();
                        state.music.name = name;
                        return Some(Snapshot::from_states(s0, state));
                    }
                }
            }
            None
        }
    }
}
