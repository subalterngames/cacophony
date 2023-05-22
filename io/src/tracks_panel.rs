use crate::{panel::*, tooltip::get_tooltip};
use common::open_file::OpenFile;
use common::MidiTrack;
use std::path::PathBuf;
use text::get_file_name_no_ex;

pub(crate) struct TracksPanel {
    /// The text-to-speech string for the panel if there is not a selected track.
    tts_no_selection: String,
}

impl TracksPanel {
    pub fn new(input: &Input, text: &Text) -> Self {
        // TRACKS_LIST_PANEL_TTS_NO_SELECTION,\2 to add a track.
        let tts_no_selection = get_tooltip(
            "TRACKS_LIST_PANEL_TTS_NO_SELECTION",
            &[InputEvent::AddTrack],
            input,
            text,
        );
        Self { tts_no_selection }
    }

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
        paths: &Paths,
    ) -> Option<UndoRedoState> {
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
                None => tts.say(&self.tts_no_selection),
            }
            None
        }
        // Sub-panel TTS.
        else if input.happened(&InputEvent::SubPanelTTS) {
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
                        let sf_name = get_file_name_no_ex(&PathBuf::from(program.path.clone()));
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
                panic!("CLEAR THE STACK! ENABLE");
                state.input.can_undo = false;
                OpenFile::soundfont(paths, state);
                None
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
