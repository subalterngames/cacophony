//! This crate handles all user input.
//!
//! - `InputEvent` is an enum defining an event triggered by user input, e.g. a decrease in track volume.
//! - `Input` maps raw qwerty keycode and raw MIDI messages (control bindings) to input events. It updates per frame, reading input and storing new events.

mod input;
mod input_event;
mod keys;
mod midi_binding;
mod midi_conn;
mod mods;
mod note_on;
mod qwerty_binding;
pub use input::Input;
pub use input_event::InputEvent;
use keys::KEYS;
use midi_binding::MidiBinding;
use midi_conn::MidiConn;
use mods::{ALPHANUMERIC_INPUT_MODS, MODS};
use note_on::NoteOn;
use qwerty_binding::QwertyBinding;
