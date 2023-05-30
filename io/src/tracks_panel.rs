use crate::{panel::*, tooltip::get_tooltip, OpenFileType};
use common::MidiTrack;
use text::get_file_name_no_ex;

pub(crate) struct TracksPanel {}

impl TracksPanel {
    /// Increment or decrement the preset index. Returns a new undo-redo state.
    fn set_preset(channel: u8, conn: &mut Conn, up: bool) -> UndoRedoState {
        let program = conn.state.programs.get(&channel).unwrap();
        let mut index = Index::new(program.preset_index, program.num_presets);
        index.increment(up);
        let preset_index = index.get();
        let path = program.path.clone();
        let c0 = vec![Command::SetProgram {
            channel,
            path: path.clone(),
            bank_index: program.bank_index,
            preset_index: program.preset_index,
        }];
        let c1 = vec![Command::SetProgram {
            channel,
            path,
            bank_index: program.bank_index,
            preset_index,
        }];
        let undo = UndoRedoState::from((c0, &c1));
        conn.send(c1);
        undo
    }

    /// Increment or decrement the bank index, setting the preset index to 0. Returns a new undo-redo state.
    fn set_bank(channel: u8, conn: &mut Conn, up: bool) -> UndoRedoState {
        let program = conn.state.programs.get(&channel).unwrap();
        let mut index = Index::new(program.bank_index, program.num_banks);
        index.increment(up);
        let bank_index = index.get();
        let path = program.path.clone();
        let c0 = vec![Command::SetProgram {
            channel,
            path: path.clone(),
            bank_index: program.bank_index,
            preset_index: program.preset_index,
        }];
        let c1 = vec![Command::SetProgram {
            channel,
            path,
            bank_index,
            preset_index: 0,
        }];
        let undo = UndoRedoState::from((c0, &c1));
        conn.send(c1);
        undo
    }

    /// Increment or decrement the track gain. Returns a new undo-redo state.
    fn set_gain(state: &mut State, up: bool) -> UndoRedoState {
        let s0 = state.clone();
        let track = state.music.get_selected_track_mut().unwrap();
        let mut index = Index::new(track.gain as usize, 127);
        index.increment(up);
        let gain = index.get() as u8;
        track.gain = gain;
        UndoRedoState::from((s0, state))
    }
}

impl Panel for TracksPanel {
    fn update(
        &mut self,
        state: &mut State,
        conn: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &Text,
    ) -> Option<UndoRedoState> {
        // Status TTS.
        if input.happened(&InputEvent::StatusTTS) {
            match state.music.get_selected_track() {
                Some(track) => {
                    // Track ? is selected.
                    let mut s = text.get_with_values(
                        "TRACKS_PANEL_STATUS_TTS_PREFIX",
                        &[&track.channel.to_string()],
                    );
                    s.push(' ');
                    // Is there a SoundFont?
                    match conn.state.programs.get(&track.channel) {
                        // Track staus.
                        Some(program) => {
                            s.push_str(&text.get_with_values(
                                "TRACKS_PANEL_STATUS_TTS_SOUNDFONT",
                                &[
                                    &program.preset_name,
                                    &program.bank.to_string(),
                                    &track.gain.to_string(),
                                    &get_file_name_no_ex(&program.path),
                                ],
                            ));
                            // Muted.
                            if track.mute {
                                s.push(' ');
                                s.push_str(&text.get("TRACKS_PANEL_STATUS_TTS_MUTED"))
                            }
                            // Soloed.
                            if track.solo {
                                s.push(' ');
                                s.push_str(&text.get("TRACKS_PANEL_STATUS_TTS_SOLOED"))
                            }
                        }
                        // No SoundFont.
                        None => s.push_str(&"TRACKS_PANEL_STATUS_TTS_NO_SOUNDFONT"),
                    }
                    tts.say(&s)
                }
                None => tts.say(&text.get("TRACKS_PANEL_STATUS_TTS_NO_SELECTION")),
            }
            None
        }
        // Input TTS.
        else if input.happened(&InputEvent::InputTTS) {
            let mut s = get_tooltip("TRACKS_PANEL_INPUT_TTS_ADD", &[InputEvent::AddTrack], input, text);
            // There is a selected track.
            if let Some(track) = state.music.get_selected_track() {
                // Remove the track.
                s.push(' ');
                s.push_str(&get_tooltip_with_values("TRACKS_PANEL_INPUT_TTS_TRACK_PREFIX", &[InputEvent::RemoveTrack, InputEvent::PreviousTrack, InputEvent::NextTrack, InputEvent::EnableSoundFontPanel], &[&track.channel.to_string()], input, text));
                s.push(' ');
                // Is there a program?
                match conn.state.programs.get(&track.channel) {
                    // Program.
                    Some(program) => {
                        // Preset, bank, gain.
                        s.push_str(&get_tooltip("TRACKS_PANEL_INPUT_TTS_TRACK_SUFFIX", &[InputEvent::PreviousPreset, InputEvent::NextPreset, InputEvent::PreviousBank, InputEvent::NextBank, InputEvent::DecreaseTrackGain, InputEvent::IncreaseTrackGain], input, text));
                        // Mute.
                        s.push(' ');
                        let mute_key = if track.mute {
                            "TRACKS_PANEL_INPUT_TTS_UNMUTE"
                        }
                        else {
                            "TRACKS_PANEL_INPUT_TTS_MUTE"
                        };
                        s.push_str(&get_tooltip(mute_key, &[InputEvent::Mute], input, text));
                        // Solo.
                        s.push(' ');
                        let solo_key = if track.solo {
                            "TRACKS_PANEL_INPUT_TTS_UNSOLO"
                        }
                        else {
                            "TRACKS_PANEL_INPUT_TTS_SOLO"
                        };
                        s.push_str(&get_tooltip(solo_key, &[InputEvent::Solo], input, text));

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
                None
            } else {
                None
            }
        }
        // Add a track.
        else if input.happened(&InputEvent::AddTrack) {
            let s0 = state.clone();
            // Get the next channel.
            let channel = match state.music.midi_tracks.iter().map(|t| t.channel).max() {
                Some(channel) => channel + 1,
                None => 0,
            };
            // Set the selection.
            state.music.selected = Some(state.music.midi_tracks.len());
            // Add a track.
            state.music.midi_tracks.push(MidiTrack::new(channel));
            Some(UndoRedoState::from((s0, state)))
        }
        // There is a selected track.
        else if let Some(selected) = state.music.selected {
            // Remove the selected track.
            if input.happened(&InputEvent::RemoveTrack) {
                let channel = state.music.get_selected_track().unwrap().channel;
                let s0 = state.clone();
                state.music.selected = match state.music.midi_tracks.len() {
                    0 => panic!("Somehow, there are zero tracks. This should never happen."),
                    // There is only one track, so soon there will be none.
                    1 => None,
                    _ => match selected {
                        // First track.
                        0 => Some(0),
                        other => Some(other - 1),
                    },
                };
                // This track has a program that needs to be unset.
                match conn.state.programs.get(&channel) {
                    Some(program) => {
                        // Undo: Set the program.
                        let c0 = vec![Command::SetProgram {
                            channel,
                            path: program.path.clone(),
                            bank_index: program.bank_index,
                            preset_index: program.preset_index,
                        }];
                        let c1 = vec![Command::UnsetProgram { channel }];
                        let undo_redo = UndoRedoState::from((s0, c0, state, &c1));
                        // Remove the program.
                        conn.send(c1);
                        return Some(undo_redo);
                    }
                    None => return Some(UndoRedoState::from((s0, state))),
                }
            } else if input.happened(&InputEvent::EnableSoundFontPanel) {
                return Some(UndoRedoState::from(Some(vec![IOCommand::EnableOpenFile(
                    OpenFileType::SoundFont,
                )])));
            } else {
                let track = state.music.get_selected_track().unwrap();
                let channel = track.channel;
                // Set the program.
                match conn.state.programs.get(&channel) {
                    Some(_) => {
                        if input.happened(&InputEvent::NextPreset) {
                            Some(TracksPanel::set_preset(channel, conn, true))
                        } else if input.happened(&InputEvent::PreviousPreset) {
                            Some(TracksPanel::set_preset(track.channel, conn, false))
                        } else if input.happened(&InputEvent::NextBank) {
                            Some(TracksPanel::set_bank(track.channel, conn, true))
                        } else if input.happened(&InputEvent::PreviousBank) {
                            Some(TracksPanel::set_bank(track.channel, conn, false))
                        } else if input.happened(&InputEvent::IncreaseTrackGain) {
                            Some(TracksPanel::set_gain(state, true))
                        } else if input.happened(&InputEvent::DecreaseTrackGain) {
                            Some(TracksPanel::set_gain(state, false))
                        } else {
                            None
                        }
                    }
                    None => None,
                }
            }
        } else {
            None
        }
    }
}
