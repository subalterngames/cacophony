use std::path::PathBuf;

/// A command for the synthesizer.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Command {
    /// Load a SoundFont file.
    LoadSoundFont { channel: u8, path: PathBuf },
    /// Set a program.
    SetProgram {
        channel: u8,
        path: PathBuf,
        bank_index: usize,
        preset_index: usize,
    },
    /// Set the program to None.
    UnsetProgram { channel: u8 },
    /// Set the overall gain.
    SetGain { gain: u8 },
}
