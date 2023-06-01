use audio::{Command, CommandsMessage, Conn, ExportState};
use common::hashbrown::HashMap;
use common::ini::Ini;
use common::time::bar_to_samples;
use common::{Fraction, InputState, MidiTrack, Music, Note, PanelType, Paths, State, MAX_VOLUME};
use edit::edit_file;
use input::{Input, InputEvent};
use std::path::PathBuf;
use text::{Text, TTS};
mod io_command;
mod music_panel;
mod open_file;
mod panel;
mod piano_roll;
mod tracks_panel;
mod undo_state;
use io_command::IOCommand;
pub(crate) use io_command::IOCommands;
use music_panel::MusicPanel;
use open_file::open_file_panel::OpenFilePanel;
use open_file::open_file_type::OpenFileType;
pub(crate) use panel::Panel;
use piano_roll::PianoRollPanel;
use tooltip::*;
use tracks_panel::TracksPanel;
pub(crate) use undo_state::UndoRedoState;

/// The maximum size of the undo stack.
const MAX_UNDOS: usize = 100;

pub struct IO {
    /// A stack of states that can be popped to undo an action.
    undo: Vec<UndoRedoState>,
    /// A stack of states that can be popped to redo an action.
    redo: Vec<UndoRedoState>,
    /// Top-level text-to-speech lookups.
    tts: HashMap<InputEvent, String>,
    /// The music panel.
    music_panel: MusicPanel,
    /// The tracks panel.
    tracks_panel: TracksPanel,
    /// The open-file panel.
    open_file_panel: OpenFilePanel,
    /// The piano roll panel.
    piano_roll_panel: PianoRollPanel,
    /// The file path. If None, we haven't saved this music yet.
    pub save_path: Option<PathBuf>,
    /// The export path, if any.
    pub export_path: Option<PathBuf>,
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
    ) -> bool {
        // Update the input state.
        input.update(state);

        // Quit.
        if input.happened(&InputEvent::Quit) {
            return true;
        }
        // New file.
        else if input.happened(&InputEvent::NewFile) {
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
                Some(path) => state.write(path),
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
                        let tracks = get_playable_tracks(&state.music);
                        let notes = get_notes_per_track(&tracks, &state.time.playback);
                        // We have notes we can export.
                        if !notes.is_empty() {
                            let mut commands =
                                notes_to_commands(&notes, &state.time.playback, state.music.bpm);
                            // Get the end time.
                            let t1 = bar_to_samples(
                                &notes
                                    .values()
                                    .map(|ns| {
                                        ns.1.iter().map(|n| n.start + n.duration).max().unwrap()
                                    })
                                    .max()
                                    .unwrap(),
                                state.music.bpm,
                            );
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
                            // Begin exporting.
                            conn.send(commands);
                        }
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
        else if input.happened(&InputEvent::Undo) && !self.undo.is_empty() {
            // Get the first undo-redo state.
            let undo_redo = self.undo.remove(0);
            let redo = UndoRedoState::from((undo_redo.redo, &undo_redo.undo));
            // Assign the undo state to the current state.
            if let Some(s1) = undo_redo.undo.state {
                *state = s1;
            }
            // Send the commands.
            if let Some(commands) = undo_redo.undo.commands {
                conn.send(commands);
            }
            // Push to the redo stack.
            self.redo.push(redo);
        // Redo.
        } else if input.happened(&InputEvent::Redo) && !self.redo.is_empty() {
            // Get the first undo-redo state.
            let undo_redo = self.redo.remove(0);
            let undo = UndoRedoState::from((undo_redo.undo, &undo_redo.redo));
            // Assign the redo state to the current state.
            if let Some(s1) = undo_redo.redo.state {
                *state = s1;
            }
            // Send the commands.
            if let Some(commands) = undo_redo.redo.commands {
                conn.send(commands);
            }
            // Push to the undo stack.
            self.undo.push(undo);
        }
        // Cycle panels.
        else if input.happened(&InputEvent::NextPanel) {
            let s0 = state.clone();
            state.focus.increment(true);
            self.push_state_to_undos(s0, state);
        } else if input.happened(&InputEvent::PreviousPanel) {
            let s0 = state.clone();
            state.focus.increment(false);
            self.push_state_to_undos(s0, state);
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
            PanelType::Music => self.music_panel.update(state, conn, input, tts, text),
            PanelType::Tracks => self.tracks_panel.update(state, conn, input, tts, text),
            PanelType::OpenFile => self.open_file_panel.update(state, conn, input, tts, text),
            PanelType::PianoRoll => self.piano_roll_panel.update(state, conn, input, tts, text),
            other => panic!("Not implemented: {:?}", other),
        };
        // Push an undo state generated by the focused panel.
        if let Some(undo) = resp {
            // Execute IO commands.
            if let Some(io_commands) = &undo.undo.io_commands {
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
            if undo.undo.state.is_some() || undo.undo.commands.is_some() {
                self.push_undo(undo);
            }
        }
        // Try to update time itself.
        conn.update_time();

        // We're not done yet.
        false
    }

    /// Push this `State` to the undo stack and clear the redo stack.
    fn push_state_to_undos(&mut self, s0: State, s1: &State) {
        self.push_undo(UndoRedoState::from((s0, s1)));
    }

    /// Push this `UndoRedoState` to the undo stack and clear the redo stack.
    fn push_undo(&mut self, undo_redo: UndoRedoState) {
        self.undo.push(undo_redo);
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

/// Returns all notes per track that start after the playback time.
fn get_notes_per_track<'a>(
    tracks: &[&'a MidiTrack],
    playback: &Fraction,
) -> HashMap<u8, (f64, Vec<&'a Note>)> {
    let mut notes_per_track = HashMap::new();
    let max_gain = MAX_VOLUME as f64;
    for track in tracks.iter() {
        let notes: Vec<&Note> = track
            .notes
            .iter()
            .filter(|n| n.start >= *playback)
            .collect();
        if !notes.is_empty() {
            let gain = track.gain as f64 * max_gain;
            notes_per_track.insert(track.channel, (gain, notes));
        }
    }
    notes_per_track
}

/// Converts all playable tracks to note-on commands.
pub(crate) fn tracks_to_commands(music: &Music, playback: &Fraction) -> CommandsMessage {
    let tracks = get_playable_tracks(music);
    let notes = get_notes_per_track(&tracks, playback);
    if notes.is_empty() {
        vec![]
    } else {
        notes_to_commands(&notes, playback, music.bpm)
    }
}

/// Converts playable notes to commands.
fn notes_to_commands(
    notes: &HashMap<u8, (f64, Vec<&Note>)>,
    playback: &Fraction,
    bpm: u32,
) -> CommandsMessage {
    let t0 = bar_to_samples(playback, bpm);
    // Start playing music.
    let mut commands = vec![Command::PlayMusic { time: t0 }];
    // Add notes.
    for channel in notes.keys() {
        let ns = &notes[channel];
        // Add the note.
        for note in ns.1.iter() {
            let start = bar_to_samples(&note.start, bpm);
            let duration = bar_to_samples(&note.duration, bpm);
            let velocity = (note.velocity as f64 * ns.0) as u8;
            commands.push(Command::NoteOnAt {
                channel: *channel,
                key: note.note,
                velocity,
                time: start,
                duration,
            });
        }
    }
    commands
}
