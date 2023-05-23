use crate::edit_mode::EDIT_MODES;
use crate::input_state::SerializableInputState;
use crate::music::SerializableMusic;
use crate::music_panel_field::{MusicPanelField, MUSIC_PANEL_FIELDS};
use crate::time::{SerializableTime, Time};
use crate::viewport::SerializableViewport;
use crate::{Index, InputState, Music, PanelType, Viewport};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, Error};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

const READ_ERROR: &str = "Error reading file: ";
const WRITE_ERROR: &str = "Error writing file: ";

#[derive(Clone)]
pub struct State {
    /// The music.
    pub music: Music,
    /// The viewport.
    pub viewport: Viewport,
    /// The time state.
    pub time: Time,
    /// The input state.
    pub input: InputState,
    /// A list of all panels that need to be drawn.
    pub panels: Vec<PanelType>,
    /// The index of the focused panel.
    pub focus: Index,
    /// The currently-selected music panel field.
    pub music_panel_field: Index,
    /// The index of the current piano roll edit mode.
    pub edit_mode: Index,
}

impl State {
    /// Write this state to a file.
    pub fn write(&self, path: &PathBuf) {
        match OpenOptions::new()
            .write(true)
            .append(false)
            .truncate(true)
            .create(true)
            .open(path)
        {
            Ok(mut file) => {
                let s = self.serialize();
                if let Err(error) = file.write(s.as_bytes()) {
                    panic!("{} {}", WRITE_ERROR, error)
                }
            }
            Err(error) => panic!("{} {}", WRITE_ERROR, error),
        }
    }

    /// Load a file and deserialize its contents to a `State`.
    pub fn read(path: &PathBuf) -> State {
        match File::open(path) {
            Ok(mut file) => {
                let mut string = String::new();
                match file.read_to_string(&mut string) {
                    Ok(_) => {
                        let q: Result<SerializableState, Error> = from_str(&string);
                        match q {
                            Ok(s) => s.deserialize(),
                            Err(error) => panic!("{} {}", READ_ERROR, error),
                        }
                    }
                    Err(error) => panic!("{} {}", READ_ERROR, error),
                }
            }
            Err(error) => panic!("{} {}", READ_ERROR, error),
        }
    }

    pub fn get_music_panel_field(&self) -> &MusicPanelField {
        &MUSIC_PANEL_FIELDS[self.music_panel_field.get()]
    }

    fn serialize(&self) -> String {
        let music = self.music.serialize();
        let viewport = self.viewport.serialize();
        let input = self.input.serialize();
        let time = self.time.serialize();
        let s = SerializableState {
            input,
            music,
            viewport,
            time,
            panels: self.panels.clone(),
            focus: self.focus.get(),
            music_panel_field: self.music_panel_field.get(),
            edit_mode: self.edit_mode.get(),
        };
        match to_string(&s) {
            Ok(s) => s,
            Err(error) => panic!("{} {}", WRITE_ERROR, error),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SerializableState {
    /// The input state.
    input: SerializableInputState,
    /// The serializable music.
    music: SerializableMusic,
    /// The serializable viewport.
    viewport: SerializableViewport,
    /// The serializable time state.
    time: SerializableTime,
    /// A list of all panels that need to be drawn.
    panels: Vec<PanelType>,
    /// The index of the focused panel.
    focus: usize,
    /// The currently-selected music panel field.
    music_panel_field: usize,
    /// The index of the current piano roll edit mode.
    edit_mode: usize,
}

impl SerializableState {
    fn deserialize(&self) -> State {
        let music = self.music.deserialize();
        let viewport = self.viewport.deserialize();
        let input = self.input.deserialize();
        let time = self.time.deserialize();
        let panels = self.panels.clone();
        let focus = Index::new(self.focus, panels.len());
        let music_panel_field = Index::new(self.music_panel_field, MUSIC_PANEL_FIELDS.len());
        let edit_mode = Index::new(self.edit_mode, EDIT_MODES.len());
        State {
            input,
            music,
            viewport,
            time,
            panels,
            focus,
            music_panel_field,
            edit_mode,
        }
    }
}
