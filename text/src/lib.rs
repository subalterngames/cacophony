//! This crate handles three related, but separate, tasks:
//!
//! 1. `Text` stores localized text. Throughout Cacophony, all strings that will be spoken or drawn are referenced via lookup keys. The text data is in `data/text.csv`.
//! 2. `TTS` converts text-to-speech strings into spoken audio.
//! 3. This crate also contains language-agnostic string manipulation functions e.g. `truncate`.

mod tooltips;
mod tts;
mod value_map;
pub use self::tts::{Enqueable, TTS};
use std::path::Path;
pub use value_map::ValueMap;
mod tts_string;
use common::config::parse;
use common::{EditMode, EffectType, Event, Paths, PianoRollMode, Time, MIN_NOTE, PPQ_F, PPQ_U};
use csv::Reader;
use hashbrown::HashMap;
use ini::Ini;
use input::KEYS;
use macroquad::input::KeyCode;
pub use tooltips::Tooltips;
pub use tts_string::TtsString;

/// All possible languages.
const LANGUAGES: [&str; 1] = ["en"];
/// Keycode lookup string prefixes.
const KEYCODE_LOOKUPS: [&str; 121] = [
    "Space",
    "Apostrophe",
    "Comma",
    "Minus",
    "Period",
    "Slash",
    "Key0",
    "Key1",
    "Key2",
    "Key3",
    "Key4",
    "Key5",
    "Key6",
    "Key7",
    "Key8",
    "Key9",
    "Semicolon",
    "Equal",
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "G",
    "H",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "O",
    "P",
    "Q",
    "R",
    "S",
    "T",
    "U",
    "V",
    "W",
    "X",
    "Y",
    "Z",
    "LeftBracket",
    "Backslash",
    "RightBracket",
    "GraveAccent",
    "World1",
    "World2",
    "Escape",
    "Enter",
    "Tab",
    "Backspace",
    "Insert",
    "Delete",
    "Right",
    "Left",
    "Down",
    "Up",
    "PageUp",
    "PageDown",
    "Home",
    "End",
    "CapsLock",
    "ScrollLock",
    "NumLock",
    "PrintScreen",
    "Pause",
    "F1",
    "F2",
    "F3",
    "F4",
    "F5",
    "F6",
    "F7",
    "F8",
    "F9",
    "F10",
    "F11",
    "F12",
    "F13",
    "F14",
    "F15",
    "F16",
    "F17",
    "F18",
    "F19",
    "F20",
    "F21",
    "F22",
    "F23",
    "F24",
    "F25",
    "Kp0",
    "Kp1",
    "Kp2",
    "Kp3",
    "Kp4",
    "Kp5",
    "Kp6",
    "Kp7",
    "Kp8",
    "Kp9",
    "KpDecimal",
    "KpDivide",
    "KpMultiply",
    "KpSubtract",
    "KpAdd",
    "KpEnter",
    "KpEqual",
    "LeftShift",
    "LeftControl",
    "LeftAlt",
    "LeftSuper",
    "RightShift",
    "RightControl",
    "RightAlt",
    "RightSuper",
    "Menu",
    "Unknown",
];

type TextMap = HashMap<String, String>;

/// Localized text lookup.
pub struct Text {
    /// The text key-value map.
    text: TextMap,
    /// A map of key codes to spoken text.
    keycodes_spoken: HashMap<KeyCode, String>,
    /// A map of key codes to seen text.
    keycodes_seen: HashMap<KeyCode, String>,
    /// The text for each edit mode.
    edit_modes: HashMap<EditMode, String>,
    /// The text for each piano roll mode.
    piano_roll_modes: HashMap<PianoRollMode, String>,
    /// The name of each MIDI note.
    note_names: Vec<String>,
    /// Boolean display strings.
    booleans: ValueMap<bool>,
    effect_types: ValueMap<EffectType>,
}

impl Text {
    pub fn new(config: &Ini, paths: &Paths) -> Self {
        // Get the text language.
        let language: String = parse(config.section(Some("TEXT")).unwrap(), "language");
        // Find the column with the language.
        let column = LANGUAGES.iter().position(|&lang| lang == language).unwrap() + 1;
        // Get the text.
        let mut text = HashMap::new();
        // Read the .csv file.
        let mut reader = Reader::from_path(&paths.text_path).unwrap();
        for record in reader.records().filter(|r| r.is_ok()).flatten() {
            let key = record.get(0).unwrap().to_string();
            let value = record.get(column).unwrap().to_string();
            text.insert(key, value);
        }
        let note_names: Vec<String> = text
            .remove("NOTE_NAMES")
            .unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        let keycodes_spoken = Text::get_keycode_map(&text, true);
        let keycodes_seen = Text::get_keycode_map(&text, false);
        let edit_modes = Text::get_edit_mode_map(&text);
        let piano_roll_modes = Text::get_piano_roll_mode_map(&text);
        let booleans = ValueMap::new_from_strings(
            [true, false],
            [text["TRUE"].clone(), text["FALSE"].clone()],
        );
        let effect_types = ValueMap::new_from_strings(
            EffectType::get_array(),
            [
                text["EFFECT_TYPE_CHORUS"].clone(),
                text["EFFECT_TYPE_PAN"].clone(),
                text["EFFECT_TYPE_REVERB"].clone(),
                text["EFFECT_TYPE_PITCH_BEND"].clone(),
                text["EFFECT_TYPE_CHANNEL_PRESSURE"].clone(),
                text["EFFECT_TYPE_POLYPHONIC_KEY_PRESSURE"].clone(),
            ],
        );
        Self {
            text,
            keycodes_spoken,
            keycodes_seen,
            edit_modes,
            piano_roll_modes,
            note_names,
            booleans,
            effect_types,
        }
    }

    /// Returns the text.
    pub fn get(&self, key: &str) -> String {
        match self.text.get(key) {
            Some(t) => t.clone(),
            None => panic!("Invalid text key {}", key),
        }
    }

    /// Returns the text.
    pub fn get_ref(&self, key: &str) -> &str {
        match self.text.get(key) {
            Some(t) => t,
            None => panic!("Invalid text key {}", key),
        }
    }

    /// Returns the text. Fills in the values.
    pub fn get_with_values(&self, key: &str, values: &[&str]) -> String {
        match self.text.get(key) {
            Some(t) => {
                let mut text = t.clone();
                for (i, v) in values.iter().enumerate() {
                    let mut k: String = String::from("\\");
                    k.push_str(i.to_string().as_str());
                    let vv = v.to_string();
                    text = text.replace(&k, vv.as_str());
                }
                if text.contains('\\') {
                    println!("WARNING! Bad TTS text. {} {} {:?}", text, key, values);
                }
                text.replace("  ", " ")
            }
            None => panic!("Invalid text key {}", key),
        }
    }

    /// Returns the string version of a key code.
    pub fn get_keycode(&self, key: &KeyCode, spoken: bool) -> &str {
        match (if spoken {
            &self.keycodes_spoken
        } else {
            &self.keycodes_seen
        })
        .get(key)
        {
            Some(t) => t,
            None => panic!("Invalid key code {:?}", key),
        }
    }

    /// Returns the string version of a piano roll mode.
    pub fn get_piano_roll_mode(&self, mode: &PianoRollMode) -> &str {
        match self.piano_roll_modes.get(mode) {
            Some(t) => t,
            None => panic!("Invalid piano roll mode {:?}", mode),
        }
    }

    /// Returns the string version of an edit mode.
    pub fn get_edit_mode(&self, mode: &EditMode) -> &str {
        match self.edit_modes.get(mode) {
            Some(t) => t,
            None => panic!("Invalid edit mode {:?}", mode),
        }
    }

    /// Returns boolean text.
    pub fn get_boolean(&self, value: &bool) -> &str {
        self.booleans.get(value)
    }

    /// Returns the maximum character width of the boolean values.
    pub fn get_max_boolean_length(&self) -> u32 {
        self.booleans.max_length
    }

    /// Converts a beat PPQ value into a time string.
    pub fn get_time(&self, ppq: u64, time: &Time) -> String {
        let duration = time.ppq_to_duration(ppq);
        let whole_seconds = duration.as_secs();
        let hours = whole_seconds / 3600;
        let minutes = whole_seconds / 60 - (hours * 60);
        let seconds = whole_seconds - (minutes * 60);
        // Include hours?
        if hours > 0 {
            self.get_with_values(
                "TIME_TTS_HOURS",
                &[
                    hours.to_string().as_str(),
                    minutes.to_string().as_str(),
                    seconds.to_string().as_str(),
                ],
            )
        } else {
            self.get_with_values(
                "TIME_TTS",
                &[minutes.to_string().as_str(), seconds.to_string().as_str()],
            )
        }
    }

    /// Returns a text-to-speech string of the `ppq` value.
    pub fn get_ppq_tts(&self, ppq: &u64) -> String {
        // This is a whole note.
        if ppq % PPQ_U == 0 {
            (ppq / PPQ_U).to_string()
        } else {
            match ppq {
                288 => self.get("FRACTION_TTS_ONE_AND_A_HALF"),
                96 => self.get("FRACTION_TTS_ONE_HALF"),
                64 => self.get("FRACTION_TTS_ONE_THIRD"),
                48 => self.get("FRACTION_TTS_ONE_FOURTH"),
                32 => self.get("FRACTION_TTS_ONE_SIXTH"),
                24 => self.get("FRACTION_TTS_ONE_EIGHTH"),
                12 => self.get("FRACTION_TTS_ONE_SIXTEENTH"),
                6 => self.get("FRACTION_TTS_ONE_THIRTY_SECOND"),
                other => format!("{:.2}", (*other as f32 / PPQ_F)),
            }
        }
    }

    /// Returns an error text-to-speech string.
    pub fn get_error(&self, error: &str) -> String {
        self.get_with_values("ERROR", &[error])
    }

    /// Returns the name of a note or event.
    pub fn get_event_name(&self, event: &Event<'_>) -> String {
        match event {
            Event::Effect { effect, index: _ } => {
                self.get_effect_type_name(&effect.effect).to_string()
            }
            Event::Note { note, index: _ } => note.get_name().to_string(),
        }
    }

    /// Returns the name of an effect type.
    pub fn get_effect_type_name<'a>(&'a self, effect: &EffectType) -> &'a str {
        self.effect_types.get(effect)
    }

    pub fn get_max_effect_type_length(&self) -> u32 {
        self.effect_types.max_length
    }

    /// Returns the name of the note.
    pub fn get_note_name(&self, note: u8) -> &str {
        &self.note_names[(note - MIN_NOTE) as usize]
    }

    /// Returns a map of keycodes to displayable/sayable text (NOT string keys).
    fn get_keycode_map(text: &HashMap<String, String>, spoken: bool) -> HashMap<KeyCode, String> {
        let suffix = if spoken { "_SPOKEN" } else { "_SEEN" };
        let mut keycodes = HashMap::new();
        for (key, lookup) in KEYS.iter().zip(KEYCODE_LOOKUPS) {
            let mut lookup_key = lookup.to_string();
            lookup_key.push_str(suffix);
            keycodes.insert(*key, text[&lookup_key].clone());
        }
        keycodes
    }

    /// Returns a HashMap of the edit modes.
    fn get_edit_mode_map(text: &HashMap<String, String>) -> HashMap<EditMode, String> {
        let mut edit_modes = HashMap::new();
        edit_modes.insert(EditMode::Normal, text["EDIT_MODE_NORMAL"].clone());
        edit_modes.insert(EditMode::Quick, text["EDIT_MODE_QUICK"].clone());
        edit_modes.insert(EditMode::Precise, text["EDIT_MODE_PRECISE"].clone());
        edit_modes
    }

    /// Returns a HashMap of the piano roll modes.
    fn get_piano_roll_mode_map(text: &HashMap<String, String>) -> HashMap<PianoRollMode, String> {
        let mut piano_roll_modes = HashMap::new();
        piano_roll_modes.insert(PianoRollMode::Edit, text["PIANO_ROLL_MODE_EDIT"].clone());
        piano_roll_modes.insert(
            PianoRollMode::Select,
            text["PIANO_ROLL_MODE_SELECT"].clone(),
        );
        piano_roll_modes.insert(PianoRollMode::Time, text["PIANO_ROLL_MODE_TIME"].clone());
        piano_roll_modes.insert(PianoRollMode::View, text["PIANO_ROLL_MODE_VIEW"].clone());
        piano_roll_modes
    }
}

/// Converts a PPQ value into a string beat value.
pub fn ppq_to_string(ppq: u64) -> String {
    // This is a whole note.
    if ppq % PPQ_U == 0 {
        (ppq / PPQ_U).to_string()
    } else {
        match ppq {
            288 => "3/2".to_string(),
            96 => "1/2".to_string(),
            64 => "1/3".to_string(),
            48 => "1/4".to_string(),
            32 => "1/6".to_string(),
            24 => "1/8".to_string(),
            12 => "1/16".to_string(),
            6 => "1/32".to_string(),
            other => format!("{:.2}", (other as f32 / PPQ_F)),
        }
    }
}

/// Truncate a string to fit a specified length.
///
/// - `string` The string.
/// - `length` The maximum length of the string.
/// - `left` If true, remove characters from the left. Example: `"ABCDEFG" -> `"DEFG"`. If false, remove characters from the right. Example: `"ABCDEFG" -> `"ABCD"`.
pub fn truncate(string: &str, length: usize, left: bool) -> &str {
    let len = string.chars().count();
    if len <= length {
        string
    }
    // Remove characters on the left.
    else if left {
        &string[len - length..len]
    }
    // Remove characters on the right.
    else {
        &string[0..length]
    }
}

/// Returns the file name of a path.
pub fn get_file_name(path: &Path) -> &str {
    match path.file_name() {
        Some(filename) => match filename.to_str() {
            Some(s) => s,
            None => panic!("Invalid filename: {:?}", filename),
        },
        None => panic!("Not a file: {:?}", path),
    }
}

/// Returns the file name of a path without the extension.
pub fn get_file_name_no_ex(path: &Path) -> &str {
    match path.file_stem() {
        Some(filename) => match filename.to_str() {
            Some(s) => s,
            None => panic!("Invalid filename: {:?}", filename),
        },
        None => panic!("Not a file: {:?}", path),
    }
}
