//! This crate contains a variety of types that are shared throughout Cacophony.
//!
//! There are two app-state-level structs defined in this crate:
//!
//! 1. `State` is *most* of the app state. It contains any data that can be placed on the undo/redo stacks. Because the undo/redo stacks contain entire `State` structs, the struct needs to be as small as possible.
//! 2. `PathsState` The state of directories, files, etc. defined by the user navigating through open-file dialogues. This isn't part of `State` because nothing here should go on the undo/redo stacks.
//!
//! There are two other state objects that aren't defined in this crate:
//!
//! - `SynthState` (defined in `audio`).
//! - `Exporter` (defined in `audio`).
//!
//! `common` is designed such that any Cacophony crate can use it, but itself does not depend on any Cacophony crates.

pub mod args;
pub mod config;
mod effect;
mod event;
mod index;
mod input_state;
mod midi_track;
mod music;
mod note;
mod panel_type;
pub mod paths;
mod paths_state;
mod state;
pub mod time;
pub mod view;
pub use effect::{
    effect_type::{EffectType, MAX_PITCH_BEND},
    Effect,
};
pub use event::Event;
pub use index::Index;
mod indexed_values;
pub use indexed_values::IndexedValues;
pub use input_state::InputState;
pub use midi_track::MidiTrack;
pub use music::*;
pub use note::{Note, MAX_NOTE, MIDDLE_C, MIN_NOTE, NOTE_NAMES};
pub use panel_type::PanelType;
pub use paths::Paths;
pub use state::State;
use view::View;
mod edit_mode;
pub mod music_panel_field;
pub use edit_mode::*;
mod selection;
pub use selection::Selection;
mod piano_roll_mode;
pub use piano_roll_mode::PianoRollMode;
use std::env::current_dir;
use std::fs::{metadata, File};
use std::io::Read;
use std::path::{Path, PathBuf};
pub mod font;
pub mod open_file;
pub mod sizes;
pub use paths_state::PathsState;
mod u64_or_f32;
pub use self::time::*;
pub use u64_or_f32::*;
pub mod fraction;

/// The version that will be printed on-screen.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// The maximum volume.
pub const MAX_VOLUME: u8 = 127;

/// Read bytes from a file.
pub fn get_bytes(path: &Path) -> Vec<u8> {
    let metadata = metadata(path).unwrap();
    let mut f = File::open(path).unwrap();
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer).unwrap();
    buffer
}

/// Default directory for looking at the 'data/' folder.
pub fn get_default_data_folder() -> PathBuf {
    current_dir().unwrap().join("data")
}
