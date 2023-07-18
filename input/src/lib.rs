//! This crate handles all user input.
//!
//! - `InputEvent` is an enum defining an event triggered by user input, e.g. a decrease in track volume.
//! - `Input` maps raw qwerty keycode and raw MIDI messages (control bindings) to input events. It updates per frame, reading input and storing new events.

mod input;
mod input_event;
mod keys;
mod midi_binding;
mod midi_conn;
mod note_on;
mod qwerty_binding;
pub use input::Input;
pub use input_event::InputEvent;
pub use keys::KEYS;
use keys::{ALPHANUMERIC_INPUT_MODS, MODS};
use midi_binding::MidiBinding;
use midi_conn::MidiConn;
use note_on::NoteOn;
pub use qwerty_binding::QwertyBinding;
