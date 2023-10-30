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

use audio::exporter::{Exporter, MultiFileSuffix};
use audio::{Command, CommandsMessage, Conn, ExportState, SharedExporter};
use common::{
    InputState, MidiTrack, Music, Note, PanelType, Paths, PathsState, SelectMode, State, MAX_VOLUME,
};
use edit::edit_file;
use hashbrown::HashMap;
use ini::Ini;
use input::{Input, InputEvent};
use std::path::Path;
use text::{Enqueable, Text, Tooltips, TtsString, TTS};
mod export_panel;
mod import_midi;
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
use common::open_file::{FileAndDirectory, OpenFileType};
use export_panel::ExportPanel;
use export_settings_panel::ExportSettingsPanel;
use open_file_panel::OpenFilePanel;
use panel::Panel;
use piano_roll::PianoRollPanel;
use save::Save;
use snapshot::Snapshot;
use tracks_panel::TracksPanel;
mod abc123;
mod export_settings_panel;
mod quit_panel;
use quit_panel::QuitPanel;
mod links_panel;
mod popup;
use links_panel::LinksPanel;

/// The maximum size of the undo stack.
const MAX_UNDOS: usize = 100;
/// Commands that are queued for export.
type QueuedExportCommands = (CommandsMessage, Option<ExportState>);

/// Parse user input and apply it to the application's various states as needed:
///
/// - Play ad-hoc notes.
/// - Modify the `State` and push the old version to the undo stack.
/// - Modify the `PathsState`.
/// - Modify the `SynthState` and send commands through the `Conn`.
/// - Modify the `Exporter` and send a copy via a command to the `Conn`.
pub struct IO {
    /// A stack of snapshots that can be popped to undo an action.
    undo: Vec<Snapshot>,
    /// A stack of snapshots that can be popped to redo an action.
    redo: Vec<Snapshot>,
    /// Top-level text-to-speech lookups.
    tts: HashMap<InputEvent, Vec<TtsString>>,
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
    /// The quit panel.
    quit_panel: QuitPanel,
    /// The links panel.
    links_panel: LinksPanel,
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
        let mut tooltips = Tooltips::default();
        // App TTS.
        let app_tts = vec![
            TtsString::from(text.get_ref("APP_TTS_0")),
            tooltips
                .get_tooltip(
                    "APP_TTS_1",
                    &[
                        InputEvent::StatusTTS,
                        InputEvent::InputTTS,
                        InputEvent::FileTTS,
                    ],
                    input,
                    text,
                )
                .clone(),
            tooltips
                .get_tooltip("APP_TTS_2", &[InputEvent::Quit], input, text)
                .clone(),
            tooltips
                .get_tooltip(
                    "APP_TTS_3",
                    &[InputEvent::PreviousPanel, InputEvent::NextPanel],
                    input,
                    text,
                )
                .clone(),
            tooltips
                .get_tooltip(
                    "APP_TTS_4",
                    &[InputEvent::Undo, InputEvent::Redo],
                    input,
                    text,
                )
                .clone(),
            tooltips
                .get_tooltip("APP_TTS_5", &[InputEvent::StopTTS], input, text)
                .clone(),
            tooltips
                .get_tooltip("APP_TTS_6", &[InputEvent::EnableLinksPanel], input, text)
                .clone(),
        ];
        tts.insert(InputEvent::AppTTS, app_tts);
        // File TTS.
        let file_tts = vec![
            tooltips
                .get_tooltip("FILE_TTS_0", &[InputEvent::NewFile], input, text)
                .clone(),
            tooltips
                .get_tooltip("FILE_TTS_1", &[InputEvent::OpenFile], input, text)
                .clone(),
            tooltips
                .get_tooltip(
                    "FILE_TTS_2",
                    &[InputEvent::SaveFile, InputEvent::SaveFileAs],
                    input,
                    text,
                )
                .clone(),
            tooltips
                .get_tooltip("FILE_TTS_3", &[InputEvent::ExportFile], input, text)
                .clone(),
            tooltips
                .get_tooltip("FILE_TTS_4", &[InputEvent::ImportMidi], input, text)
                .clone(),
            tooltips
                .get_tooltip("FILE_TTS_5", &[InputEvent::EditConfig], input, text)
                .clone(),
        ];
        tts.insert(InputEvent::FileTTS, file_tts);
        let music_panel = MusicPanel::default();
        let tracks_panel = TracksPanel::default();
        let open_file_panel = OpenFilePanel::default();
        let piano_roll_panel = PianoRollPanel::new(&input_state.beat.get_u(), config);
        let export_panel = ExportPanel::default();
        let export_settings_panel = ExportSettingsPanel::default();
        let quit_panel = QuitPanel::default();
        let links_panel = LinksPanel::default();
        Self {
            tts,
            music_panel,
            tracks_panel,
            open_file_panel,
            piano_roll_panel,
            export_panel,
            export_settings_panel,
            quit_panel,
            links_panel,
            redo: vec![],
            undo: vec![],
            export_queue: vec![],
            pre_export_panels: vec![],
            pre_export_focus: 0,
        }
    }

    /// Update the state of the app. Returns true if we're done.
    ///
    /// - `state` The state of the app.
    /// - `conn` The synthesizer-player connection.
    /// - `input` Input events, key presses, etc.
    /// - `tts` Text-to-speech.
    /// - `text` The text.
    /// - `paths_state` Dynamic path data.
    /// - `exporter` Export settings.
    ///
    /// Returns: An `Snapshot`.
    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        state: &mut State,
        conn: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &mut Text,
        paths_state: &mut PathsState,
        exporter: &mut SharedExporter,
    ) -> bool {
        if input.happened(&InputEvent::Quit) {
            // Enable the quit panel.
            if state.unsaved_changes {
                self.quit_panel.enable(state);
            }
            // Quit.
            else {
                return true;
            }
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

        // Alphanumeric input.
        if state.input.alphanumeric_input {
            // Get the focused panel.
            let panel = self.get_panel(&state.panels[state.focus.get()]);

            // Toggle off alphanumeric input.
            if panel.allow_alphanumeric_input(state, exporter) {
                if input.happened(&InputEvent::ToggleAlphanumericInput) {
                    let s0 = state.clone();
                    state.input.alphanumeric_input = false;
                    // Do something on disable.
                    panel.on_disable_abc123(state, exporter);
                    // There is always a snapshot (because we toggled off alphanumeric input).
                    let snapshot = Some(Snapshot::from_states(s0, state));
                    // Apply the snapshot.
                    self.apply_snapshot(snapshot, state, conn, paths_state, exporter);
                    return false;
                }
                // Try to do alphanumeric input.
                else {
                    let (snapshot, updated) = panel.update_abc123(state, input, exporter);
                    // We applied alphanumeric input.
                    if updated {
                        self.apply_snapshot(snapshot, state, conn, paths_state, exporter);
                        return false;
                    }
                }
            }
        }
        // Apply alphanumeric input.
        else {
            let panel = self.get_panel(&state.panels[state.focus.get()]);
            if panel.allow_alphanumeric_input(state, exporter)
                && input.happened(&InputEvent::ToggleAlphanumericInput)
            {
                let snapshot = Some(Snapshot::from_state_value(
                    |s| &mut s.input.alphanumeric_input,
                    true,
                    state,
                ));
                self.apply_snapshot(snapshot, state, conn, paths_state, exporter);
                return false;
            } else if let Some(track) = state.music.get_selected_track() {
                let mut commands = vec![];
                // Play notes.
                if !&input.note_on_messages.is_empty()
                    && panel.allow_play_music()
                    && conn.state.programs.get(&track.channel).is_some()
                {
                    let gain = track.gain as f64 / 127.0;
                    // Set the framerate for playback.
                    commands.push(Command::SetFramerate {
                        framerate: conn.framerate as u32,
                    });
                    // Play the notes.
                    for note in input.note_on_messages.iter() {
                        // Set the volume.
                        let volume = (note[2] as f64 * gain) as u8;
                        commands.push(Command::NoteOn {
                            channel: track.channel,
                            key: note[1],
                            velocity: volume,
                        });
                    }
                }
                // Note-offs.
                for note_off in input.note_off_keys.iter() {
                    commands.push(Command::NoteOff {
                        channel: track.channel,
                        key: *note_off,
                    });
                }
                if !commands.is_empty() {
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
        } else if input.happened(&InputEvent::ImportMidi) {
            self.open_file_panel.import_midi(state, paths_state);
        }
        // Open config file.
        else if input.happened(&InputEvent::EditConfig) {
            let paths = Paths::get();
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
                tts.stop();
                tts.enqueue(tts_e.1.clone());
            }
        }
        // Stop talking or clear the queue for new speech.
        if input.happened(&InputEvent::StopTTS)
            || input.happened(&InputEvent::StatusTTS)
            || input.happened(&InputEvent::InputTTS)
        {
            tts.stop();
        }
        // Links.
        if input.happened(&InputEvent::EnableLinksPanel) {
            self.links_panel.enable(state);
            return false;
        }
        // Get the focused panel.
        let panel = self.get_panel(&state.panels[state.focus.get()]);
        // Update the focuses panel and potentially get a screenshot.
        let snapshot = panel.update(state, conn, input, tts, text, paths_state, exporter);
        let (applied, need_to_quit) =
            self.apply_snapshot(snapshot, state, conn, paths_state, exporter);
        // Quit while we're ahead.
        if need_to_quit {
            return true;
        }
        // Stop doing stuff here but don't quit.
        else if applied {
            return false;
        }
        // Get the focused panel.
        let panel = self.get_panel(&state.panels[state.focus.get()]);
        // Play music.
        if panel.allow_play_music() && input.happened(&InputEvent::PlayStop) {
            match conn.state.time.music {
                // Stop playing.
                true => conn.send(vec![Command::StopMusic]),
                false => {
                    conn.send(
                        combine_tracks_to_commands(state, conn.framerate, state.time.playback).0,
                    );
                }
            }
        }
        // We're not done yet.
        false
    }

    /// Open a save file from a path.
    pub fn load_save(
        &self,
        save_path: &Path,
        state: &mut State,
        conn: &mut Conn,
        paths_state: &mut PathsState,
        exporter: &mut SharedExporter,
    ) {
        Save::read(&save_path, state, conn, paths_state, exporter);
        // Set the saves directory.
        paths_state.saves = FileAndDirectory::new_path(save_path.to_path_buf());
    }

    fn get_panel(&mut self, panel_type: &PanelType) -> &mut dyn Panel {
        match panel_type {
            PanelType::ExportSettings => &mut self.export_settings_panel,
            PanelType::ExportState => &mut self.export_panel,
            PanelType::MainMenu => panic!(
                "Tried to get a mutable reference to the main menu. This should never happen!"
            ),
            PanelType::Music => &mut self.music_panel,
            PanelType::OpenFile => &mut self.open_file_panel,
            PanelType::PianoRoll => &mut self.piano_roll_panel,
            PanelType::Tracks => &mut self.tracks_panel,
            PanelType::Quit => &mut self.quit_panel,
            PanelType::Links => &mut self.links_panel,
        }
    }

    /// Apply the snapshot. Apply IO commands and put a state on the undo stack.
    ///
    /// Returns: True if a state was applied, true if we need to quit.
    fn apply_snapshot(
        &mut self,
        snapshot: Option<Snapshot>,
        state: &mut State,
        conn: &mut Conn,
        paths_state: &mut PathsState,
        exporter: &mut SharedExporter,
    ) -> (bool, bool) {
        // Push an undo state generated by the focused panel.
        if let Some(snapshot) = snapshot {
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
                            OpenFileType::ImportMidi => {
                                self.open_file_panel.import_midi(state, paths_state)
                            }
                        },
                        // Export.
                        IOCommand::Export(path) => {
                            self.export(path, state, conn, &mut exporter.lock())
                        }
                        // Close the open-file panel.
                        IOCommand::CloseOpenFile => self.open_file_panel.disable(state),
                        // Quit the application.
                        IOCommand::Quit => return (false, true),
                    }
                }
            }
            // Push to the undo stack.
            if snapshot.from_state.is_some() || snapshot.from_commands.is_some() {
                state.unsaved_changes = true;
                self.push_undo(snapshot);
            }
            (true, false)
        } else {
            (false, false)
        }
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
    fn export(&mut self, path: &Path, state: &mut State, conn: &mut Conn, exporter: &mut Exporter) {
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
        exporter: &mut Exporter,
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
            exporter.export_type.index.set(0);
            // Start to play music.
            let mut commands = vec![
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
                MultiFileSuffix::Channel => track.channel.to_string(),
                MultiFileSuffix::Preset => conn
                    .state
                    .programs
                    .get(&track.channel)
                    .unwrap()
                    .preset_name
                    .clone(),
                MultiFileSuffix::ChannelAndPreset => format!(
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
        *exporter = e0;
        self.export_queue.push((
            vec![Command::StopMusic, Command::AppendSilences { paths }],
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

/// Try to select a track, given user input.
///
/// This is here an not in a more obvious location because both `TracksPanel` and `PianoRollPanel` need it.
pub(crate) fn select_track(
    state: &mut State,
    input: &Input,
    events: [InputEvent; 2],
) -> Option<Snapshot> {
    if let Some(selected) = state.music.selected {
        if input.happened(&events[0]) && selected > 0 {
            let s0 = state.clone();
            state.music.selected = Some(selected - 1);
            deselect(state);
            Some(Snapshot::from_states(s0, state))
        } else if input.happened(&events[1]) && selected < state.music.midi_tracks.len() - 1 {
            let s0 = state.clone();
            state.music.selected = Some(selected + 1);
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
