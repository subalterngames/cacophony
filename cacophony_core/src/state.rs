use crate::music::SerializableMusic;
use crate::viewport::SerializableViewport;
use crate::{Music, Viewport};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, Error};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

const READ_ERROR: &str = "Error reading file: ";
const WRITE_ERROR: &str = "Error writing file: ";

pub struct State {
    /// If true, we will accept musical input.
    pub armed: bool,
    /// The music.
    pub music: Music,
    /// The viewport.
    pub viewport: Viewport,
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

    fn serialize(&self) -> String {
        let music = self.music.serialize();
        let viewport = self.viewport.serialize();
        let s = SerializableState {
            armed: self.armed,
            music,
            viewport,
        };
        match to_string(&s) {
            Ok(s) => s,
            Err(error) => panic!("{} {}", WRITE_ERROR, error),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SerializableState {
    /// If true, we will accept musical input.
    armed: bool,
    /// The serializable music.
    music: SerializableMusic,
    /// The serializable viewport.
    viewport: SerializableViewport,
}

impl SerializableState {
    fn deserialize(&self) -> State {
        let music = self.music.deserialize();
        let viewport = self.viewport.deserialize();
        State {
            armed: self.armed,
            music,
            viewport,
        }
    }
}
