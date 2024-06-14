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
        const LEN: usize = 3;

        // There are a few 2-byte MIDI messages that need to be ignored.
        if message.len() != LEN {
            return;
        }
        let mut m = [0u8; LEN];
        m.copy_from_slice(message);
        let mut buffer = sender.lock();
        buffer.push(m);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::MidiConn;

    use midly::{live::LiveEvent, MidiMessage};
    use parking_lot::Mutex;

    #[test]
    fn midi_test() {
        // These messages should be ready.
        for midi_message in [
            MidiMessage::NoteOn {
                key: 60.into(),
                vel: 120.into(),
            },
            MidiMessage::NoteOff {
                key: 60.into(),
                vel: 120.into(),
            },
        ]
        .iter()
        .zip([144, 128])
        {
            let message = LiveEvent::Midi {
                channel: 0.into(),
                message: midi_message.0.clone(),
            };
            let mut buffer_conn = Arc::new(Mutex::new(Vec::new()));
            let mut buffer_message = Vec::new();
            message.write(&mut buffer_message).unwrap();
            MidiConn::midi_callback(0, &buffer_message, &mut buffer_conn);
            // The message was ready.
            let b = buffer_conn.lock();
            assert_eq!(b.len(), 1);
            assert_eq!(b[0], [midi_message.1, 60, 120]);
        }
        // These messages should be ignored.
        for ignore_message in [
            MidiMessage::ChannelAftertouch { vel: 5.into() },
            MidiMessage::ProgramChange { program: 0.into() },
        ] {
            let message = LiveEvent::Midi {
                channel: 0.into(),
                message: ignore_message,
            };
            let mut buffer_conn = Arc::new(Mutex::new(Vec::new()));
            let mut buffer_message = Vec::new();
            message.write(&mut buffer_message).unwrap();
            MidiConn::midi_callback(0, &buffer_message, &mut buffer_conn);
            // The message was ignored.
            assert_eq!(buffer_conn.lock().len(), 0);
        }
    }
}
