use crate::mid::to_mid;
use crate::panel::*;
use crate::{get_tooltip, get_tooltip_with_values, Save};
use common::export_settings::*;
use common::open_file::*;
use common::PanelType;
use text::{get_file_name_no_ex, get_folder_name};

/// Data for an open-file panel.
#[derive(Default)]
pub struct OpenFilePanel {
    /// The index of the previously-focused panel.
    previous_focus: Index,
    /// The previously-active panels.
    previous_panels: Vec<PanelType>,
}

impl OpenFilePanel {
    /// Enable the panel.
    fn enable(
        &mut self,
        open_file_type: OpenFileType,
        state: &mut State,
        paths_state: &mut PathsState,
    ) {
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
        // Set the file type.
        paths_state.open_file_type = open_file_type;
    }

    /// Enable the panel for loading SoundFonts.
    pub fn soundfont(&mut self, state: &mut State, paths_state: &mut PathsState) {
        let open_file_type = OpenFileType::SoundFont;
        paths_state
            .children
            .set(&paths_state.soundfonts.directory, &open_file_type, None);
        self.enable(open_file_type, state, paths_state);
    }

    /// Enable the panel for setting the save path to be read from.
    pub fn read_save(&mut self, state: &mut State, paths_state: &mut PathsState) {
        self.enable_as_save(OpenFileType::ReadSave, state, paths_state);
    }

    /// Enable the panel for setting the save path to be written to.
    pub fn write_save(&mut self, state: &mut State, paths_state: &mut PathsState) {
        self.enable_as_save(OpenFileType::WriteSave, state, paths_state);
    }

    /// Enable a panel for setting the export path.
    pub fn export(&mut self, state: &mut State, paths_state: &mut PathsState) {
        let open_file_type = OpenFileType::Export;
        paths_state
            .children
            .set(&paths_state.exports.directory, &open_file_type, None);
        self.enable(open_file_type, state, paths_state);
    }

    fn enable_as_save(
        &mut self,
        open_file_type: OpenFileType,
        state: &mut State,
        paths_state: &mut PathsState,
    ) {
        paths_state
            .children
            .set(&paths_state.saves.directory, &open_file_type, None);
        self.enable(open_file_type, state, paths_state);
    }

    /// Disable this panel.
    pub fn disable(&self, state: &mut State) {
        state.input.alphanumeric_input = false;
        // Restore the panels.
        state.panels = self.previous_panels.clone();
        // Restore the focus.
        state.focus = self.previous_focus;
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
        paths_state: &mut PathsState,
    ) -> Option<Snapshot> {
        match &paths_state.open_file_type {
            OpenFileType::SoundFont | OpenFileType::ReadSave => (),
            _ => {
                // Get a modifiable filename.
                let mut filename = match &paths_state.get_filename() {
                    Some(filename) => filename.clone(),
                    None => String::new(),
                };
                // Modify the path.
                if input.modify_string_abc123(&mut filename) {
                    paths_state.set_filename(&filename);
                    return None;
                }
            }
        }
        // Status TTS.
        if input.happened(&InputEvent::StatusTTS) {
            // Current working directory.
            let mut s = text.get_with_values(
                "OPEN_FILE_PANEL_STATUS_TTS_CWD",
                &[&get_folder_name(paths_state.get_directory())],
            );
            s.push(' ');
            // Export file type.
            if paths_state.open_file_type == OpenFileType::Export {
                let export_type =
                    EXPORT_TYPE_STRINGS[paths_state.export_settings.export_type.get()];
                s.push_str(
                    &text.get_with_values("OPEN_FILE_PANEL_STATUS_TTS_EXPORT", &[export_type]),
                );
                s.push(' ');
            }
            // Selection.
            match paths_state.children.selected {
                Some(selected) => {
                    let path = &paths_state.children.children[selected];
                    let name = if path.is_file {
                        text.get_with_values("FILE", &[&get_file_name_no_ex(&path.path)])
                    } else {
                        text.get_with_values("FILE", &[&get_folder_name(&path.path)])
                    };
                    s.push_str(
                        &text.get_with_values("OPEN_FILE_PANEL_STATUS_TTS_SELECTION", &[&name]),
                    );
                }
                _ => s.push_str(&text.get("OPEN_FILE_PANEL_STATUS_TTS_NO_SELECTION")),
            }
            tts.say(&s);
        }
        // Input TTS.
        else if input.happened(&InputEvent::InputTTS) {
            let mut strings = vec![];
            // Up directory.
            if let Some(parent) = paths_state.get_directory().parent() {
                strings.push(get_tooltip_with_values(
                    "OPEN_FILE_PANEL_INPUT_TTS_UP_DIRECTORY",
                    &[InputEvent::UpDirectory],
                    &[&get_folder_name(parent)],
                    input,
                    text,
                ))
            }
            // Scroll.
            if paths_state.children.children.len() > 1 {
                strings.push(get_tooltip(
                    "OPEN_FILE_PANEL_INPUT_TTS_SCROLL",
                    &[InputEvent::PreviousPath, InputEvent::NextPath],
                    input,
                    text,
                ));
            }
            // Set export type.
            if paths_state.open_file_type == OpenFileType::Export {
                let mut index = paths_state.export_settings.export_type;
                index.increment(true);
                let next_export_type = EXPORT_TYPE_STRINGS[index.get()];
                strings.push(get_tooltip_with_values(
                    "OPEN_FILE_PANEL_INPUT_TTS_CYCLE_EXPORT",
                    &[InputEvent::CycleExportType],
                    &[next_export_type],
                    input,
                    text,
                ));
            }
            // Selection.
            if let Some(selected) = paths_state.children.selected {
                let events = vec![InputEvent::SelectFile];
                let path = &paths_state.children.children[selected];
                match path.is_file {
                    // Select.
                    true => {
                        let open_file_key = match paths_state.open_file_type {
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
            paths_state.up_directory();
        }
        // Go down a directory.
        else if input.happened(&InputEvent::DownDirectory) {
            paths_state.down_directory();
        }
        // Scroll up.
        else if input.happened(&InputEvent::PreviousPath) {
            paths_state.scroll(true);
        }
        // Scroll down.
        else if input.happened(&InputEvent::NextPath) {
            paths_state.scroll(false);
        }
        // Export type.
        else if paths_state.open_file_type == OpenFileType::Export
            && input.happened(&InputEvent::CycleExportType)
        {
            paths_state.export_settings.export_type.increment(true);
        }
        // We selected something.
        else if input.happened(&InputEvent::SelectFile) {
            // Do something with the selected file.
            match &paths_state.open_file_type {
                // Load a save file.
                OpenFileType::ReadSave => {
                    if let Some(selected) = paths_state.children.selected {
                        // Disable the panel.
                        self.disable(state);
                        // Get the path.
                        let path = paths_state.children.children[selected].path.clone();
                        // Read the save file.
                        Save::read(&path, state, conn, paths_state);
                        // Set the saves directory.
                        paths_state.saves = FileAndDirectory::new_path(path);
                    }
                }
                // Load a SoundFont.
                OpenFileType::SoundFont => {
                    if let Some(selected) = paths_state.children.selected {
                        if paths_state.children.children[selected].is_file {
                            // Disable the panel.
                            self.disable(state);
                            // Get the selected track's channel.
                            let channel = state.music.get_selected_track().unwrap().channel;
                            // To revert: unset the program.
                            let c0 = vec![Command::UnsetProgram { channel }];
                            // A command to load the SoundFont.
                            let c1 = vec![Command::LoadSoundFont {
                                channel,
                                path: paths_state.children.children[selected].path.clone(),
                            }];
                            // *click*
                            let snapshot = Snapshot::from_commands(c0, &c1);
                            // Send the commands.
                            conn.send(c1);
                            return Some(snapshot);
                        }
                    }
                }
                // Write a save file.
                OpenFileType::WriteSave => {
                    // There is a filename.
                    if let Some(filename) = &paths_state.saves.filename {
                        // Disable the panel.
                        self.disable(state);
                        // Append the extension.
                        let mut filename = filename.clone();
                        filename.push_str(".cac");
                        // Write.
                        Save::write(
                            &paths_state.saves.directory.join(filename),
                            state,
                            conn,
                            paths_state,
                        );
                    }
                }
                // Write an export file.
                OpenFileType::Export => {
                    // There is a filename.
                    if let Some(filename) = &paths_state.exports.filename {
                        // Disable the panel.
                        self.disable(state);
                        // Append the extension.
                        let mut filename = filename.clone();
                        match &EXPORT_TYPES[paths_state.export_settings.export_type.get()] {
                            // Export to a .wav file.
                            ExportType::Wav => {
                                filename.push_str(".wav");
                                return Some(Snapshot::from_io_commands(vec![IOCommand::Export(
                                    paths_state.exports.directory.join(filename),
                                )]));
                            }
                            // Export to a .mid file.
                            ExportType::Mid => {
                                filename.push_str(".mid");
                                to_mid(
                                    &paths_state.exports.directory.join(filename),
                                    &state.music,
                                    &state.time,
                                    &conn.state,
                                    text,
                                    &paths_state.export_settings,
                                );
                            }
                        }
                    }
                }
            }
        }
        // Close this.
        else if input.happened(&InputEvent::CloseOpenFile) {
            self.disable(state);
        }
        None
    }
}
