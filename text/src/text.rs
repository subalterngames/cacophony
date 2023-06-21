use common::config::parse;
use common::csv::Reader;
use common::hashbrown::HashMap;
use common::ini::Ini;
use common::macroquad::input::KeyCode;
use common::{EditMode, Paths, PianoRollMode, Time, MIN_NOTE, PPQ_F, PPQ_U};

const LANGUAGES: [&str; 1] = ["en"];

/// Localized text lookup.
pub struct Text {
    /// The text key-value map.
    text: HashMap<String, String>,
    /// A map of key codes to displayable/sayable text.
    keycodes: HashMap<KeyCode, String>,
    /// The text for each edit mode.
    edit_modes: HashMap<EditMode, String>,
    /// The text for each piano roll mode.
    piano_roll_modes: HashMap<PianoRollMode, String>,
    /// The name of each MIDI note.
    note_names: Vec<String>,
    /// Boolean dislay
    booleans: HashMap<bool, String>,
}

impl Text {
    pub fn new(config: &Ini, paths: &Paths) -> Self {
        // Get the text language.
        let language: String = parse(config.section(Some("TEXT")).unwrap(), "language");
        // Find the column with the language.
        let column = LANGUAGES.iter().position(|&lang| lang == language).unwrap() + 1;
        // Get the text.
        let mut text: HashMap<String, String> = HashMap::new();
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
        let keycodes = Text::get_keycode_map(&text);
        let edit_modes = Text::get_edit_mode_map(&text);
        let piano_roll_modes = Text::get_piano_roll_mode_map(&text);
        let mut booleans = HashMap::new();
        booleans.insert(true, text["TRUE"].clone());
        booleans.insert(false, text["FALSE"].clone());
        Self {
            text,
            keycodes,
            edit_modes,
            piano_roll_modes,
            note_names,
            booleans,
        }
    }

    /// Returns the text.
    pub fn get(&self, key: &str) -> String {
        match self.text.get(&key.to_string()) {
            Some(t) => t.clone(),
            None => panic!("Invalid text key {}", key),
        }
    }

    /// Returns the text. Fills in the values.
    pub fn get_with_values(&self, key: &str, values: &[&str]) -> String {
        match self.text.get(&String::from(key)) {
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
    pub fn get_keycode(&self, key: &KeyCode) -> String {
        match self.keycodes.get(key) {
            Some(t) => t.to_uppercase(),
            None => panic!("Invalid key code {:?}", key),
        }
    }

    /// Returns the string version of a piano roll mode.
    pub fn get_piano_roll_mode(&self, mode: &PianoRollMode) -> String {
        match self.piano_roll_modes.get(mode) {
            Some(t) => t.clone(),
            None => panic!("Invalid piano roll mode {:?}", mode),
        }
    }

    /// Returns the string version of an edit mode.
    pub fn get_edit_mode(&self, mode: &EditMode) -> String {
        match self.edit_modes.get(mode) {
            Some(t) => t.clone(),
            None => panic!("Invalid edit mode {:?}", mode),
        }
    }

    /// Returns boolean text.
    pub fn get_boolean(&self, value: bool) -> String {
        self.booleans[&value].clone()
    }

    pub fn get_max_boolean_length(&self) -> usize {
        self.booleans
            .iter()
            .map(|kv| kv.1.chars().count())
            .max()
            .unwrap()
    }

    /// Converts a beat PPQ value into a time string.
    pub fn get_time(&self, ppq: u64, time: &Time) -> String {
        let duration = time.ppq_to_duration(ppq);
        let hours = duration.whole_hours();
        let minutes = duration.whole_minutes() - (hours * 60);
        let seconds = duration.whole_seconds() - (minutes * 60);
        // Include hours?
        match duration.whole_hours() > 0 {
            true => self.get_with_values(
                "TIME_TTS_HOURS",
                &[
                    hours.to_string().as_str(),
                    minutes.to_string().as_str(),
                    seconds.to_string().as_str(),
                ],
            ),
            false => self.get_with_values(
                "TIME_TTS",
                &[minutes.to_string().as_str(), seconds.to_string().as_str()],
            ),
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

    /// Returns the name of the note.
    pub fn get_note_name(&self, note: u8) -> String {
        self.note_names[(note - MIN_NOTE) as usize].clone()
    }

    /// Returns a map of keycodes to displayable/sayable text (NOT string keys).
    fn get_keycode_map(text: &HashMap<String, String>) -> HashMap<KeyCode, String> {
        let mut keycodes = HashMap::new();
        keycodes.insert(KeyCode::Space, text["Space"].clone());
        keycodes.insert(KeyCode::Apostrophe, text["Apostrophe"].clone());
        keycodes.insert(KeyCode::Comma, text["Comma"].clone());
        keycodes.insert(KeyCode::Minus, text["Minus"].clone());
        keycodes.insert(KeyCode::Period, text["Period"].clone());
        keycodes.insert(KeyCode::Slash, text["Slash"].clone());
        keycodes.insert(KeyCode::Key0, text["Key0"].clone());
        keycodes.insert(KeyCode::Key1, text["Key1"].clone());
        keycodes.insert(KeyCode::Key2, text["Key2"].clone());
        keycodes.insert(KeyCode::Key3, text["Key3"].clone());
        keycodes.insert(KeyCode::Key4, text["Key4"].clone());
        keycodes.insert(KeyCode::Key5, text["Key5"].clone());
        keycodes.insert(KeyCode::Key6, text["Key6"].clone());
        keycodes.insert(KeyCode::Key7, text["Key7"].clone());
        keycodes.insert(KeyCode::Key8, text["Key8"].clone());
        keycodes.insert(KeyCode::Key9, text["Key9"].clone());
        keycodes.insert(KeyCode::Semicolon, text["Semicolon"].clone());
        keycodes.insert(KeyCode::Equal, text["Equal"].clone());
        keycodes.insert(KeyCode::A, text["A"].clone());
        keycodes.insert(KeyCode::B, text["B"].clone());
        keycodes.insert(KeyCode::C, text["C"].clone());
        keycodes.insert(KeyCode::D, text["D"].clone());
        keycodes.insert(KeyCode::E, text["E"].clone());
        keycodes.insert(KeyCode::F, text["F"].clone());
        keycodes.insert(KeyCode::G, text["G"].clone());
        keycodes.insert(KeyCode::H, text["H"].clone());
        keycodes.insert(KeyCode::I, text["I"].clone());
        keycodes.insert(KeyCode::J, text["J"].clone());
        keycodes.insert(KeyCode::K, text["K"].clone());
        keycodes.insert(KeyCode::L, text["L"].clone());
        keycodes.insert(KeyCode::M, text["M"].clone());
        keycodes.insert(KeyCode::N, text["N"].clone());
        keycodes.insert(KeyCode::O, text["O"].clone());
        keycodes.insert(KeyCode::P, text["P"].clone());
        keycodes.insert(KeyCode::Q, text["Q"].clone());
        keycodes.insert(KeyCode::R, text["R"].clone());
        keycodes.insert(KeyCode::S, text["S"].clone());
        keycodes.insert(KeyCode::T, text["T"].clone());
        keycodes.insert(KeyCode::U, text["U"].clone());
        keycodes.insert(KeyCode::V, text["V"].clone());
        keycodes.insert(KeyCode::W, text["W"].clone());
        keycodes.insert(KeyCode::X, text["X"].clone());
        keycodes.insert(KeyCode::Y, text["Y"].clone());
        keycodes.insert(KeyCode::Z, text["Z"].clone());
        keycodes.insert(KeyCode::LeftBracket, text["LeftBracket"].clone());
        keycodes.insert(KeyCode::Backslash, text["Backslash"].clone());
        keycodes.insert(KeyCode::RightBracket, text["RightBracket"].clone());
        keycodes.insert(KeyCode::GraveAccent, text["GraveAccent"].clone());
        keycodes.insert(KeyCode::World1, text["World1"].clone());
        keycodes.insert(KeyCode::World2, text["World2"].clone());
        keycodes.insert(KeyCode::Escape, text["Escape"].clone());
        keycodes.insert(KeyCode::Enter, text["Enter"].clone());
        keycodes.insert(KeyCode::Tab, text["Tab"].clone());
        keycodes.insert(KeyCode::Backspace, text["Backspace"].clone());
        keycodes.insert(KeyCode::Insert, text["Insert"].clone());
        keycodes.insert(KeyCode::Delete, text["Delete"].clone());
        keycodes.insert(KeyCode::Right, text["Right"].clone());
        keycodes.insert(KeyCode::Left, text["Left"].clone());
        keycodes.insert(KeyCode::Down, text["Down"].clone());
        keycodes.insert(KeyCode::Up, text["Up"].clone());
        keycodes.insert(KeyCode::PageUp, text["PageUp"].clone());
        keycodes.insert(KeyCode::PageDown, text["PageDown"].clone());
        keycodes.insert(KeyCode::Home, text["Home"].clone());
        keycodes.insert(KeyCode::End, text["End"].clone());
        keycodes.insert(KeyCode::CapsLock, text["CapsLock"].clone());
        keycodes.insert(KeyCode::ScrollLock, text["ScrollLock"].clone());
        keycodes.insert(KeyCode::NumLock, text["NumLock"].clone());
        keycodes.insert(KeyCode::PrintScreen, text["PrintScreen"].clone());
        keycodes.insert(KeyCode::Pause, text["Pause"].clone());
        keycodes.insert(KeyCode::F1, text["F1"].clone());
        keycodes.insert(KeyCode::F2, text["F2"].clone());
        keycodes.insert(KeyCode::F3, text["F3"].clone());
        keycodes.insert(KeyCode::F4, text["F4"].clone());
        keycodes.insert(KeyCode::F5, text["F5"].clone());
        keycodes.insert(KeyCode::F6, text["F6"].clone());
        keycodes.insert(KeyCode::F7, text["F7"].clone());
        keycodes.insert(KeyCode::F8, text["F8"].clone());
        keycodes.insert(KeyCode::F9, text["F9"].clone());
        keycodes.insert(KeyCode::F10, text["F10"].clone());
        keycodes.insert(KeyCode::F11, text["F11"].clone());
        keycodes.insert(KeyCode::F12, text["F12"].clone());
        keycodes.insert(KeyCode::F13, text["F13"].clone());
        keycodes.insert(KeyCode::F14, text["F14"].clone());
        keycodes.insert(KeyCode::F15, text["F15"].clone());
        keycodes.insert(KeyCode::F16, text["F16"].clone());
        keycodes.insert(KeyCode::F17, text["F17"].clone());
        keycodes.insert(KeyCode::F18, text["F18"].clone());
        keycodes.insert(KeyCode::F19, text["F19"].clone());
        keycodes.insert(KeyCode::F20, text["F20"].clone());
        keycodes.insert(KeyCode::F21, text["F21"].clone());
        keycodes.insert(KeyCode::F22, text["F22"].clone());
        keycodes.insert(KeyCode::F23, text["F23"].clone());
        keycodes.insert(KeyCode::F24, text["F24"].clone());
        keycodes.insert(KeyCode::F25, text["F25"].clone());
        keycodes.insert(KeyCode::Kp0, text["Kp0"].clone());
        keycodes.insert(KeyCode::Kp1, text["Kp1"].clone());
        keycodes.insert(KeyCode::Kp2, text["Kp2"].clone());
        keycodes.insert(KeyCode::Kp3, text["Kp3"].clone());
        keycodes.insert(KeyCode::Kp4, text["Kp4"].clone());
        keycodes.insert(KeyCode::Kp5, text["Kp5"].clone());
        keycodes.insert(KeyCode::Kp6, text["Kp6"].clone());
        keycodes.insert(KeyCode::Kp7, text["Kp7"].clone());
        keycodes.insert(KeyCode::Kp8, text["Kp8"].clone());
        keycodes.insert(KeyCode::Kp9, text["Kp9"].clone());
        keycodes.insert(KeyCode::KpDecimal, text["KpDecimal"].clone());
        keycodes.insert(KeyCode::KpDivide, text["KpDivide"].clone());
        keycodes.insert(KeyCode::KpMultiply, text["KpMultiply"].clone());
        keycodes.insert(KeyCode::KpSubtract, text["KpSubtract"].clone());
        keycodes.insert(KeyCode::KpAdd, text["KpAdd"].clone());
        keycodes.insert(KeyCode::KpEnter, text["KpEnter"].clone());
        keycodes.insert(KeyCode::KpEqual, text["KpEqual"].clone());
        keycodes.insert(KeyCode::LeftShift, text["LeftShift"].clone());
        keycodes.insert(KeyCode::LeftControl, text["LeftControl"].clone());
        keycodes.insert(KeyCode::LeftAlt, text["LeftAlt"].clone());
        keycodes.insert(KeyCode::LeftSuper, text["LeftSuper"].clone());
        keycodes.insert(KeyCode::RightShift, text["RightShift"].clone());
        keycodes.insert(KeyCode::RightControl, text["RightControl"].clone());
        keycodes.insert(KeyCode::RightAlt, text["RightAlt"].clone());
        keycodes.insert(KeyCode::RightSuper, text["RightSuper"].clone());
        keycodes.insert(KeyCode::Menu, text["Menu"].clone());
        keycodes.insert(KeyCode::Unknown, text["Unknown"].clone());
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
