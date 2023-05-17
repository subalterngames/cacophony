mod midi_conn;
mod input;
mod note_on;
pub(crate) use midi_conn::MidiConn;
pub(crate) use note_on::NoteOn;
pub use input::Input;