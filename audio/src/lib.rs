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
pub mod export;
pub mod exporter;
pub(crate) mod midi_event_queue;
mod player;
mod program;
mod synth_state;
mod time_state;
pub(crate) mod timed_midi_event;
mod types;
pub use crate::command::Command;
pub use crate::conn::Conn;
use crate::program::Program;
pub use crate::synth_state::SynthState;
use crate::time_state::TimeState;
pub(crate) use crate::types::{AudioBuffer, SharedMidiEventQueue, SharedTimeState};
pub use crate::types::{AudioMessage, CommandsMessage, SharedExportState, SharedSynth};
use player::Player;
