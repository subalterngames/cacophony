use crate::panel::*;
use crate::select_track;
use common::open_file::OpenFileType;
use common::{MidiTrack, Paths, SelectMode, MAX_VOLUME};
use std::path::PathBuf;
use text::get_file_name_no_ex;

const TRACK_SCROLL_EVENTS: [InputEvent; 2] = [InputEvent::PreviousTrack, InputEvent::NextTrack];
/// A list of tracks and their parameters.
pub(crate) struct TracksPanel {
    default_soundfont_path: PathBuf,
    tooltips: Tooltips,
}

impl TracksPanel {
    /// Increment or decrement the preset index. Returns a new undo-redo state.
    fn set_preset(channel: u8, conn: &mut Conn, up: bool) -> Option<Snapshot> {
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
        Some(Snapshot::from_commands(c0, c1, conn))
    }

    /// Increment or decrement the bank index, setting the preset index to 0. Returns a new undo-redo state.
    fn set_bank(channel: u8, conn: &mut Conn, up: bool) -> Option<Snapshot> {
        let program = conn.state.programs.get(&channel).unwrap();
        let bank_index_0 = program.bank_index;
        let mut index = Index::new(program.bank_index, program.num_banks);
        index.increment(up);
        let bank_index = index.get();
        // The bank didn't change. Don't reset anything.
        if bank_index == bank_index_0 {
            None
        } else {
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
            Some(Snapshot::from_commands(c0, c1, conn))
        }
    }

    /// Increment or decrement the track gain. Returns a new undo-redo state.
    fn set_gain(state: &mut State, up: bool) -> Option<Snapshot> {
        let s0 = state.clone();
        let track = state.music.get_selected_track_mut().unwrap();
        let mut index = Index::new(track.gain, MAX_VOLUME + 1);
        index.increment(up);
        let gain = index.get();
        track.gain = gain;
        Some(Snapshot::from_states(s0, state))
    }
}

impl Default for TracksPanel {
    fn default() -> Self {
        let default_soundfont_path = Paths::default().default_soundfont_path.clone();
        Self {
            default_soundfont_path,
            tooltips: Tooltips::default(),
        }
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
        _: &mut PathsState,
        _: &mut SharedExporter,
    ) -> Option<Snapshot> {
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
                        None => s.push_str(&text.get("TRACKS_PANEL_STATUS_TTS_NO_SOUNDFONT")),
                    }
                    tts.enqueue(s)
                }
                None => tts.enqueue(text.get("TRACKS_PANEL_STATUS_TTS_NO_SELECTION")),
            }
            None
        }
        // Input TTS.
        else if input.happened(&InputEvent::InputTTS) {
            let mut s = vec![self.tooltips.get_tooltip(
                "TRACKS_PANEL_INPUT_TTS_ADD",
                &[InputEvent::AddTrack],
                input,
                text,
            )];
            // There is a selected track.
            if let Some(track) = state.music.get_selected_track() {
                s.push(self.tooltips.get_tooltip(
                    "TRACKS_PANEL_INPUT_TTS_TRACK_PREFIX_0",
                    &[InputEvent::RemoveTrack],
                    input,
                    text,
                ));
                s.push(self.tooltips.get_tooltip(
                    "TRACKS_PANEL_INPUT_TTS_TRACK_PREFIX_1",
                    &[InputEvent::PreviousTrack, InputEvent::NextTrack],
                    input,
                    text,
                ));
                s.push(self.tooltips.get_tooltip(
                    "TRACKS_PANEL_INPUT_TTS_TRACK_PREFIX_2",
                    &[InputEvent::EnableSoundFontPanel],
                    input,
                    text,
                ));
                // Is there a program?
                if conn.state.programs.get(&track.channel).is_some() {
                    // Preset, bank, gain.
                    s.push(self.tooltips.get_tooltip(
                        "TRACKS_PANEL_INPUT_TTS_TRACK_SUFFIX_0",
                        &[InputEvent::PreviousPreset, InputEvent::NextPreset],
                        input,
                        text,
                    ));
                    s.push(self.tooltips.get_tooltip(
                        "TRACKS_PANEL_INPUT_TTS_TRACK_SUFFIX_1",
                        &[InputEvent::PreviousBank, InputEvent::NextBank],
                        input,
                        text,
                    ));
                    s.push(self.tooltips.get_tooltip(
                        "TRACKS_PANEL_INPUT_TTS_TRACK_SUFFIX_2",
                        &[InputEvent::DecreaseTrackGain, InputEvent::IncreaseTrackGain],
                        input,
                        text,
                    ));
                    // Mute.
                    let mute_key = if track.mute {
                        "TRACKS_PANEL_INPUT_TTS_UNMUTE"
                    } else {
                        "TRACKS_PANEL_INPUT_TTS_MUTE"
                    };
                    s.push(
                        self.tooltips
                            .get_tooltip(mute_key, &[InputEvent::Mute], input, text),
                    );
                    // Solo.
                    let solo_key = if track.solo {
                        "TRACKS_PANEL_INPUT_TTS_UNSOLO"
                    } else {
                        "TRACKS_PANEL_INPUT_TTS_SOLO"
                    };
                    s.push(
                        self.tooltips
                            .get_tooltip(solo_key, &[InputEvent::Solo], input, text),
                    );
                }
                // Say it.
                tts.enqueue(s);
                None
            } else {
                tts.enqueue(s);
                None
            }
        }
        // Add a track.
        else if input.happened(&InputEvent::AddTrack) {
            let s0 = state.clone();
            // Get all channels currently being used.
            let track_channels: Vec<u8> =
                state.music.midi_tracks.iter().map(|t| t.channel).collect();
            // Get all available channels and get the minimum availabe channel.
            match (0u8..255u8).filter(|c| !track_channels.contains(c)).min() {
                Some(channel) => {
                    // Deselect.
                    state.select_mode = match &state.select_mode {
                        SelectMode::Single(_) => SelectMode::Single(None),
                        SelectMode::Many(_) => SelectMode::Many(None),
                    };
                    // Set the selection.
                    state.music.selected = Some(state.music.midi_tracks.len());
                    // Add a track.
                    state.music.midi_tracks.push(MidiTrack::new(channel));
                    // Set the soundfont to the default.
                    let c0 = vec![Command::UnsetProgram { channel }];
                    let c1 = vec![Command::LoadSoundFont {
                        channel,
                        path: self.default_soundfont_path.clone(),
                    }];
                    Some(Snapshot::from_states_and_commands(s0, state, c0, c1, conn))
                }
                None => None,
            }
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
                // Deselect.
                state.select_mode = match &state.select_mode {
                    SelectMode::Single(_) => SelectMode::Single(None),
                    SelectMode::Many(_) => SelectMode::Many(None),
                };
                // Remove the track.
                state.music.midi_tracks.retain(|t| t.channel != channel);
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
                        Some(Snapshot::from_states_and_commands(s0, state, c0, c1, conn))
                    }
                    None => Some(Snapshot::from_states(s0, state)),
                }
            } else if input.happened(&InputEvent::EnableSoundFontPanel) {
                return Some(Snapshot::from_io_commands(vec![IOCommand::EnableOpenFile(
                    OpenFileType::SoundFont,
                )]));
            }
            // Select a track.
            else if let Some(snapshot) = select_track(state, input, TRACK_SCROLL_EVENTS) {
                return Some(snapshot);
            }
            // Track-specific operations.
            else {
                let track = state.music.get_selected_track().unwrap();
                let channel = track.channel;
                // Set the program.
                match conn.state.programs.get(&channel) {
                    Some(_) => {
                        if input.happened(&InputEvent::NextPreset) {
                            TracksPanel::set_preset(channel, conn, true)
                        } else if input.happened(&InputEvent::PreviousPreset) {
                            TracksPanel::set_preset(track.channel, conn, false)
                        } else if input.happened(&InputEvent::NextBank) {
                            TracksPanel::set_bank(track.channel, conn, true)
                        } else if input.happened(&InputEvent::PreviousBank) {
                            TracksPanel::set_bank(track.channel, conn, false)
                        } else if input.happened(&InputEvent::IncreaseTrackGain) {
                            TracksPanel::set_gain(state, true)
                        } else if input.happened(&InputEvent::DecreaseTrackGain) {
                            TracksPanel::set_gain(state, false)
                        } else if input.happened(&InputEvent::Mute) {
                            let s0 = state.clone();
                            let track = state.music.get_selected_track_mut().unwrap();
                            track.mute = !track.mute;
                            // Un-solo.
                            if track.mute && track.solo {
                                track.solo = false;
                            }
                            Some(Snapshot::from_states(s0, state))
                        } else if input.happened(&InputEvent::Solo) {
                            let s0 = state.clone();
                            let track = state.music.get_selected_track_mut().unwrap();
                            track.solo = !track.solo;
                            // Un-mute.
                            if track.mute && track.solo {
                                track.mute = false;
                            }
                            Some(Snapshot::from_states(s0, state))
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

    fn on_disable_abc123(&mut self, _: &mut State, _: &mut SharedExporter) {}

    fn update_abc123(
        &mut self,
        _: &mut State,
        _: &Input,
        _: &mut SharedExporter,
    ) -> (Option<Snapshot>, bool) {
        (None, false)
    }

    fn allow_alphanumeric_input(&self, _: &State, _: &SharedExporter) -> bool {
        false
    }

    fn allow_play_music(&self) -> bool {
        true
    }
}
