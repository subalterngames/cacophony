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

use audio::{Command, CommandsMessage, Conn, ExportState};
use common::hashbrown::HashMap;
use common::ini::Ini;
use common::time::bar_to_samples;
use common::{InputState, MidiTrack, Music, Note, PanelType, Paths, State, MAX_VOLUME, PathsState};
use edit::edit_file;
use input::{Input, InputEvent};
use std::path::PathBuf;
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
use open_file_panel::OpenFilePanel;
use panel::Panel;
use piano_roll::PianoRollPanel;
use save::Save;
use snapshot::Snapshot;
use tooltip::*;
use tracks_panel::TracksPanel;

/// The maximum size of the undo stack.
const MAX_UNDOS: usize = 100;

pub struct IO {
    /// A stack of snapshots that can be popped to undo an action.
    undo: Vec<Snapshot>,
    /// A stack of snapshots that can be popped to redo an action.
    redo: Vec<Snapshot>,
    /// Top-level text-to-speech lookups.
    tts: HashMap<InputEvent, String>,
    /// The music panel.
    music_panel: MusicPanel,
    /// The tracks panel.
    tracks_panel: TracksPanel,
    /// The open-file panel.
    pub open_file_panel: OpenFilePanel,
    /// The piano roll panel.
    piano_roll_panel: PianoRollPanel,
    /// The file path. If None, we haven't saved this music yet.
    save_path: Option<PathBuf>,
    /// The export path, if any.
    export_path: Option<PathBuf>,
}

impl IO {
    pub fn new(config: &Ini, input: &Input, input_state: &InputState, text: &Text) -> Self {
        let mut tts = HashMap::new();
        // App TTS.
        let app = get_tooltip(
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
            text,
        );
        tts.insert(InputEvent::AppTTS, app);
        // File TTS.
        let file = get_tooltip(
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
            text,
        );
        tts.insert(InputEvent::FileTTS, file);
        let music_panel = MusicPanel {};
        let tracks_panel = TracksPanel {};
        let open_file_panel = OpenFilePanel::default();
        let piano_roll_panel = PianoRollPanel::new(&input_state.beat, config);
        Self {
            tts,
            music_panel,
            tracks_panel,
            open_file_panel,
            piano_roll_panel,
            redo: vec![],
            undo: vec![],
            save_path: None,
            export_path: None,
        }
    }

    pub fn update(
        &mut self,
        state: &mut State,
        conn: &mut Conn,
        input: &mut Input,
        tts: &mut TTS,
        text: &Text,
        paths: &Paths,
        paths_state: &mut PathsState,
    ) -> bool {
        // Update the input state.
        input.update(state);

        // Quit.
        if input.happened(&InputEvent::Quit) {
            return true;
        }

        // Play notes.
        if !&input.play_now.is_empty() {
            if let Some(track) = state.music.get_selected_track() {
                if conn.state.programs.get(&track.channel).is_some() {
                    let gain = track.gain as f64 / 127.0;
                    let mut commands = vec![];
                    let duration = bar_to_samples(&state.input.beat, state.music.bpm);
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
            self.save_path = None;
            state.music = Music::default();
        }
        // Open file.
        else if input.happened(&InputEvent::OpenFile) {
            self.open_file_panel.read_save(paths, state);
        }
        // Save file.
        else if input.happened(&InputEvent::SaveFile) {
            match &self.save_path {
                // Save to the existing path,
                Some(path) => Save::write(path, state, conn, paths_state),
                // Set a new path.
                None => self.open_file_panel.write_save(paths, state),
            }
        }
        // Save to a new path.
        else if input.happened(&InputEvent::SaveFileAs) {
            self.open_file_panel.write_save(paths, state)
        }
        // Export.
        else if input.happened(&InputEvent::ExportFile) {
            // We aren't exporting already.
            if conn.export_state.is_none() {
                match &self.export_path {
                    Some(path) => {
                        let (mut commands, t1) = tracks_to_commands(state);
                        // Define the export state.
                        let export_state = ExportState::new(t1);
                        // Insert the export command.
                        commands.insert(
                            0,
                            Command::Export {
                                path: path.clone(),
                                state: export_state,
                            },
                        );
                        // Send the commands.
                        conn.send(commands);
                    }
                    None => self.open_file_panel.export(paths, state),
                }
            }
        }
        // Open config file.
        else if input.happened(&InputEvent::EditConfig) {
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
            }
        }
        // Cycle panels.
        else if input.happened(&InputEvent::NextPanel) {
            let s0 = state.clone();
            state.focus.increment(true);
            self.undo.push(Snapshot::from_states(s0, state));
        } else if input.happened(&InputEvent::PreviousPanel) {
            let s0 = state.clone();
            state.focus.increment(false);
            self.undo.push(Snapshot::from_states(s0, state));
        }

        // App-level TTS.
        for tts_e in self.tts.iter() {
            if input.happened(tts_e.0) {
                tts.say(tts_e.1)
            }
        }
        // Stop talking.
        if input.happened(&InputEvent::StopTTS) {
            tts.stop();
        }

        // Listen to the focused panel.
        let resp = match state.panels[state.focus.get()] {
            PanelType::Music => self.music_panel.update(state, conn, input, tts, text, paths, paths_state),
            PanelType::Tracks => self.tracks_panel.update(state, conn, input, tts, text, paths, paths_state),
            PanelType::OpenFile => self.open_file_panel.update(state, conn, input, tts, text, paths, paths_state),
            PanelType::PianoRoll => self.piano_roll_panel.update(state, conn, input, tts, text, paths, paths_state),
            other => panic!("Not implemented: {:?}", other),
        };
        // Push an undo state generated by the focused panel.
        if let Some(snapshot) = resp {
            // Execute IO commands.
            if let Some(io_commands) = &snapshot.io_commands {
                for command in io_commands {
                    match command {
                        IOCommand::EnableOpenFile(open_file_type) => match open_file_type {
                            OpenFileType::Export => self.open_file_panel.export(paths, state),
                            OpenFileType::ReadSave => self.open_file_panel.read_save(paths, state),
                            OpenFileType::SoundFont => self.open_file_panel.soundfont(paths, state),
                            OpenFileType::WriteSave => {
                                self.open_file_panel.write_save(paths, state)
                            }
                        },
                        IOCommand::DisableOpenFile => {
                            self.open_file_panel.disable(state);
                        }
                        IOCommand::SetSavePath(path) => self.save_path = path.clone(),
                        IOCommand::SetExportPath(path) => self.export_path = Some(path.clone()),
                    }
                }
            }
            // Push to the undo stack.
            if snapshot.from_state.is_some() || snapshot.from_commands.is_some() {
                self.push_undo(snapshot);
            }
        }
        // Try to update time itself.
        conn.update_time();

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
fn get_playback_notes(state: &State, track: &MidiTrack) -> Vec<Note> {
    let gain = track.gain as f64 / MAX_VOLUME as f64;
    let mut notes = vec![];
    for note in track
        .notes
        .iter()
        .filter(|n| n.start >= state.time.playback)
    {
        let mut n1 = *note;
        n1.velocity = (n1.velocity as f64 * gain) as u8;
        notes.push(n1);
    }
    notes
}

/// Converts all playable tracks to note-on commands.
fn tracks_to_commands(state: &State) -> (CommandsMessage, u64) {
    let bpm = state.music.bpm;
    // Start playing music.
    let t0 = bar_to_samples(&state.time.playback, bpm);
    let mut t1 = t0;
    let mut commands = vec![Command::PlayMusic { time: t0 }];
    // Get playable tracks.
    for track in get_playable_tracks(&state.music) {
        let notes = get_playback_notes(state, track);
        for note in notes.iter() {
            // Convert the start and duration to sample lengths.
            let start = bar_to_samples(&note.start, bpm);
            let duration = bar_to_samples(&note.duration, bpm);
            // Is this the last note?
            let note_t1 = start + duration;
            if note_t1 > t1 {
                t1 = note_t1;
            }
            // Add the command.
            commands.push(Command::NoteOnAt {
                channel: track.channel,
                key: note.note,
                velocity: note.velocity,
                time: start,
                duration,
            })
        }
    }
    // All-off.
    commands.push(Command::StopMusicAt { time: t1 });
    (commands, t1)
}
