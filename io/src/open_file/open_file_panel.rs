use crate::open_file::*;
use crate::panel::*;
use crate::{get_tooltip, get_tooltip_with_values};
use common::{PanelType, Paths};
use file_or_directory::FileOrDirectory;
use open_file_type::OpenFileType;
use std::path::{Path, PathBuf};
use text::{get_file_name_no_ex, get_folder_name};

const SOUNDFONT_EXTENSIONS: [&str; 2] = ["sf2", "sf3"];
const SAVE_FILE_EXTENSIONS: [&str; 1] = ["cac"];

/// Data for an open-file panel.
#[derive(Default)]
pub struct OpenFilePanel {
    /// Valid file extensions.
    extensions: Vec<String>,
    /// The current directory we're in.
    pub directory: PathBuf,
    /// This defines what we're using the panel for.
    pub open_file_type: OpenFileType,
    /// The index of the selected file or folder.
    pub selected: Option<usize>,
    /// The folders and files in the directory.
    pub paths: Vec<FileOrDirectory>,
    /// The filename. This is used for write operations.
    pub filename: String,
    /// The index of the previously-focused panel.
    previous_focus: Index,
    /// The previously-active panels.
    previous_panels: Vec<PanelType>,
}

impl OpenFilePanel {
    /// Enable the panel.
    fn enable(&mut self, state: &mut State) {
        // Get the selected child and the children.
        let (selected, paths) = self.get_paths();
        self.selected = selected;
        self.paths = paths;
        // Lock undo/redo.
        state.input.can_undo = false;
        // Remember the active panels.
        self.previous_panels = state.panels.clone();
        // Clear all active panels.
        state.panels.clear();
        // Make this the only active panel.
        state.panels.push(PanelType::OpenFile);
        // Remember the focus.
        self.previous_focus = state.focus;
        // Set a new index.
        state.focus = Index::new(0, 1);
    }

    /// Enable a panel that can read SoundFonts.
    pub fn soundfont(&mut self, paths: &Paths, state: &mut State) {
        // Get the initial working directory.
        if self.open_file_type != OpenFileType::SoundFont {
            self.directory = paths.soundfonts_directory.clone();
        }
        self.open_file_type = OpenFileType::SoundFont;
        self.extensions = SOUNDFONT_EXTENSIONS.iter().map(|s| s.to_string()).collect();
        self.enable(state);
    }

    /// Enable a panel that can read save files.
    pub fn read_save(&mut self, paths: &Paths, state: &mut State) {
        self.enable_as_save(paths, state);
        self.open_file_type = OpenFileType::ReadSave;
    }

    /// Enable a panel that can write save files.
    pub fn write_save(&mut self, paths: &Paths, state: &mut State) {
        self.enable_as_save(paths, state);
        self.open_file_type = OpenFileType::WriteSave;
    }

    fn enable_as_save(&mut self, paths: &Paths, state: &mut State) {
        // Get the initial working directory.
        if self.open_file_type != OpenFileType::ReadSave
            && self.open_file_type != OpenFileType::WriteSave
        {
            self.directory = paths.saves_directory.clone();
        }
        self.extensions = SAVE_FILE_EXTENSIONS.iter().map(|s| s.to_string()).collect();
        self.enable(state);
    }

    /// Disable this panel.
    pub fn disable(&self, state: &mut State) {
        // Restore the panels.
        state.panels = self.previous_panels.clone();
        // Restore the focus.
        state.focus = self.previous_focus;
        // Restore undo/redo.
        state.input.can_undo = true;
    }

    /// Get the child paths.
    fn get_paths(&self) -> (Option<usize>, Vec<FileOrDirectory>) {
        // Find all valid paths.
        let valid_paths: Vec<PathBuf> = match self.directory.read_dir() {
            Ok(read) => read
                .filter(|e| e.is_ok())
                .map(|e| e.unwrap().path())
                .filter(|p| p.is_file() || p.read_dir().is_ok())
                .collect(),
            Err(_) => vec![],
        };
        // Get the files.
        let mut files: Vec<&PathBuf> = valid_paths
            .iter()
            .filter(|p| {
                p.is_file()
                    && p.extension().is_some()
                    && self
                        .extensions
                        .contains(&p.extension().unwrap().to_str().unwrap().to_string())
            })
            .collect();
        files.sort();
        // Get the directories.
        let mut folders: Vec<&PathBuf> = valid_paths.iter().filter(|p| p.is_dir()).collect();
        folders.sort();

        let mut paths: Vec<FileOrDirectory> =
            folders.iter().map(|f| FileOrDirectory::new(f)).collect();
        paths.append(&mut files.iter().map(|f| FileOrDirectory::new(f)).collect());

        // Set the selection index.
        let selected: Option<usize> = match !paths.is_empty() {
            true => {
                // Start at the first file.
                match (!folders.is_empty(), !files.is_empty()) {
                    (true, true) => Some(folders.len()),
                    (true, false) => Some(0),
                    (false, true) => Some(0),
                    (false, false) => None,
                }
            }
            false => None,
        };
        (selected, paths)
    }

    fn set_save_path(path: &Path) -> Option<UndoRedoState> {
        Some(UndoRedoState::from(Some(vec![IOCommand::SetPath(Some(
            path.to_path_buf(),
        ))])))
    }
}

impl Panel for OpenFilePanel {
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
            let mut s = text.get_with_values(
                "OPEN_FILE_PANEL_STATUS_TTS_CWD",
                &[&get_folder_name(&self.directory)],
            );
            s.push(' ');
            match self.selected {
                Some(selected) => {
                    let path = &self.paths[selected];
                    let name = if path.is_file {
                        text.get_with_values("FILE", &[&get_file_name_no_ex(&path.path)])
                    } else {
                        text.get_with_values("FILE", &[&get_folder_name(&path.path)])
                    };
                    s.push_str(
                        &text.get_with_values("OPEN_FILE_PANEL_STATUS_TTS_SELECTION", &[&name]),
                    );
                }
                None => s.push_str(&text.get("OPEN_FILE_PANEL_STATUS_TTS_NO_SELECTION")),
            }
            tts.say(&s);
        }
        // Input TTS.
        else if input.happened(&InputEvent::InputTTS) {
            let mut strings = vec![];
            // Up directory.
            if let Some(parent) = self.directory.parent() {
                strings.push(get_tooltip_with_values(
                    "OPEN_FILE_PANEL_INPUT_TTS_UP_DIRECTORY",
                    &[InputEvent::UpDirectory],
                    &[&get_folder_name(parent)],
                    input,
                    text,
                ))
            }
            // Scroll.
            if self.paths.len() > 1 {
                strings.push(get_tooltip(
                    "OPEN_FILE_PANEL_INPUT_TTS_SCROLL",
                    &[InputEvent::PreviousPath, InputEvent::NextPath],
                    input,
                    text,
                ));
            }
            if let Some(selected) = self.selected {
                let events = vec![InputEvent::SelectFile];
                let path = &self.paths[selected];
                match path.is_file {
                    // Select.
                    true => {
                        let open_file_key = match self.open_file_type {
                            OpenFileType::ReadSave => "OPEN_FILE_PANEL_INPUT_TTS_READ_SAVE",
                            OpenFileType::SoundFont => "OPEN_FILE_PANEL_INPUT_TTS_SOUNDFONT",
                            OpenFileType::WriteSave => "OPEN_FILE_PANEL_INPUT_TTS_WRITE_SAVE",
                        };
                        strings.push(get_tooltip_with_values(
                            open_file_key,
                            &events,
                            &[&get_file_name_no_ex(&path.path)],
                            input,
                            text,
                        ));
                    }
                    // Down directory.
                    false => strings.push(get_tooltip_with_values(
                        "OPEN_FILE_PANEL_INPUT_TTS_DOWN_DIRECTORY",
                        &[InputEvent::DownDirectory],
                        &[&get_folder_name(&path.path)],
                        input,
                        text,
                    )),
                }
            }
            // Close.
            strings.push(get_tooltip(
                "OPEN_FILE_PANEL_INPUT_TTS_CLOSE",
                &[InputEvent::CloseOpenFile],
                input,
                text,
            ));
            tts.say(&strings.join(" "));
        }
        // Go up a directory.
        else if input.happened(&InputEvent::UpDirectory) {
            if let Some(parent) = self.directory.parent() {
                self.directory = parent.to_path_buf();
                let (selected, paths) = self.get_paths();
                self.selected = selected;
                self.paths = paths;
            }
        }
        // Go down a directory.
        else if input.happened(&InputEvent::DownDirectory) {
            if let Some(selected) = self.selected {
                if !self.paths[selected].is_file {
                    self.directory = self.paths[selected].path.clone();
                    let (selected, paths) = self.get_paths();
                    self.selected = selected;
                    self.paths = paths;
                }
            }
        }
        // Scroll up.
        else if input.happened(&InputEvent::PreviousPath) {
            if let Some(selected) = self.selected {
                if selected > 0 {
                    self.selected = Some(selected - 1);
                }
            }
        }
        // Scroll down.
        else if input.happened(&InputEvent::NextPath) {
            if let Some(selected) = self.selected {
                if selected < self.paths.len() - 1 {
                    self.selected = Some(selected + 1);
                }
            }
        }
        // We selected something.
        else if input.happened(&InputEvent::SelectFile) {
            self.disable(state);
            match self.open_file_type {
                // Load a save file.
                OpenFileType::ReadSave => {
                    let mut filename = self.filename.clone();
                    filename.push_str(".cac");
                    let path = self.directory.join(filename);
                    *state = State::read(&path);
                    return OpenFilePanel::set_save_path(&path);
                }
                // Load a SoundFont.
                OpenFileType::SoundFont => {
                    if let Some(selected) = self.selected {
                        if self.paths[selected].is_file {
                            let channel = state.music.get_selected_track().unwrap().channel;
                            let c0 = vec![Command::UnsetProgram { channel }];
                            let c1 = vec![Command::LoadSoundFont {
                                channel,
                                path: self.paths[selected].path.clone(),
                            }];
                            let mut undo = UndoRedoState::from((c0, &c1));
                            // Add an IO command.
                            undo.undo.io_commands = Some(vec![IOCommand::DisableOpenFile]);
                            conn.send(c1);
                            return Some(undo);
                        }
                    }
                }
                // Write a save file.
                OpenFileType::WriteSave => {
                    let mut filename = self.filename.clone();
                    filename.push_str(".cac");
                    let path = self.directory.join(filename);
                    state.write(&path);
                    return OpenFilePanel::set_save_path(&path);
                }
            }
        }
        // Close this.
        else if input.happened(&InputEvent::CloseOpenFile) {
            self.disable(state);
            return Some(UndoRedoState::from(Some(vec![IOCommand::DisableOpenFile])));
        }
        None
    }
}
