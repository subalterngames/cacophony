//! This crate handles all audio output in Cacophony:
//!
//! - `Player` handles the cpal audio output stream. It receives audio samples.
//! - `Synthesizer` handles the audio generator synthesizer. It runs in its own thread. It can receive commands and will try to send audio samples.
//! - `Conn` manages the connection between external crates (command input), the synthesizer (audio sample output), and the audio player.
//! - `Exporter` handles all exporting. This is different from writing samples; see its documentation.
//!
//! It is possible to rout synthesizer output to either a `Player` (to play the audio) or to a file buffer (to write to disk).
//!
//! There is one way to input:
//!
//! - `Command` is the enum value describing a synthesizer command.
//!
//! There are four data struct outputs that other crates in Cacophony can read, and may be sent by the `Conn`:
//!
//! - `SynthState` describes the state of the synthesizer.
//! - `Program` is a struct found within `SynthState` that describes a single program (preset, bank, etc.).
//! - `TimeState` describes the current playback time.
//! - `ExportState` can be used to monitor how many bytes have been exported to a .wav file.
//!
//! As far as external crates are concerned, it's only necessary to do the following:
//!
//! 1. Create a shared exporter on the main thread: `Exporter::new_shared()`.
//! 2. Call `connect()` on the main thread, which sets up everything else and returns a `Conn`.

mod command;
mod conn;
mod export;
mod export_state;
pub mod exporter;
pub(crate) mod midi_event_queue;
mod player;
mod program;
mod synth_state;
mod synthesizer;
mod time_state;
pub(crate) mod timed_midi_event;
mod types;
pub use crate::command::Command;
pub use crate::conn::Conn;
use crate::program::Program;
pub use crate::synth_state::SynthState;
use crate::time_state::TimeState;
pub(crate) use crate::types::{AudioBuffer, SharedMidiEventQueue, SharedTimeState, SharedSample};
pub use crate::types::{AudioMessage, CommandsMessage, SharedExporter, SharedSynth};
use crossbeam_channel::{bounded, unbounded};
pub use export_state::ExportState;
use player::Player;
use std::sync::Arc;
use std::thread::spawn;

/// Start the synthesizer and the audio player. Returns a `conn`.
pub fn connect(exporter: &SharedExporter) -> Conn {
    let (send_commands, recv_commands) = unbounded();
    let (send_state, recv_state) = bounded(1);
    let (send_audio, recv_audio) = bounded(1);
    let (send_time, recv_time) = bounded(1);
    let (send_export, recv_export) = bounded(1);
    let (send_sample, recv_sample) = bounded(1);

    let ex = Arc::clone(exporter);
    // Spawn the synthesizer thread.
    spawn(move || {
        synthesizer::Synthesizer::start(
            recv_commands,
            send_audio,
            send_state,
            send_export,
            send_time,
            send_sample,
            ex,
        )
    });
    // Spawn the audio thread.
    let player = Player::new(recv_audio);
    // Get the conn.
    Conn::new(
        player,
        send_commands,
        recv_state,
        recv_export,
        recv_time,
        recv_sample,
    )
}

#[cfg(test)]
mod tests {
    use crate::exporter::Exporter;
    use crate::{connect, Command};
    use std::path::PathBuf;

    const SF_PATH: &str = "tests/CT1MBGMRSV1.06.sf2";
    const CHANNEL: u8 = 0;

    #[test]
    fn sf() {
        // Make sure we can load the file.
        assert!(std::fs::File::open(SF_PATH).is_ok());
        let exporter = Exporter::new_shared();
        let mut conn = connect(&exporter);
        let commands = vec![Command::LoadSoundFont {
            path: PathBuf::from(SF_PATH),
            channel: CHANNEL,
        }];
        // Make sure we can send commands.
        conn.send(commands);
        assert!(conn.state.programs.contains_key(&CHANNEL));
        let program = &conn.state.programs[&CHANNEL];
        assert_eq!(program.num_banks, 2);
        assert_eq!(program.bank_index, 0);
        assert_eq!(program.num_presets, 128);
        assert_eq!(program.preset_index, 0);
        assert_eq!(program.preset_name, "Piano 1");
    }
}
