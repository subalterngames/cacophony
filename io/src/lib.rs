//! This crate handles essentially all of Cacophony's functionality except the rendering (see the `render` crate).
//!
//! The only public struct is `IO`.
//!
//! Per frame, `IO` listens for user input via an `Input` (see the `input` crate), and then does any of the following:
//!
//! - Update `State` (see the `common` crate), for example add a new track.
//! - Update `Conn` (see the `audio` crate), for example to play notes.
//! - Send an internal `IOCommand` to itself.
//! - Play text-to-speech audio (see the `text` crate).
//!
//! Certain operations will create a copy of the current `State` which will be added to an undo stack.
//! Undoing an action reverts the app to that state, pops it from the undo stack, and pushes it to the redo stack.
//!
//! `IO` divides input listening into discrete panels, e.g. the music panel and the tracks panel.
//! Each panel implements the `Panel` trait.

use audio::export::ExportState;
use audio::Conn;
use common::{InputState, Music, PanelType, Paths, PathsState, State};
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

/// Parse user input and apply it to the application's various states as needed:
///
/// - Play ad-hoc notes.
/// - Modify the `State` and push the old version to the undo stack.
/// - Modify the `PathsState`.
/// - Modify the `Conn`.
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
    ///
    /// Returns: An `Snapshot`.
    pub fn update(
        &mut self,
        state: &mut State,
        conn: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &mut Text,
        paths_state: &mut PathsState,
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

        // Don't do anything while exporting.
        if conn.exporting() {
            return false;
        }

        // Mark the music as not dirty.
        state.music.dirty = false;

        // Alphanumeric input.
        if state.input.alphanumeric_input {
            // Get the focused panel.
            let panel = self.get_panel(&state.panels[state.focus.get()]);

            // Toggle off alphanumeric input.
            if panel.allow_alphanumeric_input(state, conn) {
                if input.happened(&InputEvent::ToggleAlphanumericInput) {
                    let s0 = state.clone();
                    state.input.alphanumeric_input = false;
                    // Do something on disable.
                    panel.on_disable_abc123(state, conn);
                    // There is always a snapshot (because we toggled off alphanumeric input).
                    let snapshot = Some(Snapshot::from_states(s0, state));
                    // Apply the snapshot.
                    self.apply_snapshot(snapshot, state, conn, paths_state);
                    return false;
                }
                // Try to do alphanumeric input.
                else {
                    let (snapshot, updated) = panel.update_abc123(state, input, conn);
                    // We applied alphanumeric input.
                    if updated {
                        self.apply_snapshot(snapshot, state, conn, paths_state);
                        return false;
                    }
                }
            }
        }
        // Apply alphanumeric input.
        else {
            let panel = self.get_panel(&state.panels[state.focus.get()]);
            if panel.allow_alphanumeric_input(state, conn)
                && input.happened(&InputEvent::ToggleAlphanumericInput)
            {
                let snapshot = Some(Snapshot::from_state_value(
                    |s| &mut s.input.alphanumeric_input,
                    true,
                    state,
                ));
                self.apply_snapshot(snapshot, state, conn, paths_state);
                return false;
            } else if let Some(track) = state.music.get_selected_track() {
                // Play notes.
                if !&input.note_on_messages.is_empty()
                    && panel.allow_play_music()
                    && conn.state.programs.get(&track.channel).is_some()
                {
                    conn.note_ons(state, &input.note_on_messages);
                }
                if !&input.note_off_keys.is_empty() {
                    conn.note_offs(state, &input.note_off_keys)
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
                    Save::write(&path.with_extension("cac"), state, conn, paths_state);
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
            let export_state = *conn.export_state.lock();
            // We aren't exporting already.
            if export_state == ExportState::NotExporting {
                self.pre_export_focus = state.focus.get();
                self.pre_export_panels = state.panels.clone();
                self.open_file_panel.export(state, paths_state, conn)
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
                    conn.do_commands(&commands);
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
                    conn.do_commands(&commands);
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
        let snapshot = panel.update(state, conn, input, tts, text, paths_state);
        let (applied, need_to_quit) = self.apply_snapshot(snapshot, state, conn, paths_state);
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
            conn.set_music(state);
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
    ) {
        Save::read(save_path, state, conn, paths_state);
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
                        IOCommand::Export => {
                            self.export_panel.enable(
                                state,
                                &self.pre_export_panels,
                                self.pre_export_focus,
                            );
                            conn.start_export(state, paths_state);
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
            state.selection.deselect();
            Some(Snapshot::from_states(s0, state))
        } else if input.happened(&events[1]) && selected < state.music.midi_tracks.len() - 1 {
            let s0 = state.clone();
            state.music.selected = Some(selected + 1);
            state.selection.deselect();
            Some(Snapshot::from_states(s0, state))
        } else {
            None
        }
    } else {
        None
    }
}
