use crate::{InputEvent, MidiConn, MidiBinding, NoteOn, QwertyBinding, KEYS};
use cacophony_audio::{connect, Conn};
use cacophony_core::State;
use derivative::Derivative;
use hashbrown::HashMap;
use ini::Ini;
use macroquad::input::*;

/// Listens for user input from qwerty and MIDI devices and records the current input state.
#[derive(Derivative)]
#[derivative(Default)]
pub struct Input {
    /// Events that began on this frame (usually due to a key press or MIDI controller message).
    pub event_starts: Vec<InputEvent>,
    /// Events that began on a previous frame and have continued on this frame. See: `[TIME]` -> `held_time_frames` in config.ini.
    pub event_continues: Vec<InputEvent>,
    /// The MIDI connection.
    midi_conn: Option<MidiConn>,
    /// The synthesizer-audio player `Conn`.
    #[derivative(Default(value = "connect()"))]
    pub conn: Conn,
    /// A buffer of raw MIDI messages polled on this frame.
    pub midi: Vec<[u8; 3]>,
    // Note-on MIDI messages. These will be sent immediately to the synthesizer to be played.
    pub note_ons: Vec<[u8; 3]>,
    /// Note-on events that don't have corresponding off events.
    note_on_events: Vec<NoteOn>,
    /// Notes that were added after all note-off events are done.
    pub new_notes: Vec<[u8; 3]>,
    /// Input events generated by MIDI input.
    midi_events: HashMap<InputEvent, MidiBinding>,
    /// Input events generated by qwerty input.
    qwerty_events: HashMap<InputEvent, QwertyBinding>,
    /// Was backspace pressed on this frame?
    backspace: bool,
    /// Characters pressed on this frame.
    pub pressed_chars: Vec<char>,
}

impl Input {
    pub fn new(config: &Ini) -> Self {
        let midi_conn = MidiConn::new(config);
        let conn = connect();
        Self {
            midi_conn,
            conn,
            ..Default::default()
        }
    }

    pub fn update(&mut self, state: &State) {
        // QWERTY INPUT.

        // Was backspace pressed?
        self.backspace = is_key_down(KeyCode::Backspace);
        // Get the pressed characters.
        self.pressed_chars.clear();
        while let Some(c) = get_char_pressed() {
            self.pressed_chars.push(c);
        }
        // Get all pressed keys.
        let pressed: Vec<KeyCode> = KEYS.iter().filter(|&k| is_key_pressed(*k)).map(|k| *k).collect();
        // Get all held keys.
        let down: Vec<KeyCode> = KEYS.iter().filter(|&k| is_key_down(*k)).map(|k| *k).collect();

        // Update the qwerty key bindings.
        self.qwerty_events.iter_mut().for_each(|q| q.1.update(&pressed, &down, state.alphanumeric_input));
        // Get the key presses.
        self.event_starts = self.qwerty_events.iter().filter(|q| q.1.pressed).map(|q| *q.0).collect();
        // Get the key downs.
        self.event_continues = self.qwerty_events.iter().filter(|q| q.1.down).map(|q| *q.0).collect();

        // MIDI INPUT.
        if let Some(midi_conn) = &mut self.midi_conn {
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
                    if state.armed {
                        // Remember the note-on for piano roll input.
                        self.note_on_events.push(NoteOn::new(midi));
                    }
                    // Copy this note to the immediate note-on array.
                    self.note_ons.push(*midi);
                }
                // Note-off.
                if state.armed && midi[0] >= 128 && midi[0] <= 143 {
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
                    self.new_notes.push(note_on.note);
                }
                self.note_on_events.clear();
            }
        }
    }

    /// Reads the qwerty and MIDI bindings for an event.
    pub fn get_bindings(&self, event: &InputEvent) -> (Option<QwertyBinding>, Option<MidiBinding>) {
        (self.qwerty_events.get(event).cloned(), self.midi_events.get(event).cloned())
    }
}
