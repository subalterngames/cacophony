use ini::Ini;
use crate::{MidiConn, NoteOn};
use cacophony_audio::{Conn, connect};

/// Listens for user input from qwerty and MIDI devices and records the current input state.
pub struct Input {
    /// The MIDI connection.
    midi_conn: Option<MidiConn>,
    /// The synthesizer-audio player `Conn`.
    pub conn: Conn,
    /// A buffer of raw MIDI messages polled on this frame.
    pub midi: Vec<[u8; 3]>,
    /// If true, we're armed and listing for input to add to music.
    pub armed: bool,
    // Note-on MIDI messages. These will be sent immediately to the synthesizer to be played.
    pub note_ons: Vec<[u8; 3]>,
    /// Note-on events that don't have corresponding off events.
    note_on_events: Vec<NoteOn>,
    /// Notes that were added after all note-off events are done.
    pub note_offs: Vec<[u8; 3]>,
}

impl Input {
    pub fn new(config: &Ini) -> Self {
        Self {midi_conn: MidiConn::new(config), conn: connect(), midi: vec![], armed: false, note_on_events: vec![], note_ons: vec![], note_offs: vec![] }
    }

    pub fn update(&mut self) {
        if let Some(midi_conn) = self.midi_conn {
            self.midi.clear();
            self.midi.extend(midi_conn.poll());

            // Append MIDI events.
            for mde in self.midi_events.iter_mut() {
                if mde.1.update(&self.midi) {
                    self.event_starts.push(*mde.0);
                }
            }

            // Get note-on and note-off events.
            for midi in self.midi.iter() {
                // Note-on.
                if midi[0] >= 144 && midi[0] <= 159 {
                    if self.armed {
                        // Remember the note-on for piano roll input.
                        self.note_on_events.push(NoteOn::new(midi));
                    }
                    // Copy this note to the immediate note-on array.
                    self.note_ons.push(*midi);
                }
                // Note-off.
                if self.armed && midi[0] >= 128 && midi[0] <= 143 {
                    // Find the corresponding note.
                    for note_on in self.note_on_events.iter_mut() {
                        // Same key. Note-off.
                        if note_on.note[1] == midi[1] {
                            note_on.off = true;
                        }
                    }
                }
            }
            // If all note-ons are off, add them to the `notes` buffer as notes.
            if !self.note_on_events.is_empty() && self.note_on_events.iter().all(|n| n.off) {
                for note_on in self.note_on_events.iter() {
                    self.note_offs.push(note_on.note);
                }
                self.note_on_events.clear();
            }
        }
    }
}