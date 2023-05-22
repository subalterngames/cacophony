use common::open_file::OpenFileType;
use crate::panel::*;
use text::{get_folder_name, get_file_name_no_ex, push_space};
use crate::{get_tooltip, get_tooltip_with_values};

/// The file dialogue "popup" panel.
pub struct OpenFilePanel {
    /// This is is used by IO to decide whether to keep this panel active.
    pub enabled: bool,
    /// If true, we selected something.
    pub selected: bool,
    /// Text-to-speech string to close the panel.
    tts_close: String,
    /// Text-to-speech string to write a save file.
    tts_write_save: String,
    /// Text-to-speech if there is no selection.
    tts_no_selection: String,
    /// Text-to-speech to go down a directory.
    tts_down_directory: String,
    /// Text-to-speech to read a save file.
    tts_read_save: String,
    /// Text-to-speech to load a SoundFont.
    tts_load_soundfont: String,
}

impl OpenFilePanel {
    pub fn new(input: &Input, text: &Text) -> Self {
        let tts_close = get_tooltip("OPEN_FILE_TTS_CLOSE", &[InputEvent::CloseOpenFile], input, text);
        let tts_write_save = get_tooltip("OPEN_FILE_TTS_WRITE_SAVE", &[InputEvent::SelectFile], input, text);
        let tts_no_selection = text.get("OPEN_FILE_TTS_NO_SELECTION");
        let tts_down_directory = get_tooltip("OPEN_FILE_TTS_DOWN_DIRECTORY", &[InputEvent::DownDirectory], input, text);
        let tts_read_save = get_tooltip("OPEN_FILE_TTS_READ_SAVE", &[InputEvent::SelectFile], input, text);
        let tts_load_soundfont = get_tooltip("OPEN_FILE_TTS_LOAD_SOUNDFONT", &[InputEvent::SelectFile], input, text);
        Self { enabled: false, selected: false, tts_close, tts_write_save, tts_no_selection, tts_down_directory, tts_read_save, tts_load_soundfont }
    }

    /// Enable the panel.
    pub fn enable(&mut self) {
        self.enabled = true;
        self.selected = false;
    }
}

impl Panel for OpenFilePanel {
    fn update(
            &mut self,
            state: &mut State,
            _: &mut Conn,
            input: &Input,
            tts: &mut TTS,
            text: &Text,
        ) -> Option<UndoRedoState> {
        if let Some(open_file) = &mut state.open_file {
            // Text-to-speech.
            if input.happened(&InputEvent::PanelTTS) || input.happened(&InputEvent::SubPanelTTS) {
                // The current directory.
                let mut s = text.get_with_values("OPEN_FILE_TTS_DIRECTORY", &[&get_folder_name(&open_file.directory)]);
                s.push(' ');
                // Go up a directory.
                if let Some(parent) = open_file.directory.parent() {
                    // OPEN_FILE_TTS_UP_DIRECTORY,\0 to go up to folder \1.
                    let parent_name = get_folder_name(parent);
                    s.push_str(&get_tooltip_with_values("OPEN_FILE_TTS_UP_DIRECTORY", &[InputEvent::UpDirectory], &[&parent_name], input, text));
                    s.push(' ');
                }
                // Describe the selection.
                match open_file.selected {
                    Some(selected) => {
                        let path = &open_file.paths[selected];
                        let stem = if path.is_file { get_file_name_no_ex(&path.path) } else { get_folder_name(&path.path) };
                        s.push_str(&text.get_with_values("OPEN_FILE_TTS_SELECTION", &[&stem]));
                        // Do something with the file.
                        if path.is_file {
                            match open_file.open_file_type {
                                OpenFileType::SoundFont => {
                                    s.push(' ');
                                    s.push_str(&self.tts_load_soundfont);
                                }
                                OpenFileType::ReadSave => {
                                    s.push(' ');
                                    s.push_str(&self.tts_read_save);
                                }
                                _ => ()
                            }
                        }
                        // Down a directory.
                        else {
                            s.push(' ');
                            s.push_str(&self.tts_down_directory);
                        }
                    }
                    None => s.push_str(&self.tts_no_selection)
                }
                // Write a save.
                if let OpenFileType::WriteSave = open_file.open_file_type {
                    push_space(&mut s);
                    s.push_str(&self.tts_write_save);
                }
                // Close.
                push_space(&mut s);
                s.push_str(&self.tts_close);
                // Say it!
                tts.say(&s)
            }
            // Go up a directory.
            else if input.happened(&InputEvent::UpDirectory) {
                open_file.up_directory();
            }
            // Go down a directory.
            else if input.happened(&InputEvent::DownDirectory) {
                open_file.down_directory();
            }
            // Scroll up.
            else if input.happened(&InputEvent::PreviousPath) {
                open_file.previous_path();
            }
            // Scroll down.
            else if input.happened(&InputEvent::NextPath) {
                open_file.next_path();
            }
            // We selected something.
            else if input.happened(&InputEvent::SelectFile) {
                self.enabled = false;
                self.selected = true;
            }
            // Close this.
            else if input.happened(&InputEvent::CloseOpenFile) {
                self.enabled = false;
            }
        }
        else {
            panic!("This should never happen!")
        }
        None
    }
}