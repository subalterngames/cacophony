use audio::*;
use common::serde_json::{from_str, to_string, Error};
use common::{SerializableState, State};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

const READ_ERROR: &str = "Error reading file: ";
const WRITE_ERROR: &str = "Error writing file: ";

/// Serializable save data.
#[derive(Deserialize, Serialize)]
pub(crate) struct Save {
    state: SerializableState,
    synth_state: SynthState,
}

impl Save {
    /// Write this state to a file.
    ///
    /// - `path` The path we will write to.
    /// - `state` The app state. This will be converted to a `SerializableState`.
    /// - `conn` The audio connection. Its `SynthState` will be serialized.
    pub fn write(path: &PathBuf, state: &State, conn: &Conn) {
        // Convert the state to something that can be serialized.
        let save = Save {
            state: state.serialize(),
            synth_state: conn.state.clone(),
        };
        // Try to open the file.
        match OpenOptions::new()
            .write(true)
            .append(false)
            .truncate(true)
            .create(true)
            .open(path)
        {
            Ok(mut file) => {
                let s = match to_string(&save) {
                    Ok(s) => s,
                    Err(error) => panic!("{} {}", WRITE_ERROR, error),
                };
                if let Err(error) = file.write(s.as_bytes()) {
                    panic!("{} {}", WRITE_ERROR, error)
                }
            }
            Err(error) => panic!("{} {}", WRITE_ERROR, error),
        }
    }

    /// Load a file and deserialize.
    pub fn read(path: &PathBuf, state: &mut State, conn: &mut Conn) {
        match File::open(path) {
            Ok(mut file) => {
                let mut string = String::new();
                match file.read_to_string(&mut string) {
                    Ok(_) => {
                        let q: Result<Save, Error> = from_str(&string);
                        match q {
                            Ok(s) => {
                                // Set the app state.
                                *state = s.state.deserialize();

                                // Set the synthesizer.
                                // Set the gain.
                                let mut commands = vec![Command::SetGain {
                                    gain: s.synth_state.gain,
                                }];
                                // Set each program.
                                for program in s.synth_state.programs.iter() {
                                    let channel = *program.0;
                                    // Load the SoundFont.
                                    commands.push(Command::LoadSoundFont {
                                        channel,
                                        path: program.1.path.clone(),
                                    });
                                    // Set hte program.
                                    commands.push(Command::SetProgram {
                                        channel,
                                        path: program.1.path.clone(),
                                        bank_index: program.1.bank_index,
                                        preset_index: program.1.preset_index,
                                    });
                                }

                                // Set the synth state.
                                conn.state = s.synth_state;

                                // Send the commands.
                                conn.send(commands);
                            }
                            Err(error) => panic!("{} {}", READ_ERROR, error),
                        }
                    }
                    Err(error) => panic!("{} {}", READ_ERROR, error),
                }
            }
            Err(error) => panic!("{} {}", READ_ERROR, error),
        }
    }
}
