use crate::{panel::*, tooltip::get_tooltip};

pub(crate) struct TracksPanel {
    /// The text-to-speech string for the panel if there is not a selected track.
    panel_tts_no_selection: String,
}

impl TracksPanel {
    pub fn new(input: &Input, text: &Text) -> Self {
        // TRACKS_LIST_PANEL_TTS_NO_SELECTION,\2 to add a track.
        let panel_tts_no_selection = get_tooltip(
            "TRACKS_LIST_PANEL_TTS_NO_SELECTION",
            &[InputEvent::AddTrack],
            input,
            text,
        );
        Self {
            panel_tts_no_selection,
        }
    }

    fn get_soundfont_name(path: &str) -> String {
        path.replace('\\', "/")
            .split('/')
            .last()
            .unwrap()
            .split('.')
            .next()
            .unwrap()
            .to_string()
    }
}

impl Panel for TracksPanel {
    fn update(
        &mut self,
        state: &State,
        conn: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &Text,
    ) -> Option<UndoState> {
        // Panel TTS.
        if input.happened(&InputEvent::PanelTTS) {
            match state.music.get_selected_track() {
                Some(track) => {
                    let tts_text = get_tooltip_with_values(
                        "TRACKS_LIST_PANEL_TTS_SELECTION",
                        &[
                            InputEvent::NextTrack,
                            InputEvent::PreviousTrack,
                            InputEvent::AddTrack,
                            InputEvent::RemoveTrack,
                        ],
                        &[&track.channel.to_string()],
                        input,
                        text,
                    );
                    tts.say(&tts_text)
                }
                None => tts.say(&self.panel_tts_no_selection),
            }
            return None;
        }
        // Sub-panel TTS.
        if input.happened(&InputEvent::SubPanelTTS) {
            // There is a selected track.
            if let Some(track) = state.music.get_selected_track() {
                // Is there a program?
                match conn.state.programs.get(&track.channel) {
                    // Program.
                    Some(program) => {
                        // Preset, bank, gain.
                        let mut s = get_tooltip_with_values(
                            "TRACKS_LIST_SUB_PANEL_TTS_SOUNDFONT_0",
                            &[
                                InputEvent::PreviousPreset,
                                InputEvent::NextPreset,
                                InputEvent::PreviousBank,
                                InputEvent::NextBank,
                                InputEvent::DecreaseTrackGain,
                                InputEvent::IncreaseTrackGain,
                            ],
                            &[
                                &program.preset_name,
                                &program.bank.to_string(),
                                &track.gain.to_string(),
                            ],
                            input,
                            text,
                        );
                        // Mute.
                        s.push(' ');
                        let mute_key = if track.mute {
                            "TRACKS_LIST_SUB_PANEL_TTS_MUTED"
                        } else {
                            "TRACKS_LIST_SUB_PANEL_TTS_UNMUTED"
                        };
                        s.push_str(&get_tooltip(mute_key, &[InputEvent::Mute], input, text));
                        // Solo.
                        s.push(' ');
                        let solo_key = if track.solo {
                            "TRACKS_LIST_SUB_PANEL_TTS_SOLOED"
                        } else {
                            "TRACKS_LIST_SUB_PANEL_TTS_NONSOLOED"
                        };
                        s.push_str(&get_tooltip(solo_key, &[InputEvent::Solo], input, text));
                        // SoundFont.
                        s.push(' ');
                        let sf_name = TracksPanel::get_soundfont_name(&program.path);
                        s.push_str(&get_tooltip_with_values(
                            "TRACKS_LIST_SUB_PANEL_TTS_SOUNDFONT_1",
                            &[InputEvent::EnableSoundFontPanel],
                            &[&sf_name],
                            input,
                            text,
                        ));
                        // Say it.
                        tts.say(&s);
                    }
                    // No program.
                    None => {
                        let tts_text = get_tooltip_with_values(
                            "TRACKS_LIST_SUB_PANEL_TTS_NO_SOUNDFONT",
                            &[InputEvent::EnableSoundFontPanel],
                            &[&track.channel.to_string()],
                            input,
                            text,
                        );
                        tts.say(&tts_text);
                    }
                }
                return None;
            }
        }
        None
    }
}
