/// A command for the synthesizer.
#[derive(Eq, PartialEq, Clone)]
pub enum Command {
    /// Send this to announce that we're playing music, as opposed to arbitrary user input audio.
    PlayMusic,
    /// Send this to stop playing music.
    StopMusic,
    /// Stop playing audio. Set the state to not-playing.
    StopAll { channels: Vec<u8> },
    /// Schedule a stop-all event.
    StopAllAt { channels: Vec<u8>, time: u64 },
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
    /// Schedule a note-off event. `time` is the sample count from 0.
    NoteOffAt { channel: u8, key: u8, time: u64 },
    /// Load a SoundFont file.
    LoadSoundFont { channel: u8, path: String },
    /// Set a program.
    SetProgram {
        channel: u8,
        path: String,
        bank_index: usize,
        preset_index: usize,
    },
    /// Set the program to None.
    UnsetProgram { channel: u8 },
    /// Set the overall gain.
    SetGain { gain: u8 },
    /// Export audio.
    Export { commands: Vec<Command> },
    /// Set the time.
    SetTime { time: u64 },
}
