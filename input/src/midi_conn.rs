use common::config::parse;
use ini::Ini;
use midir::{MidiInput, MidiInputConnection, MidiInputPort};
use parking_lot::Mutex;
use std::sync::Arc;

/// The MIDI connection error message.
const MIDI_ERROR_MESSAGE: &str = "Couldn't connect to a MIDI input device";
/// Type alias for a growable MIDI buffer.
type MidiBuffer = Arc<Mutex<Vec<[u8; 3]>>>;

/// The MIDI connection tries to open a connection to an input device. See: `[MIDI_DEVICES]` -> `input` in config.ini
///
/// If a connection is made, the MIDI context will listen for events. When `poll()` is called, the previous event buffer is cleared and a new one is assembled and returned.
pub(crate) struct MidiConn {
    /// The buffer of received MIDI messages since the last frame.
    pub(crate) buffer: MidiBuffer,
    /// The MIDI connection. We need this in order to keep the connection alive.
    _conn: Option<MidiInputConnection<MidiBuffer>>,
}

impl MidiConn {
    /// Returns a new MIDI context. Returns None if we can't find an input device, and prints a helpful message.
    ///
    /// - `config` The config file that has user-defined MIDI settings.
    pub(crate) fn new(config: &Ini) -> Option<Self> {
        // Get the MIDI device section.
        let midi_device_section = config.section(Some("MIDI_DEVICES")).unwrap();
        // Get the port index and the max number of events.
        let port_index: usize = parse(midi_device_section, "input");
        // The buffer than can be accessed by the `Input` struct.
        let buffer = Arc::new(Mutex::new(Vec::new()));
        // The buffer that the MIDI input device is writing to.
        let data = Arc::clone(&buffer);

        // Try to open a MIDI input connection.
        let conn = match MidiInput::new("midir input") {
            // We have a MIDI Input interface.
            Ok(midi_in) => {
                // Is the port index in config.ini valid?
                let num_ports: usize = midi_in.port_count();
                if port_index >= num_ports {
                    println!(
                        "{}: Requested port {} but there are only {} ports.",
                        MIDI_ERROR_MESSAGE, port_index, num_ports
                    );
                    None
                } else {
                    // Get the MIDI port.
                    let midi_port: &MidiInputPort = &midi_in.ports()[port_index];
                    // Can we get the name of the device?
                    match midi_in.port_name(midi_port) {
                        // Can we open a connection?
                        Ok(port_name) => match midi_in.connect(
                            midi_port,
                            port_name.as_str(),
                            Self::midi_callback,
                            data,
                        ) {
                            Ok(c) => Some(c),
                            Err(error) => {
                                println!("{}: {}", MIDI_ERROR_MESSAGE, error);
                                None
                            }
                        },
                        Err(error) => {
                            println!("{}: {}", MIDI_ERROR_MESSAGE, error);
                            None
                        }
                    }
                }
            }
            Err(error) => {
                println!("{}: {}", MIDI_ERROR_MESSAGE, error);
                None
            }
        };
        Some(Self {
            buffer,
            _conn: conn,
        })
    }

    /// The MIDI callback function. Send the message out of the thread.
    fn midi_callback(_: u64, message: &[u8], sender: &mut MidiBuffer) {
        let mut m = [0u8; 3];
        m.copy_from_slice(&message[0..3]);
        let mut buffer = sender.lock();
        buffer.push(m);
    }
}
