//! This crate handles all user input.
//!
//! - `InputEvent` is an enum defining an event triggered by user input, e.g. a decrease in track volume.
//! - `Input` maps raw qwerty keycode and raw MIDI messages (control bindings) to input events. It updates per frame, reading input and storing new events.

mod debug_input_event;

mod input_event;
mod keys;
mod midi_binding;
mod midi_conn;
mod note_on;
mod qwerty_binding;

use common::args::Args;
use common::{State, MAX_NOTE, MIN_NOTE};
use debug_input_event::DebugInputEvent;
use hashbrown::HashMap;
use ini::Ini;
pub use input_event::InputEvent;
pub use keys::KEYS;
use keys::{ALPHANUMERIC_INPUT_MODS, MODS};
use macroquad::input::*;
use midi_binding::MidiBinding;
use midi_conn::MidiConn;
use note_on::NoteOn;
pub use qwerty_binding::QwertyBinding;
use serde_json::from_str;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

const MAX_OCTAVE: u8 = 9;
/// Only these events are allowed during alphanumeric input.
const ALLOWED_DURING_ALPHANUMERIC_INPUT: [InputEvent; 12] = [
    InputEvent::Quit,
    InputEvent::AppTTS,
    InputEvent::StatusTTS,
    InputEvent::InputTTS,
    InputEvent::FileTTS,
    InputEvent::ToggleAlphanumericInput,
    InputEvent::UpDirectory,
    InputEvent::DownDirectory,
    InputEvent::SelectFile,
    InputEvent::NextPath,
    InputEvent::PreviousPath,
    InputEvent::CloseOpenFile,
];
/// Note-on events generated by a qwerty keyboard, and the index of each key on a C scale.
const QWERTY_NOTE_EVENTS: [(InputEvent, u8); 12] = [
    (InputEvent::G, 7),
    (InputEvent::FSharp, 6),
    (InputEvent::F, 5),
    (InputEvent::E, 4),
    (InputEvent::DSharp, 3),
    (InputEvent::D, 2),
    (InputEvent::CSharp, 1),
    (InputEvent::C, 0),
    (InputEvent::B, 11),
    (InputEvent::ASharp, 10),
    (InputEvent::A, 9),
    (InputEvent::GSharp, 8),
];
/// Don't allow these when typing a filename.
const ILLEGAL_FILENAME_CHARACTERS: [char; 23] = [
    '!', '@', '#', '$', '%', '^', '&', '*', '=', '+', '{', '}', '\\', '|', ':', '"', '\'', '<',
    '>', '/', '\n', '\r', '\t',
];

/// Listens for user input from qwerty and MIDI devices and records the current input state.
#[derive(Default)]
pub struct Input {
    /// Events that began on this frame (usually due to a key press or MIDI controller message).
    events: Vec<InputEvent>,
    /// The MIDI connection.
    midi_conn: Option<MidiConn>,
    /// Note-on MIDI messages. These will be sent immediately to the synthesizer to be played.
    pub note_on_messages: Vec<[u8; 3]>,
    /// Note-off MIDI messages. These will be sent immediately to the synthesizer.
    pub note_off_keys: Vec<u8>,
    /// Note-on events that don't have corresponding off events.
    note_on_events: Vec<NoteOn>,
    /// Notes that were added after all note-off events are done.
    pub new_notes: Vec<[u8; 3]>,
    /// Input events generated by MIDI input.
    midi_events: HashMap<InputEvent, MidiBinding>,
    /// Input events generated by qwerty input.
    qwerty_events: HashMap<InputEvent, QwertyBinding>,
    /// The octave for qwerty input.
    qwerty_octave: u8,
    /// Was backspace pressed on this frame?
    backspace: bool,
    /// Characters pressed on this frame.
    pub pressed_chars: Vec<char>,
    /// Debug input events.
    debug_inputs: Vec<DebugInputEvent>,
    /// The MIDI time counter.
    time_counter: i16,
}

impl Input {
    pub fn new(config: &Ini, args: &Args) -> Self {
        // Get the audio connections.
        let midi_conn = MidiConn::new();

        // Get qwerty events.
        let mut qwerty_events: HashMap<InputEvent, QwertyBinding> = HashMap::new();
        // Get the qwerty input mapping.
        let keyboard_input = config.section(Some("QWERTY_BINDINGS")).unwrap();
        for kv in keyboard_input.iter() {
            let k_input = Input::parse_qwerty_binding(kv.0, kv.1);
            qwerty_events.insert(k_input.0, k_input.1);
        }

        // Get MIDI events.
        let mut midi_events: HashMap<InputEvent, MidiBinding> = HashMap::new();
        // Get the qwerty input mapping.
        let midi_input = config.section(Some("MIDI_BINDINGS")).unwrap();
        for kv in midi_input.iter() {
            let k_input = Input::parse_midi_binding(kv.0, kv.1);
            midi_events.insert(k_input.0, k_input.1);
        }

        let mut debug_inputs = vec![];
        if let Some(events) = &args.events {
            match File::open(events) {
                Ok(mut file) => {
                    let mut s = String::new();
                    file.read_to_string(&mut s).unwrap();
                    let lines = s.split('\n');
                    for line in lines {
                        match line.trim().parse::<InputEvent>() {
                            Ok(e) => debug_inputs.push(DebugInputEvent::InputEvent(e)),
                            Err(_) => line
                                .trim()
                                .chars()
                                .for_each(|c| debug_inputs.push(DebugInputEvent::Alphanumeric(c))),
                        }
                    }
                }
                Err(error) => panic!("Failed to open file {:?}: {}", &events, error),
            }
        }

        Self {
            midi_conn,
            qwerty_events,
            midi_events,
            qwerty_octave: 4,
            debug_inputs,
            ..Default::default()
        }
    }

    /// Update the input state:
    ///
    /// 1. Clear all note and event frame data.
    /// 2. Check for pressed characters and add them to `self.pressed_characters.
    /// 3. Check all pressed keys and all qwerty bindings and register new events accordingly.
    /// 4. Remove some events during alphanumeric input.
    /// 5. Poll the MIDI connection, if any.
    ///
    /// If a MIDI connection polled:
    ///
    /// 1. Compare all polled MIDI events to MIDI bindings and register new events accordingly.
    /// 2. Add note messages to the list for playing notes.
    /// 3. Store new note-on events.
    /// 4. If all note-ons have had a corresponding note-off, add them to the new notes lists.
    pub fn update(&mut self, state: &State) {
        // Clear the old new notes.
        self.new_notes.clear();
        self.note_on_messages.clear();
        self.note_off_keys.clear();

        // QWERTY INPUT.

        // Was backspace pressed?
        self.backspace = is_key_pressed(KeyCode::Backspace);
        // Get the pressed characters.
        self.pressed_chars.clear();
        while let Some(c) = get_char_pressed() {
            self.pressed_chars.push(c);
        }
        // Get all pressed keys.
        let pressed = get_keys_pressed();
        // Get all held keys.
        let down = get_keys_down();

        // Update the qwerty key bindings.
        self.qwerty_events
            .iter_mut()
            .for_each(|q| q.1.update(&pressed, &down, state.input.alphanumeric_input));
        // Get the key presses.
        let mut events: Vec<InputEvent> = self
            .qwerty_events
            .iter()
            .filter(|q| q.1.pressed)
            .map(|q| *q.0)
            .collect();

        // DEBUG.
        if cfg!(debug_assertions) && !&self.debug_inputs.is_empty() {
            match self.debug_inputs.remove(0) {
                // Push an event.
                DebugInputEvent::InputEvent(e) => events.push(e),
                // Push a char.
                DebugInputEvent::Alphanumeric(c) => self.pressed_chars.push(c),
            }
        }

        // Qwerty note input.
        for (_, note_index) in QWERTY_NOTE_EVENTS
            .iter()
            .filter(|(e, _)| events.contains(e))
        {
            self.qwerty_note(*note_index, state);
        }
        // Octave up.
        if events.contains(&InputEvent::OctaveUp) && self.qwerty_octave < MAX_OCTAVE {
            self.clear_notes_on_qwerty_octave();
            self.qwerty_octave += 1;
        }
        // Octave down.
        if events.contains(&InputEvent::OctaveDown) && self.qwerty_octave > 0 {
            self.clear_notes_on_qwerty_octave();
            self.qwerty_octave -= 1;
        }
        // Qwerty note-off.
        for (_, qwerty_note_off) in QWERTY_NOTE_EVENTS.iter().filter(|(e, _)| {
            self.qwerty_events[e]
                .keys
                .iter()
                .all(|k| is_key_released(*k))
                && self.qwerty_events[e]
                    .mods
                    .iter()
                    .all(|k| is_key_released(*k))
        }) {
            self.note_off_keys.push(self.get_pitch(*qwerty_note_off));
        }

        #[cfg(debug_assertions)]
        self.listen_for_note_offs();

        // Remove events during alphanumeric input.
        if state.input.alphanumeric_input {
            events.retain(|e| ALLOWED_DURING_ALPHANUMERIC_INPUT.contains(e));
        }
        self.events = events;

        // MIDI INPUT.
        if let Some(midi_conn) = &mut self.midi_conn {
            // Poll for MIDI events.
            let mut midi = midi_conn.buffer.lock();
            // Append MIDI events.
            for mde in self.midi_events.iter_mut() {
                if mde.1.update(&midi, self.time_counter) {
                    self.events.push(*mde.0);
                }
            }
            // Increment the time counter.
            self.time_counter += 1;
            if self.time_counter >= 255 {
                self.time_counter = 0;
            }
            // Get note-on and note-off events.
            let volume = state.input.volume.get();
            for midi in midi.iter() {
                // Note-on.
                if midi[0] >= 144 && midi[0] <= 159 && midi[1] > MIN_NOTE && midi[2] <= MAX_NOTE {
                    // Set the volume.
                    let midi = if state.input.use_volume {
                        [midi[0], midi[1], volume]
                    } else {
                        *midi
                    };
                    // Remember the note-on for piano roll input.
                    if !state.input.is_playing {
                        if state.input.armed {
                            self.note_on_events.push(NoteOn::new(&midi));
                        }
                        // Copy this note to the immediate note-on array.
                        self.note_on_messages.push(midi);
                    }
                }
                // Note-off.
                if midi[0] >= 128 && midi[0] <= 143 {
                    self.note_off_keys.push(midi[1]);
                    if state.input.armed && !state.input.is_playing {
                        // Find the corresponding note.
                        for note_on in self.note_on_events.iter_mut() {
                            // Same key. Note-off.
                            if note_on.note[1] == midi[1] {
                                note_on.off = true;
                            }
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
            // Clear the MIDI buffer.
            midi.clear();
        }
    }

    /// Returns true if the event happened.
    pub fn happened(&self, event: &InputEvent) -> bool {
        self.events.contains(event)
    }

    /// Reads the qwerty and MIDI bindings for an event.
    pub fn get_bindings(
        &self,
        event: &InputEvent,
    ) -> (Option<&QwertyBinding>, Option<&MidiBinding>) {
        (self.qwerty_events.get(event), self.midi_events.get(event))
    }

    /// Modify a string with qwerty input from this frame. Allow alphanumeric input.
    pub fn modify_string_abc123(&self, string: &mut String) -> bool {
        self.modify_string(
            string,
            &self
                .pressed_chars
                .iter()
                .filter(|c| Self::is_valid_char(c))
                .copied()
                .collect::<Vec<char>>(),
        )
    }

    /// Modify a filename string with qwerty input from this frame. Allow alphanumeric input.
    pub fn modify_filename_abc123(&self, string: &mut String) -> bool {
        self.modify_string(
            string,
            &self
                .pressed_chars
                .iter()
                .filter(|c| Self::is_valid_char(c) && !ILLEGAL_FILENAME_CHARACTERS.contains(c))
                .copied()
                .collect::<Vec<char>>(),
        )
    }

    /// Returns true if the user can input this character.
    fn is_valid_char(c: &char) -> bool {
        c.is_alphanumeric() || c.is_ascii_punctuation() || c.is_whitespace()
    }

    /// Modify a u64 value.
    pub fn modify_u64(&self, value: &mut u64) -> bool {
        self.modify_value(value, 0)
    }

    /// Modify a value with qwerty input from this frame. Allow numeric input.
    fn modify_value<T>(&self, value: &mut T, default_value: T) -> bool
    where
        T: ToString + FromStr,
    {
        // Convert the value to a string.
        let mut string = value.to_string();
        // Modify the string.
        let modified = self.modify_string(
            &mut string,
            &self
                .pressed_chars
                .iter()
                .filter(|c| c.is_ascii_digit())
                .copied()
                .collect::<Vec<char>>(),
        );
        // Try to get a value.
        match T::from_str(string.as_str()) {
            Ok(v) => *value = v,
            Err(_) => *value = default_value,
        }
        modified
    }

    /// Modify a string with qwerty input from this frame.
    fn modify_string(&self, string: &mut String, chars: &[char]) -> bool {
        // Delete the last character.
        if self.backspace {
            string.pop().is_some()
        // Add new characters.
        } else if !chars.is_empty() {
            for ch in chars.iter() {
                string.push(*ch);
            }
            true
        } else {
            false
        }
    }

    /// Parse a qwerty binding from a key-value pair of strings (i.e. from a config file).
    fn parse_qwerty_binding(key: &str, value: &str) -> (InputEvent, QwertyBinding) {
        match key.parse::<InputEvent>() {
            Ok(input_key) => (input_key, QwertyBinding::deserialize(value)),
            Err(error) => panic!("Invalid input key {}: {}", key, error),
        }
    }

    // Parse a MIDI binding from a key-value pair of strings (i.e. from a config file).
    fn parse_midi_binding(key: &str, value: &str) -> (InputEvent, MidiBinding) {
        match key.parse::<InputEvent>() {
            Ok(input_key) => match from_str(value) {
                Ok(m) => (input_key, m),
                Err(error) => panic!(
                    "Failed to deserialize {} into a MidiBinding: {}",
                    value, error
                ),
            },
            Err(error) => panic!("Invalid input key {}: {}", key, error),
        }
    }

    /// Push a new note from qwerty input.
    fn qwerty_note(&mut self, note: u8, state: &State) {
        if !state.input.is_playing {
            let note: [u8; 3] = [144, self.get_pitch(note), state.input.volume.get()];
            if state.input.armed {
                self.new_notes.push(note);
            }
            self.note_on_messages.push(note);
        }
    }

    /// Converts the note index to a MIDI note value.
    fn get_pitch(&self, note: u8) -> u8 {
        (9 - self.qwerty_octave) * 12 + note
    }

    /// When a qwerty note is pressed, followed by an octave change, clear all note-on events.
    fn clear_notes_on_qwerty_octave(&mut self) {
        // Qwerty note-off.
        for (_, qwerty_note_off) in QWERTY_NOTE_EVENTS.iter() {
            self.note_off_keys.push(self.get_pitch(*qwerty_note_off));
        }
    }

    #[cfg(debug_assertions)]
    fn listen_for_note_offs(&mut self) {
        if self.happened(&InputEvent::NotesOff) {
            self.note_off_keys
                .append(&mut (MIN_NOTE..MAX_NOTE).collect());
        }
    }
}
