//! This crate handles all audio output in Cacophony:
//!
//! - `Player` handles the cpal audio output stream. It receives audio samples.
//! - `Synthesizer` handles the audio generator synthesizer. It runs in its own thread. It can receive commands and will try to send audio samples.
//! - `Conn` manages the connection between external crates (command input), the synthesizer (audio sample output), and the audio player.
//!
//! It is possible to rout synthesizer output to either a `Player` (to play the audio) or to a file buffer (to write to disk).
//!
//! There is one way to input:
//!
//! - `Command` is the enum value describing a synthesizer command.
//!
//! There are four data struct outputs that other crates in Cacophony can read:
//!
//! - `SynthState` describes the state of the synthesizer.
//! - `Program` is a struct found within `SynthState` that describes a single program (preset, bank, etc.).
//! - `TimeState` describes the current playback time.
//! - `ExportState` can be used to monitor how many bytes have been exported to a .wav file.
//!
//! As far as external crates are concerned, it's only necessary to call `connect()`, which sets everything up and returns a `Conn`.
//! The `Conn` accepts command input and stores each of the four data structs listed above.

mod command;
mod conn;
mod export_state;
mod message;
mod player;
mod program;
mod synth_state;
mod synthesizer;
mod time_state;
mod wav;
pub use crate::command::Command;
pub use crate::conn::Conn;
pub use crate::message::{AudioMessage, CommandsMessage};
use crate::program::Program;
pub use crate::synth_state::SynthState;
use crate::time_state::TimeState;
use crossbeam_channel::{bounded, unbounded};
pub use export_state::ExportState;
use player::Player;
use std::thread::spawn;

/// Start the synthesizer and the audio player. Returns a `conn`.
pub fn connect() -> Conn {
    let (send_commands, recv_commands) = unbounded();
    let (send_state, recv_state) = bounded(1);
    let (send_audio, recv_audio) = bounded(1);
    let (send_time, recv_time) = bounded(1);
    let (send_export, recv_export) = unbounded();

    // Spawn the synthesizer thread.
    spawn(move || {
        synthesizer::Synthesizer::start(
            recv_commands,
            send_audio,
            send_state,
            send_export,
            send_time,
        )
    });
    // Spawn the audio thread.
    let player = Player::new(recv_audio);
    // Get the conn.
    Conn::new(player, send_commands, recv_state, recv_export, recv_time)
}

#[cfg(test)]
mod tests {
    use crate::{connect, Command, CommandsMessage};
    use std::path::PathBuf;
    use std::thread::sleep;
    use std::time::Duration;

    const SF_PATH: &str = "tests/CT1MBGMRSV1.06.sf2";
    const CHANNEL: u8 = 0;
    const DURATION: u64 = 44100;
    const KEY: u8 = 60;
    const VELOCITY: u8 = 120;

    #[test]
    fn sf() {
        // Make sure we can load the file.
        assert!(std::fs::File::open(SF_PATH).is_ok());
        let mut conn = connect();
        let commands = vec![Command::LoadSoundFont {
            path: PathBuf::from(SF_PATH),
            channel: CHANNEL,
        }];
        // Make sure we can send commands.
        conn.send(commands);
        assert!(conn.state.programs.contains_key(&CHANNEL));
        let program = &conn.state.programs[&CHANNEL];
        assert_eq!(program.num_banks, 1);
        assert_eq!(program.bank_index, 0);
        assert_eq!(program.num_presets, 128);
        assert_eq!(program.preset_index, 0);
        assert_eq!(program.preset_name, "Piano 1");
    }

    #[test]
    fn audio() {
        let mut conn = connect();
        // Load the soundfont. set the program, and do a note-on.
        let path = PathBuf::from(SF_PATH);
        conn.send(vec![
            Command::LoadSoundFont {
                path: path.clone(),
                channel: CHANNEL,
            },
            Command::SetProgram {
                channel: CHANNEL,
                path,
                bank_index: 0,
                preset_index: 0,
            },
            Command::NoteOn {
                channel: CHANNEL,
                key: KEY,
                velocity: VELOCITY,
                duration: DURATION,
            },
        ]);
        // Listen!
        sleep(Duration::from_millis(500));
        // Schedule some events.
        let commands = get_commands();
        conn.send(commands);
        // Listen!
        sleep(Duration::from_secs(10));
    }

    fn get_commands() -> CommandsMessage {
        let dt = DURATION / 4;
        let num: u8 = 10;
        (0..num)
            .map(|i| Command::NoteOnAt {
                channel: CHANNEL,
                key: KEY + i,
                velocity: VELOCITY,
                time: DURATION * 3 + dt * i as u64,
                duration: DURATION,
            })
            .collect::<CommandsMessage>()
    }
}
