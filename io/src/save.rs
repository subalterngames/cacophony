use audio::exporter::Exporter;
use audio::SharedExporter;
use audio::*;
use common::{PathsState, State};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, Error};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const READ_ERROR: &str = "Error reading file: ";
const WRITE_ERROR: &str = "Error writing file: ";

/// Serializable save data.
#[derive(Deserialize, Serialize)]
pub(crate) struct Save {
    /// The app state.
    state: State,
    /// The synthesizer state.
    synth_state: SynthState,
    /// The paths state.
    paths_state: PathsState,
    /// The exporter state.
    exporter: Exporter,
    #[serde(default = "default_version")]
    version: String,
}

impl Save {
    /// Write this state to a file.
    ///
    /// - `path` The path we will write to.
    /// - `state` The app state.
    /// - `conn` The audio connection. Its `SynthState` will be serialized.
    /// - `paths_state` The paths state.
    /// - `exporter` The exporter.
    pub fn write(
        path: &PathBuf,
        state: &State,
        conn: &Conn,
        paths_state: &PathsState,
        exporter: &SharedExporter,
    ) {
        // Convert the state to something that can be serialized.
        let save = Save {
            state: state.clone(),
            synth_state: conn.state.clone(),
            paths_state: paths_state.clone(),
            exporter: exporter.lock().clone(),
            version: common::VERSION.to_string(),
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
    ///
    /// - `path` The path we read from.
    /// - `state` The app state, which will be set to a deserialized version.
    /// - `conn` The audio connection. Its `SynthState` will be set via commands derived from a deserialized version.
    /// - `paths_state` The paths state, which will be set to a deserialized version.
    /// - `exporter` The exporter.
    pub fn read(
        path: &Path,
        state: &mut State,
        conn: &mut Conn,
        paths_state: &mut PathsState,
        exporter: &mut SharedExporter,
    ) {
        match File::open(path) {
            Ok(mut file) => {
                let mut string = String::new();
                match file.read_to_string(&mut string) {
                    Ok(_) => {
                        // Repair the save file if needed.
                        let string = Self::fix_no_flac(&string);
                        let q: Result<Save, Error> = from_str(&string);
                        match q {
                            Ok(s) => {
                                // Set the app state.
                                *state = s.state;

                                // Set the paths.
                                *paths_state = s.paths_state;

                                // Set the exporter.
                                let mut ex = exporter.lock();
                                *ex = s.exporter;

                                // Set the synthesizer.
                                // Set the gain.
                                let mut commands = vec![Command::SetGain {
                                    gain: s.synth_state.gain,
                                }];
                                // Load each SoundFont.
                                for program in s.synth_state.programs.iter() {
                                    if !program.1.path.exists() {
                                        continue;
                                    }
                                    let channel = *program.0;
                                    commands.push(Command::LoadSoundFont {
                                        channel,
                                        path: program.1.path.clone(),
                                    });
                                }
                                // Set each program.                            // Load each SoundFont.
                                for program in s.synth_state.programs.iter() {
                                    if !program.1.path.exists() {
                                        continue;
                                    }
                                    let channel = *program.0;
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

    /// Fix the export types if this is pre-0.1.3, which didn't have Flac exporting.
    /// This apparently isn't possible to fix in Exporter via serde.
    fn fix_no_flac(string: &str) -> String {
        let re = Regex::new(r#"(("export_type":\{"values":\["Wav","Mid","MP3","Ogg"\],"index":\{"index":)([0-9]),"length":4\}\})"#).unwrap();
        re.replace(string, r#""export_type":{"values":["Wav","Mid","MP3","Ogg","Flac"],"index":{"index":0,"length":5}}"#).into()
    }
}

/// Pre-0.1.3, the version isn't in the save file. This is the default version.
fn default_version() -> String {
    "0.1.2".to_string()
}
