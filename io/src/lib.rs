//! This crate handles essentially all of Cacophony's functionality except the rendering (see the `render` crate).
//!
//! The only public struct is `IO`.
//!
//! Per frame, `IO` listens for user input via an `Input` (see the `input` crate), and then does any of the following:
//!
//! - Update `State` (see the `common` crate), for example add a new track.
//! - Send a list of `Command` to the `Conn` (see the `audio` crate).
//! - Send an internal `IOCommand` to itself.
//! - Play text-to-speech audio (see the `text` crate).
//!
//! The first two options (state and command) will create a copy of the current `State` which will be added to an undo stack.
//! Undoing an action reverts the app to that state, pops it from the undo stack, and pushes it to the redo stack.
//!
//! `IO` divides input listening into discrete panels, e.g. the music panel and the tracks panel.
//! Each panel implements the `Panel` trait.

use audio::exporter::*;
use audio::{Command, CommandsMessage, Conn, ExportState};
use common::hashbrown::HashMap;
use common::ini::Ini;
use common::{
    InputState, MidiTrack, Music, Note, PanelType, Paths, PathsState, SelectMode, State, MAX_VOLUME,
};
use edit::edit_file;
use input::{Input, InputEvent};
use std::path::Path;
use text::{Text, TTS};
mod export_panel;
mod io_command;
mod music_panel;
mod panel;
mod piano_roll;
mod save;
mod snapshot;
mod tracks_panel;
use io_command::IOCommand;
use io_command::IOCommands;
use music_panel::MusicPanel;
mod open_file_panel;
use common::open_file::OpenFileType;
use export_panel::ExportPanel;
use export_settings_panel::ExportSettingsPanel;
use open_file_panel::OpenFilePanel;
use panel::Panel;
use piano_roll::PianoRollPanel;
use save::Save;
use snapshot::Snapshot;
use text::TtsString;
use tracks_panel::TracksPanel;
mod abc123;
mod export_settings_panel;

/// The maximum size of the undo stack.
const MAX_UNDOS: usize = 100;
/// Commands that are queued for export.
type QueuedExportCommands = (CommandsMessage, Option<ExportState>);

pub struct IO {
    /// A stack of snapshots that can be popped to undo an action.
    undo: Vec<Snapshot>,
    /// A stack of snapshots that can be popped to redo an action.
    redo: Vec<Snapshot>,
    /// Top-level text-to-speech lookups.
    tts: HashMap<InputEvent, TtsString>,
    /// The music panel.
    music_panel: MusicPanel,
    /// The tracks panel.
    tracks_panel: TracksPanel,
    /// The open-file panel.
    open_file_panel: OpenFilePanel,
    /// The piano roll panel.
    piano_roll_panel: PianoRollPanel,
    /// The export panel.
    export_panel: ExportPanel,
    /// The export settings panel.
    export_settings_panel: ExportSettingsPanel,
    /// Queued commands that will be used to export audio to multiple files.
    export_queue: Vec<QueuedExportCommands>,
    /// The active panels prior to exporting audio.
    pre_export_panels: Vec<PanelType>,
    /// The index of the focused panel prior to exporting audio.
    pre_export_focus: usize,
}

impl IO {
    pub fn new(config: &Ini, input: &Input, input_state: &InputState, text: &mut Text) -> Self {
        let mut tts = HashMap::new();
        // App TTS.
        let app = text.get_tooltip(
            "APP_TTS",
            &[
                InputEvent::StatusTTS,
                InputEvent::InputTTS,
                InputEvent::AppTTS,
                InputEvent::FileTTS,
                InputEvent::Quit,
                InputEvent::NextPanel,
                InputEvent::PreviousPanel,
                InputEvent::Undo,
                InputEvent::Redo,
                InputEvent::StopTTS,
            ],
            input,
        );
        tts.insert(InputEvent::AppTTS, app);
        // File TTS.
        let file = text.get_tooltip(
            "FILE_TTS",
            &[
                InputEvent::NewFile,
                InputEvent::OpenFile,
                InputEvent::SaveFile,
                InputEvent::SaveFileAs,
                InputEvent::ExportFile,
                InputEvent::EditConfig,
            ],
            input,
        );
        tts.insert(InputEvent::FileTTS, file);
        let music_panel = MusicPanel {};
        let tracks_panel = TracksPanel {};
        let open_file_panel = OpenFilePanel::default();
        let piano_roll_panel = PianoRollPanel::new(&input_state.beat.get_u(), config);
        let export_panel = ExportPanel::default();
        let export_settings_panel = ExportSettingsPanel {};
        Self {
            tts,
            music_panel,
            tracks_panel,
            open_file_panel,
            piano_roll_panel,
            export_panel,
            export_settings_panel,
            redo: vec![],
            undo: vec![],
            export_queue: vec![],
            pre_export_panels: vec![],
            pre_export_focus: 0,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        state: &mut State,
        conn: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &mut Text,
        paths_state: &mut PathsState,
        exporter: &mut Exporter,
    ) -> bool {
        // Quit.
        if input.happened(&InputEvent::Quit) {
            return true;
        }

        // Export multiple files.
        if conn.export_state.is_none() && !self.export_queue.is_empty() {
            // Enable the panel.
            self.export_panel
                .enable(state, &self.pre_export_panels, self.pre_export_focus);
            // Get the commands and state.
            let export_commands = self.export_queue.remove(0);
            // Set the state.
            conn.export_state = export_commands.1;
            // Send the commands.
            conn.send(export_commands.0);
        }

        // Don't do anything while exporting.
        if conn.export_state.is_some() {
            return false;
        }

        // Play notes.
        if !state.input.alphanumeric_input
            && !&input.play_now.is_empty()
            && !state.panels.contains(&PanelType::OpenFile)
        {
            if let Some(track) = state.music.get_selected_track() {
                if conn.state.programs.get(&track.channel).is_some() {
                    let gain = track.gain as f64 / 127.0;
                    // Set the framerate for playback.
                    let mut commands = vec![Command::SetFramerate {
                        framerate: conn.framerate as u32,
                    }];
                    // Get the beat duration.
                    let duration = state
                        .time
                        .ppq_to_samples(state.input.beat.get_u(), conn.framerate);
                    // Play the notes.
                    for note in input.play_now.iter() {
                        // Set the volume.
                        let volume = (note[2] as f64 * gain) as u8;
                        commands.push(Command::NoteOn {
                            channel: track.channel,
                            key: note[1],
                            velocity: volume,
                            duration,
                        });
                    }
                    conn.send(commands);
                }
            }
        }
        // New file.
        if input.happened(&InputEvent::NewFile) {
            paths_state.saves.filename = None;
            state.music = Music::default();
        }
        // Open file.
        else if input.happened(&InputEvent::OpenFile) {
            self.open_file_panel.read_save(state, paths_state);
        }
        // Save file.
        else if input.happened(&InputEvent::SaveFile) {
            match &paths_state.saves.try_get_path() {
                // Save to the existing path,
                Some(path) => {
                    Save::write(
                        &path.with_extension("cac"),
                        state,
                        conn,
                        paths_state,
                        exporter,
                    );
                    state.unsaved_changes = false;
                }
                // Set a new path.
                None => self.open_file_panel.write_save(state, paths_state),
            }
        }
        // Save to a new path.
        else if input.happened(&InputEvent::SaveFileAs) {
            self.open_file_panel.write_save(state, paths_state)
        }
        // Export.
        else if input.happened(&InputEvent::ExportFile) {
            // We aren't exporting already.
            if conn.export_state.is_none() {
                self.open_file_panel.export(state, paths_state, exporter)
            }
        }
        // Open config file.
        else if input.happened(&InputEvent::EditConfig) {
            let paths = Paths::default();
            // Create a user .ini file.
            if !paths.user_ini_path.exists() {
                paths.create_user_config();
            }
            // Edit.
            if edit_file(&paths.user_ini_path).is_ok() {}
        }
        // Undo.
        else if input.happened(&InputEvent::Undo) {
            if let Some(undo) = self.undo.pop() {
                // Get the redo state.
                let redo = Snapshot::from_snapshot(&undo);
                // Assign the undo state to the previous state.
                if let Some(s1) = undo.from_state {
                    *state = s1;
                }
                // Send the commands.
                if let Some(commands) = undo.from_commands {
                    conn.send(commands);
                }
                // Push to the redo stack.
                self.redo.push(redo);
                state.unsaved_changes = true;
            }
        // Redo.
        } else if input.happened(&InputEvent::Redo) {
            if let Some(redo) = self.redo.pop() {
                let undo = Snapshot::from_snapshot(&redo);
                // Assign the redo state to the current state.
                if let Some(s1) = redo.from_state {
                    *state = s1;
                }
                // Send the commands.
                if let Some(commands) = redo.from_commands {
                    conn.send(commands);
                }
                // Push to the undo stack.
                self.undo.push(undo);
                state.unsaved_changes = true;
            }
        }
        // Cycle panels.
        else if input.happened(&InputEvent::NextPanel) {
            let s0 = state.clone();
            state.focus.increment(true);
            state.unsaved_changes = true;
            self.undo.push(Snapshot::from_states(s0, state));
        } else if input.happened(&InputEvent::PreviousPanel) {
            let s0 = state.clone();
            state.focus.increment(false);
            state.unsaved_changes = true;
            self.undo.push(Snapshot::from_states(s0, state));
        }

        // App-level TTS.
        for tts_e in self.tts.iter() {
            if input.happened(tts_e.0) {
                tts.say(tts_e.1.clone())
            }
        }
        // Stop talking.
        if input.happened(&InputEvent::StopTTS) {
            tts.stop();
        }

        // Listen to the focused panel.
        let resp = match state.panels[state.focus.get()] {
            PanelType::Music => {
                self.music_panel
                    .update(state, conn, input, tts, text, paths_state, exporter)
            }
            PanelType::Tracks => {
                self.tracks_panel
                    .update(state, conn, input, tts, text, paths_state, exporter)
            }
            PanelType::OpenFile => {
                self.open_file_panel
                    .update(state, conn, input, tts, text, paths_state, exporter)
            }
            PanelType::PianoRoll => {
                self.piano_roll_panel
                    .update(state, conn, input, tts, text, paths_state, exporter)
            }
            PanelType::ExportState => {
                self.export_panel
                    .update(state, conn, input, tts, text, paths_state, exporter)
            }
            PanelType::ExportSettings => self.export_settings_panel.update(
                state,
                conn,
                input,
                tts,
                text,
                paths_state,
                exporter,
            ),
            PanelType::MainMenu => panic!("This should never happen!"),
        };
        // Push an undo state generated by the focused panel.
        if let Some(snapshot) = resp {
            // Execute IO commands.
            if let Some(io_commands) = &snapshot.io_commands {
                for command in io_commands {
                    match command {
                        // Enable the open-file panel.
                        IOCommand::EnableOpenFile(open_file_type) => match open_file_type {
                            OpenFileType::Export => (),
                            OpenFileType::ReadSave => {
                                self.open_file_panel.read_save(state, paths_state)
                            }
                            OpenFileType::SoundFont => {
                                self.open_file_panel.soundfont(state, paths_state)
                            }
                            OpenFileType::WriteSave => {
                                self.open_file_panel.write_save(state, paths_state)
                            }
                        },
                        // Export.
                        IOCommand::Export(path) => self.export(path, state, conn, exporter),
                        // Close the open-file panel.
                        IOCommand::CloseOpenFile => self.open_file_panel.disable(state),
                    }
                }
            }
            // Push to the undo stack.
            if snapshot.from_state.is_some() || snapshot.from_commands.is_some() {
                state.unsaved_changes = true;
                self.push_undo(snapshot);
            }
        }
        // We're not done yet.
        false
    }

    /// Push this `UndoRedoState` to the undo stack and clear the redo stack.
    fn push_undo(&mut self, snapshot: Snapshot) {
        self.undo.push(snapshot);
        self.redo.clear();
        // Remove an undo if there are too many.
        if self.undo.len() > MAX_UNDOS {
            self.undo.remove(0);
        }
    }

    /// Begin to export audio.
    ///
    /// - `path` The output path.
    /// - `state` The state.
    /// - `conn` The audio conn.
    /// - `exporter` The exporter.
    fn export(&mut self, path: &Path, state: &mut State, conn: &mut Conn, exporter: &Exporter) {
        self.pre_export_panels = state.panels.clone();
        self.pre_export_focus = state.focus.get();
        // Enable the export panel.
        self.export_panel
            .enable(state, &self.pre_export_panels, self.pre_export_focus);
        // Export multiple files.
        if exporter.multi_file {
            self.queue_multi_file_export(path, state, conn, exporter);
        } else {
            // Get commands and an end time.
            let (track_commands, t1) =
                combine_tracks_to_commands(state, exporter.framerate.get_f(), 0);
            // Define the export state.
            let export_state: ExportState = ExportState::new(t1);
            conn.export_state = Some(export_state);
            // Set the framerate.
            // Sound-off. Set the framerate. Export.
            let mut commands = vec![
                Command::SoundOff,
                Command::Export {
                    path: path.to_path_buf(),
                    state: export_state,
                },
            ];
            commands.extend(track_commands);
            // Send the commands.
            conn.send(commands);
        }
    }

    /// Enqueue multi-file export commands.
    ///
    /// - `path` The root path, without tracks-specific suffixes.
    /// - `state` The state.
    /// - `conn` The audio connection.
    /// - `exporter` The exporter.
    fn queue_multi_file_export(
        &mut self,
        path: &Path,
        state: &State,
        conn: &Conn,
        exporter: &Exporter,
    ) {
        self.export_queue.clear();
        let e0 = exporter.clone();
        // Get base path information.
        let extension = path.extension().unwrap().to_str().unwrap();
        let filename_base = path.file_stem().unwrap().to_str().unwrap();
        let directory = path.parent().unwrap();
        // Get the framerate.
        let framerate_f = exporter.framerate.get_f();
        let framerate_u = exporter.framerate.get_u() as u32;
        // Start playing music.
        let t0 = state.time.ppq_to_samples(0, framerate_f);
        let mut paths = vec![];
        // Get playable tracks.
        for track in get_playable_tracks(&state.music) {
            let mut t1 = t0;
            // Export to wav.
            let mut e1 = exporter.clone();
            e1.export_type.index.set(0);
            // Start to play music.
            let mut commands = vec![
                Command::SetExporter {
                    exporter: Box::new(e1),
                },
                Command::SetFramerate {
                    framerate: framerate_u,
                },
                Command::PlayMusic { time: t0 },
            ];
            let notes = get_playback_notes(track);
            for note in notes.iter() {
                // Convert the start and duration to sample lengths.
                let start = state.time.ppq_to_samples(note.start, framerate_f);
                if start < t0 {
                    continue;
                }
                let end = state.time.ppq_to_samples(note.end, framerate_f);
                if end > t1 {
                    t1 = end;
                }
                // Add the command.
                commands.push(Command::NoteOnAt {
                    channel: track.channel,
                    key: note.note,
                    velocity: note.velocity,
                    start,
                    end,
                })
            }
            // Get the path for this track.
            let suffix = match exporter.multi_file_suffix.get() {
                MultiFile::Channel => track.channel.to_string(),
                MultiFile::Preset => conn
                    .state
                    .programs
                    .get(&track.channel)
                    .unwrap()
                    .preset_name
                    .clone(),
                MultiFile::ChannelAndPreset => format!(
                    "{}_{}",
                    track.channel,
                    conn.state.programs.get(&track.channel).unwrap().preset_name
                ),
            };
            // Get the path.
            let track_path = directory.join(format!("{}_{}.{}", filename_base, suffix, extension));
            paths.push(track_path.clone());
            // Get the export state.
            let export_state = ExportState::new(t1);
            // Export.
            commands.extend([
                Command::SoundOff,
                Command::Export {
                    path: track_path,
                    state: export_state,
                },
            ]);
            self.export_queue.push((commands, Some(export_state)));
        }
        self.export_queue.push((
            vec![
                Command::StopMusic,
                Command::SetExporter {
                    exporter: Box::new(e0),
                },
                Command::AppendSilences { paths },
            ],
            None,
        ));
    }
}

/// Returns all tracks that can be played.
fn get_playable_tracks(music: &Music) -> Vec<&MidiTrack> {
    // Get all tracks that can play music.
    let tracks = match music.midi_tracks.iter().find(|t| t.solo) {
        // Only include the solo track.
        Some(solo) => vec![solo],
        // Only include unmuted tracks.
        None => music.midi_tracks.iter().filter(|t| !t.mute).collect(),
    };
    tracks
}

/// Returns all notes in the track that can be played (they are after t0).
fn get_playback_notes(track: &MidiTrack) -> Vec<Note> {
    let gain = track.gain as f64 / MAX_VOLUME as f64;
    let mut notes = vec![];
    for note in track.notes.iter() {
        let mut n1 = *note;
        n1.velocity = (n1.velocity as f64 * gain) as u8;
        notes.push(n1);
    }
    notes.sort();
    notes
}
/// Converts all playable tracks to note-on commands.
fn combine_tracks_to_commands(
    state: &State,
    framerate: f32,
    start_time: u64,
) -> (CommandsMessage, u64) {
    // Start playing music.
    let t0 = state.time.ppq_to_samples(start_time, framerate);
    let mut t1 = t0;
    let mut commands = vec![
        Command::PlayMusic { time: t0 },
        Command::SetFramerate {
            framerate: framerate as u32,
        },
    ];
    // Get playable tracks.
    for track in get_playable_tracks(&state.music) {
        let notes = get_playback_notes(track);
        for note in notes.iter() {
            // Convert the start and duration to sample lengths.
            let start = state.time.ppq_to_samples(note.start, framerate);
            if start < t0 {
                continue;
            }
            let end = state.time.ppq_to_samples(note.end, framerate);
            if end > t1 {
                t1 = end;
            }
            // Add the command.
            commands.push(Command::NoteOnAt {
                channel: track.channel,
                key: note.note,
                velocity: note.velocity,
                start,
                end,
            })
        }
    }
    // All-off.
    commands.push(Command::StopMusicAt { time: t1 });
    (commands, t1)
}

pub(crate) fn select_track(state: &mut State, input: &Input) -> Option<Snapshot> {
    if let Some(selected) = state.music.selected {
        if input.happened(&InputEvent::NextTrack) && selected < state.music.midi_tracks.len() - 1 {
            let s0 = state.clone();
            state.music.selected = Some(selected + 1);
            deselect(state);
            Some(Snapshot::from_states(s0, state))
        }
        // Previous track.
        else if input.happened(&InputEvent::PreviousTrack) && selected > 0 {
            let s0 = state.clone();
            state.music.selected = Some(selected - 1);
            deselect(state);
            Some(Snapshot::from_states(s0, state))
        } else {
            None
        }
    } else {
        None
    }
}

fn deselect(state: &mut State) {
    state.select_mode = match &state.select_mode {
        SelectMode::Single(_) => SelectMode::Single(None),
        SelectMode::Many(_) => SelectMode::Many(None),
    };
}
