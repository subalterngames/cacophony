//! This crate contains a variety of types that are shared throughout Cacophony.
//!
//! The most important struct by far is `State`, which stores the entire app state.
//! `State` can be serialized and written to disk.
//!
//! Most of the external crates, e.g. macroquad and fraction, can be referenced elsewhere from this crate.
//! There are a few notable exceptions:
//!
//! - `strum` and `serde` (and their respective derived crates) apparently need to be in a given crate's Cargo.toml to allow the macros to work.
//! - There are a few crates that are referenced by only 1 Cacophony crate and the dependency is therefore over in that Cacophony crate.
//!
//! `common` is designed such that any Cacophony crate can use it, but itself does not depend on any Cacophony crates.

pub mod config;
mod fraction_utils;
mod index;
mod input_state;
mod midi_track;
mod music;
mod note;
mod panel_type;
mod paths;
mod paths_state;
mod state;
pub mod time;
mod view;
pub use crossbeam_channel;
pub use csv;
pub use fraction::{One, ToPrimitive, Zero};
pub use fraction_utils::Fraction;
use fraction_utils::{deserialize_fraction, serialize_fraction, SerializableFraction};
pub use hashbrown;
pub use index::Index;
pub use ini;
pub use input_state::InputState;
pub use macroquad;
pub use midi_track::MidiTrack;
pub use midir;
pub use music::{Music, DEFAULT_BPM, DEFAULT_MUSIC_NAME};
pub use note::{Note, MAX_NOTE, MAX_VOLUME, MIN_NOTE};
pub use panel_type::PanelType;
pub use paths::Paths;
pub use state::{SerializableState, State};
pub use tts;
use view::View;
mod edit_mode;
pub mod music_panel_field;
pub use edit_mode::{EditMode, EDIT_MODES};
mod select_mode;
pub use select_mode::SelectMode;
mod piano_roll_mode;
pub use piano_roll_mode::PianoRollMode;
use std::fs::{metadata, File};
use std::io::Read;
pub mod font;
pub mod open_file;
pub mod sizes;
pub use paths_state::PathsState;
pub use serde_json;

/// The version that will be printed on-screen.
pub const VERSION: &str = "0.1.0";

/// Read bytes from a file.
pub fn get_bytes(path: &str) -> Vec<u8> {
    let metadata = metadata(path).unwrap();
    let mut f = File::open(path).unwrap();
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer).unwrap();
    buffer
}
