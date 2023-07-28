use midir::{MidiInput, MidiInputConnection, MidiInputPort};
use parking_lot::Mutex;
use std::sync::Arc;

/// The MIDI connection error message.
const MIDI_ERROR_MESSAGE: &str = "Couldn't connect to a MIDI input device";
/// Type alias for a growable MIDI buffer.
type MidiBuffer = Arc<Mutex<Vec<[u8; 3]>>>;

/// The MIDI connection tries to open a connection to each input device.
///
/// If a connection is made, the MIDI context will listen for events.
pub(crate) struct MidiConn {
    /// The buffer of received MIDI messages since the last frame.
    pub(crate) buffer: MidiBuffer,
    /// The MIDI connections. We need this in order to keep the connection alive.
    _conns: Vec<MidiInputConnection<MidiBuffer>>,
}

impl MidiConn {
    /// Returns a new MIDI context. Returns None if we can't find any input device, and prints a helpful message.
    pub(crate) fn new() -> Option<Self> {
        // Get the indices and names of the ports.
        let ports: Vec<(usize, String)> = match MidiInput::new("num ports") {
            Ok(midi_in) => midi_in
                .ports()
                .iter()
                .filter(|p| midi_in.port_name(p).is_ok())
                .enumerate()
                .map(|(i, p)| (i, midi_in.port_name(p).unwrap()))
                .collect(),
            Err(error) => {
                println!("{}: {}", MIDI_ERROR_MESSAGE, error);
                vec![]
            }
        };
        if ports.is_empty() {
            None
        } else {
            // The buffer than can be accessed by the `Input` struct.
            let buffer = Arc::new(Mutex::new(Vec::new()));
            let mut conns = vec![];
            for (index, name) in ports {
                // Get a new connection.
                if let Ok(midi_in) = MidiInput::new(&format!("{} {}", index, name)) {
                    let port: &MidiInputPort = &midi_in.ports()[index];
                    // The buffer that the MIDI input device is writing to.
                    let data = Arc::clone(&buffer);
                    match midi_in.connect(port, &name, Self::midi_callback, data) {
                        Ok(c) => conns.push(c),
                        Err(error) => {
                            println!("{}: {}", MIDI_ERROR_MESSAGE, error);
                            continue;
                        }
                    }
                }
            }
            if conns.is_empty() {
                None
            } else {
                Some(Self {
                    buffer,
                    _conns: conns,
                })
            }
        }
    }

    /// The MIDI callback function. Send the message out of the thread.
    fn midi_callback(_: u64, message: &[u8], sender: &mut MidiBuffer) {
        let mut m = [0u8; 3];
        m.copy_from_slice(&message[0..3]);
        let mut buffer = sender.lock();
        buffer.push(m);
    }
}
