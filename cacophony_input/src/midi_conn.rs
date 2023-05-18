use cacophony_core::config::parse;
use crossbeam_channel::{unbounded, Receiver, Sender};
use ini::Ini;
use midir::{MidiInput, MidiInputConnection, MidiInputPort};

// The MIDI connection error message.
const MIDI_ERROR_MESSAGE: &str = "Couldn't connect to a MIDI input device";

/// The MIDI connection tries to open a connection to an input device. See: `[MIDI_DEVICES]` -> `input` in config.ini
///
/// If a connection is made, the MIDI context will listen for events. When `poll()` is called, the previous event buffer is cleared and a new one is assembled and returned.
pub(crate) struct MidiConn {
    /// The receiver end of the MIDI context channel. It receives 3-byte MIDI messages.
    receiver: Receiver<[u8; 3]>,
    /// The buffer of received MIDI messages since the last frame.
    pub(crate) buffer: Vec<[u8; 3]>,
    /// The maximum number of events we'll poll for.
    num_events: usize,
    /// The MIDI connection. We need this in order to keep the connection alive.
    _conn: Option<MidiInputConnection<Sender<[u8; 3]>>>,
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
        let num_events: usize = parse(midi_device_section, "num_events");

        // Open a thread-safe sender and receiver for MIDI events.
        let (sender, receiver) = unbounded();

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
                            sender,
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
            receiver,
            buffer: Vec::new(),
            num_events,
            _conn: conn,
        })
    }

    /// Poll the MIDI device for events.
    ///
    /// The maximum number of events is defined in config.ini: `[MIDI_DEVICES]` -> `num_events`.
    ///
    /// Returns a slice of 3-element u8 arrays.
    pub(crate) fn poll(&mut self) -> &[[u8; 3]] {
        // Clear the buffer of events.
        self.buffer.clear();
        for _ in 0..self.num_events {
            match self.receiver.try_recv() {
                Ok(resp) => self.buffer.push(resp),
                Err(_) => (),
            }
        }
        self.buffer.as_slice()
    }

    // The MIDI callback function. Send the message out of the thread.
    fn midi_callback(_: u64, message: &[u8], sender: &mut Sender<[u8; 3]>) {
        sender.send([message[0], message[1], message[2]]).unwrap();
    }
}
