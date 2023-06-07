use crate::panel::*;
use crate::{get_tooltip, get_tooltip_with_values};
use common::open_file::*;
use common::{PanelType, Paths};
use std::path::Path;
use text::{get_file_name_no_ex, get_folder_name};

/// Data for an open-file panel.
#[derive(Default)]
pub struct OpenFilePanel {
    /// The open-file context.
    pub open_file: OpenFile,
    /// The index of the previously-focused panel.
    previous_focus: Index,
    /// The previously-active panels.
    previous_panels: Vec<PanelType>,
}

impl OpenFilePanel {
    /// Enable the panel.
    fn enable(&mut self, state: &mut State) {
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
        self.open_file.soundfont(paths);
        self.enable(state);
    }

    /// Enable a panel that can read save files.
    pub fn read_save(&mut self, paths: &Paths, state: &mut State) {
        self.open_file.read_save(paths);
        self.enable_as_save(paths, state);
    }

    /// Enable a panel that can write save files.
    pub fn write_save(&mut self, paths: &Paths, state: &mut State) {
        self.open_file.write_save(paths);
        self.enable_as_save(paths, state);
    }

    pub fn export(&mut self, paths: &Paths, state: &mut State) {
        self.open_file.export(paths);
        self.enable(state);
    }

    fn enable_as_save(&mut self, paths: &Paths, state: &mut State) {
        self.open_file.enable_as_save(paths);
        self.enable(state);
    }

    /// Disable this panel.
    pub fn disable(&self, state: &mut State) {
        state.input.alphanumeric_input = false;
        // Restore the panels.
        state.panels = self.previous_panels.clone();
        // Restore the focus.
        state.focus = self.previous_focus;
        // Restore undo/redo.
        state.input.can_undo = true;
    }

    fn set_save_path(path: &Path) -> Option<UndoRedoState> {
        Some(UndoRedoState::from(Some(vec![IOCommand::SetSavePath(
            Some(path.to_path_buf()),
        )])))
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
                &[&get_folder_name(&self.open_file.directory)],
            );
            s.push(' ');
            match self.open_file.selected {
                Some(selected) => {
                    let path = &self.open_file.paths[selected];
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
            if let Some(parent) = self.open_file.directory.parent() {
                strings.push(get_tooltip_with_values(
                    "OPEN_FILE_PANEL_INPUT_TTS_UP_DIRECTORY",
                    &[InputEvent::UpDirectory],
                    &[&get_folder_name(parent)],
                    input,
                    text,
                ))
            }
            // Scroll.
            if self.open_file.paths.len() > 1 {
                strings.push(get_tooltip(
                    "OPEN_FILE_PANEL_INPUT_TTS_SCROLL",
                    &[InputEvent::PreviousPath, InputEvent::NextPath],
                    input,
                    text,
                ));
            }
            if let Some(selected) = self.open_file.selected {
                let events = vec![InputEvent::SelectFile];
                let path = &self.open_file.paths[selected];
                match path.is_file {
                    // Select.
                    true => {
                        let open_file_key = match self.open_file.open_file_type.as_ref().unwrap() {
                            OpenFileType::ReadSave => "OPEN_FILE_PANEL_INPUT_TTS_READ_SAVE",
                            OpenFileType::Export => "OPEN_FILE_PANEL_INPUT_TTS_EXPORT",
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
            self.open_file.up_directory();
        }
        // Go down a directory.
        else if input.happened(&InputEvent::DownDirectory) {
            if let Some(selected) = self.open_file.selected {
                if !self.open_file.paths[selected].is_file {
                    self.open_file.directory = self.open_file.paths[selected].path.clone();
                    let (selected, paths) = self.open_file.get_paths();
                    self.open_file.selected = selected;
                    self.open_file.paths = paths;
                }
            }
        }
        // Scroll up.
        else if input.happened(&InputEvent::PreviousPath) {
            if let Some(selected) = self.open_file.selected {
                if selected > 0 {
                    self.open_file.selected = Some(selected - 1);
                }
            }
        }
        // Scroll down.
        else if input.happened(&InputEvent::NextPath) {
            if let Some(selected) = self.open_file.selected {
                if selected < self.open_file.paths.len() - 1 {
                    self.open_file.selected = Some(selected + 1);
                }
            }
        }
        // We selected something.
        else if input.happened(&InputEvent::SelectFile) {
            self.disable(state);
            match self.open_file.open_file_type.as_ref().unwrap() {
                // Load a save file.
                OpenFileType::ReadSave => {
                    let path = &self.open_file.paths[self.open_file.selected.unwrap()].path;
                    let mut filename = path.file_name().unwrap().to_str().unwrap().to_string();
                    filename.push_str(".cac");
                    let path = self.open_file.directory.join(filename);
                    *state = State::read(&path);
                    return OpenFilePanel::set_save_path(&path);
                }
                // Load a SoundFont.
                OpenFileType::SoundFont => {
                    if let Some(selected) = self.open_file.selected {
                        if self.open_file.paths[selected].is_file {
                            let channel = state.music.get_selected_track().unwrap().channel;
                            let c0 = vec![Command::UnsetProgram { channel }];
                            let c1 = vec![Command::LoadSoundFont {
                                channel,
                                path: self.open_file.paths[selected].path.clone(),
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
                    let mut filename = self.open_file.filename.as_ref().unwrap().clone();
                    filename.push_str(".cac");
                    let path = self.open_file.directory.join(filename);
                    state.write(&path);
                    return OpenFilePanel::set_save_path(&path);
                }
                // Set an export file.
                OpenFileType::Export => {
                    let mut filename = self.open_file.filename.as_ref().unwrap().clone();
                    filename.push_str(".wav");
                    let path = self.open_file.directory.join(filename);
                    return Some(UndoRedoState::from(Some(vec![IOCommand::SetExportPath(
                        path,
                    )])));
                }
            }
        }
        // Close this.
        else if input.happened(&InputEvent::CloseOpenFile) {
            return Some(UndoRedoState::from(Some(vec![IOCommand::DisableOpenFile])));
        }
        None
    }
}
