pub mod config;
mod fraction_utils;
mod index;
mod input_state;
mod midi_track;
mod music;
mod note;
mod panel_type;
mod paths;
mod state;
pub mod time;
mod view;
pub use crossbeam_channel;
pub use csv;
pub use fraction::{ToPrimitive, Zero};
pub use fraction_utils::Fraction;
pub(crate) use fraction_utils::{deserialize_fraction, serialize_fraction, SerializableFraction};
pub use hashbrown;
pub use index::Index;
pub use ini;
pub use input_state::InputState;
pub use macroquad;
pub use midi_track::MidiTrack;
pub use midir;
pub use music::Music;
pub use note::{Note, MAX_NOTE, MAX_VOLUME, MIN_NOTE};
pub use panel_type::PanelType;
pub use paths::Paths;
pub use state::State;
pub use tts;
pub(crate) use view::View;
pub(crate) mod edit_mode;
pub mod music_panel_field;
pub use edit_mode::{EditMode, EDIT_MODES};
mod select_mode;
pub use select_mode::SelectMode;
mod piano_roll_mode;
pub use piano_roll_mode::PianoRollMode;
use std::fs::{metadata, File};
use std::io::Read;

/// Read bytes from a file.
pub fn get_bytes(path: &str) -> Vec<u8> {
    let metadata = metadata(path).unwrap();
    let mut f = File::open(path).unwrap();
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer).unwrap();
    buffer
}