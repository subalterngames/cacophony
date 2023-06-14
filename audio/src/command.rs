use crate::export_state::ExportState;
use std::path::PathBuf;

/// A command for the synthesizer.
#[derive(Eq, PartialEq, Clone)]
pub enum Command {
    /// Send this to announce that we're playing music, as opposed to arbitrary user input audio.
    PlayMusic { time: u64 },
    /// Send this to stop playing music.
    StopMusic,
    /// Schedule a stop-all event.
    StopMusicAt { time: u64 },
    /// Stop all sound.
    SoundOff,
    /// Note-on ASAP.
    NoteOn {
        channel: u8,
        key: u8,
        velocity: u8,
        duration: u64,
    },
    /// Schedule a note-on event. `time` is the sample count from 0.
    NoteOnAt {
        channel: u8,
        key: u8,
        velocity: u8,
        time: u64,
        duration: u64,
    },
    /// Note-off ASAP.
    NoteOff { channel: u8, key: u8 },
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
    /// Export audio.
    Export { path: PathBuf, state: ExportState },
}
