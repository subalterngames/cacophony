use super::import_midi::import;
use crate::panel::*;
use crate::Save;
use audio::exporter::ExportType;
use common::open_file::*;
use common::PanelType;
use text::get_file_name_no_ex;

/// Data for an open-file panel.
#[derive(Default)]
pub struct OpenFilePanel {
    /// Popup handler.
    popup: Popup,
    /// Tooltips handler.
    tooltips: Tooltips,
}

impl OpenFilePanel {
    /// Enable the panel.
    fn enable(
        &mut self,
        open_file_type: OpenFileType,
        state: &mut State,
        paths_state: &mut PathsState,
    ) {
        let mut panels = vec![PanelType::OpenFile];
        // Show export settings.
        if open_file_type == OpenFileType::Export {
            panels.push(PanelType::ExportSettings);
        }
        self.popup.enable(state, panels);
        // Set the file type.
        paths_state.open_file_type = open_file_type;
    }

    /// Enable the panel for loading SoundFonts.
    pub fn soundfont(&mut self, state: &mut State, paths_state: &mut PathsState) {
        let open_file_type = OpenFileType::SoundFont;
        paths_state.children.set(
            &paths_state.soundfonts.directory.path,
            &Extension::Sf2,
            None,
        );
        self.enable(open_file_type, state, paths_state);
    }

    /// Enable the panel for setting the save path to be read from.
    pub fn read_save(&mut self, state: &mut State, paths_state: &mut PathsState) {
        self.enable_as_save(OpenFileType::ReadSave, state, paths_state);
    }

    /// Enable the panel for setting the save path to be written to.
    pub fn write_save(&mut self, state: &mut State, paths_state: &mut PathsState) {
        self.enable_as_save(OpenFileType::WriteSave, state, paths_state);
        paths_state
            .children
            .set(&paths_state.midis.directory.path, &Extension::Cac, None);
    }

    /// Enable a panel for setting the export path.
    pub fn export(
        &mut self,
        state: &mut State,
        paths_state: &mut PathsState,
        exporter: &SharedExporter,
    ) {
        let ex = exporter.lock();
        let extension = ex.export_type.get().into();
        let open_file_type = OpenFileType::Export;
        paths_state
            .children
            .set(&paths_state.exports.directory.path, &extension, None);
        self.enable(open_file_type, state, paths_state);
    }

    /// Enable a panel for importing a MIDI file.
    pub fn import_midi(&mut self, state: &mut State, paths_state: &mut PathsState) {
        paths_state
            .children
            .set(&paths_state.midis.directory.path, &Extension::Mid, None);
        self.enable(OpenFileType::ImportMidi, state, paths_state);
    }

    fn get_extension(&self, paths_state: &PathsState, exporter: &SharedExporter) -> Extension {
        match paths_state.open_file_type {
            OpenFileType::Export => {
                let ex = exporter.lock();
                ex.export_type.get().into()
            }
            OpenFileType::ReadSave | OpenFileType::WriteSave => Extension::Cac,
            OpenFileType::SoundFont => Extension::Sf2,
            OpenFileType::ImportMidi => Extension::Mid,
        }
    }

    fn enable_as_save(
        &mut self,
        open_file_type: OpenFileType,
        state: &mut State,
        paths_state: &mut PathsState,
    ) {
        paths_state
            .children
            .set(&paths_state.saves.directory.path, &Extension::Cac, None);
        self.enable(open_file_type, state, paths_state);
    }

    /// Disable this panel.
    pub fn disable(&self, state: &mut State) {
        self.popup.disable(state);
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
        exporter: &mut SharedExporter,
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
                if input.modify_filename_abc123(&mut filename) {
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
                &[&paths_state.get_directory().stem],
            );
            s.push(' ');
            // Export file type.
            if paths_state.open_file_type == OpenFileType::Export {
                let ex = exporter.lock();
                let e = ex.export_type.get();
                let extension: Extension = e.into();
                let export_type = extension.to_str(false);
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
                        text.get_with_values("FILE", &[get_file_name_no_ex(&path.path)])
                    } else {
                        text.get_with_values("FILE", &[&path.stem])
                    };
                    s.push_str(
                        &text.get_with_values("OPEN_FILE_PANEL_STATUS_TTS_SELECTION", &[&name]),
                    );
                }
                _ => s.push_str(text.get_ref("OPEN_FILE_PANEL_STATUS_TTS_NO_SELECTION")),
            }
            tts.enqueue(s);
        }
        // Input TTS.
        else if input.happened(&InputEvent::InputTTS) {
            let mut tts_strings = vec![];
            // Up directory.
            if let Some(parent) = paths_state.get_directory().path.parent() {
                tts_strings.push(self.tooltips.get_tooltip_with_values(
                    "OPEN_FILE_PANEL_INPUT_TTS_UP_DIRECTORY",
                    &[InputEvent::UpDirectory],
                    &[&FileOrDirectory::new(parent).stem],
                    input,
                    text,
                ))
            }
            // Scroll.
            if paths_state.children.children.len() > 1 {
                tts_strings.push(self.tooltips.get_tooltip(
                    "OPEN_FILE_PANEL_INPUT_TTS_SCROLL",
                    &[InputEvent::PreviousPath, InputEvent::NextPath],
                    input,
                    text,
                ));
            }
            // Set export type.
            if paths_state.open_file_type == OpenFileType::Export {
                let ex = exporter.lock();
                let mut index = ex.export_type;
                index.index.increment(true);
                let e = index.get();
                let extension: Extension = e.into();
                let next_export_type = extension.to_str(false);
                tts_strings.push(self.tooltips.get_tooltip_with_values(
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
                            OpenFileType::ImportMidi => "OPEN_FILE_PANEL_INPUT_TTS_IMPORT_MIDI",
                        };
                        tts_strings.push(self.tooltips.get_tooltip_with_values(
                            open_file_key,
                            &events,
                            &[get_file_name_no_ex(&path.path)],
                            input,
                            text,
                        ));
                    }
                    // Down directory.
                    false => tts_strings.push(self.tooltips.get_tooltip_with_values(
                        "OPEN_FILE_PANEL_INPUT_TTS_DOWN_DIRECTORY",
                        &[InputEvent::DownDirectory],
                        &[&path.stem],
                        input,
                        text,
                    )),
                }
            }
            // Close.
            tts_strings.push(self.tooltips.get_tooltip(
                "OPEN_FILE_PANEL_INPUT_TTS_CLOSE",
                &[InputEvent::CloseOpenFile],
                input,
                text,
            ));
            tts.enqueue(tts_strings);
        }
        // Go up a directory.
        else if input.happened(&InputEvent::UpDirectory) {
            paths_state.up_directory(&self.get_extension(paths_state, exporter));
        }
        // Go down a directory.
        else if input.happened(&InputEvent::DownDirectory) {
            paths_state.down_directory(&self.get_extension(paths_state, exporter));
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
            // Set the extension.
            let mut ex = exporter.lock();
            ex.export_type.index.increment(true);
            // Set the children.
            paths_state.children.set(
                &paths_state.exports.directory.path,
                &ex.export_type.get().into(),
                None,
            );
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
                        Save::read(&path, state, conn, paths_state, exporter);
                        // Set the saves directory.
                        paths_state.saves = FileAndDirectory::new_path(path);
                    }
                }
                // Load a SoundFont.
                OpenFileType::SoundFont => {
                    if let Some(selected) = paths_state.children.selected {
                        // Disable the panel.
                        self.disable(state);
                        if paths_state.children.children[selected].is_file {
                            // Get the selected track's channel.
                            let channel = state.music.get_selected_track().unwrap().channel;
                            // To revert: unset the program.
                            let c0 = vec![Command::UnsetProgram { channel }];
                            // A command to load the SoundFont.
                            let c1 = vec![Command::LoadSoundFont {
                                channel,
                                path: paths_state.children.children[selected].path.clone(),
                            }];
                            return Some(Snapshot::from_commands(c0, c1, conn));
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
                        state.unsaved_changes = false;
                        // Write.
                        Save::write(
                            &paths_state.saves.directory.path.join(filename),
                            state,
                            conn,
                            paths_state,
                            exporter,
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
                        let ex = exporter.lock();
                        filename.push_str(
                            <ExportType as Into<Extension>>::into(ex.export_type.get())
                                .to_str(true),
                        );
                        // Export to a .mid file.
                        if ex.export_type.get() == ExportType::Mid {
                            ex.mid(
                                &paths_state.exports.directory.path.join(filename),
                                &state.music,
                                &state.time,
                                &conn.state,
                            );
                        }
                        // Export an audio file.
                        else {
                            return Some(Snapshot::from_io_commands(vec![IOCommand::Export(
                                paths_state.exports.directory.path.join(filename),
                            )]));
                        }
                    }
                }
                OpenFileType::ImportMidi => {
                    if let Some(selected) = paths_state.children.selected {
                        let path = paths_state.children.children[selected].path.clone();
                        import(&path, state, conn, exporter);
                        state.unsaved_changes = true;
                        self.disable(state);
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

    fn on_disable_abc123(&mut self, _: &mut State, _: &mut SharedExporter) {}

    fn update_abc123(
        &mut self,
        _: &mut State,
        _: &Input,
        _: &mut SharedExporter,
    ) -> (Option<Snapshot>, bool) {
        // There is alphanumeric input in this struct, obviously, but we won't handle it here because we don't need to toggle it on/off.
        (None, false)
    }

    fn allow_alphanumeric_input(&self, _: &State, _: &SharedExporter) -> bool {
        false
    }

    fn allow_play_music(&self) -> bool {
        false
    }
}
