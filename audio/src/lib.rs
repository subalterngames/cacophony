//! This crate handles all audio output in Cacophony:
//!
//! - `Player` handles the cpal audio output stream.
//! - `Conn` manages the connection between external crates (command input), the synthesizer, and the audio player.
//! - `Exporter` handles all exporting to disk.
//!
//! Various data structs are shared in a Arc<Mutex<T>> format. These aren't a unified struct because they need to be locked at different times.
//!
//! As far as external crates are concerned, it's only necessary to create a new Conn: `Conn::default()`.

mod command;
mod conn;
mod decayer;
pub(crate) mod event_queue;
pub(crate) mod event_type;
pub mod export;
pub mod exporter;
pub mod play_state;
mod player;
mod program;
mod synth_state;
pub(crate) mod timed_event;
mod types;
pub use crate::command::Command;
pub use crate::conn::Conn;
use crate::program::Program;
pub use crate::synth_state::SynthState;
pub(crate) use crate::types::{AudioBuffer, SharedEventQueue, SharedSynth};
pub use crate::types::{AudioMessage, CommandsMessage, SharedExportState, SharedPlayState};
use player::Player;
