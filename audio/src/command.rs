use crate::export_state::ExportState;
use crate::exporter::Exporter;
use std::path::PathBuf;

/// A command for the synthesizer.
#[derive(Eq, PartialEq, Clone)]
pub enum Command {
    /// Set the synthesizer's framerate.
    SetFramerate { framerate: u32 },
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
    /// Schedule a note-on event.
    NoteOnAt {
        channel: u8,
        key: u8,
        velocity: u8,
        start: u64,
        end: u64,
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
    /// Ask for the export state.
    SendExportState,
    /// Set the exporter.
    SetExporter { exporter: Box<Exporter> },
    /// Append silence to all but the longest audio.
    AppendSilences { paths: Vec<PathBuf> },
}
