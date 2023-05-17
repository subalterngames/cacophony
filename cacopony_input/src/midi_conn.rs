use midir::{MidiInput, MidiInputConnection, MidiInputPort};

// The MIDI connection error message.
const MIDI_ERROR_MESSAGE: &str = "Couldn't connect to a MIDI input device";

/// The MIDI context tries to open a connection to an input device. See: `[MIDI_DEVICES]` -> `input` in config.ini
///
/// If a connection is made, the MIDI context will listen for events. When `poll()` is called, the previous event buffer is cleared and a new one is assembled and returned.
struct MidiContext {
    /// The receiver end of the MIDI context channel. It receives 3-byte MIDI messages.
    receiver: Receiver<[u8; 3]>,
    /// The buffer of received MIDI messages since the last frame.
    buffer: Vec<[u8; 3]>,
    /// The maximum number of events we'll poll for.
    num_events: usize,
    /// The MIDI connection. We need this in order to keep the connection alive.
    _conn: Option<MidiInputConnection<Sender<[u8; 3]>>>,
}