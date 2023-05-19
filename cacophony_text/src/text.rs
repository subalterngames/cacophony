use cacophony_core::config::parse;
use cacophony_core::Paths;
use csv::Reader;
use hashbrown::HashMap;
use ini::Ini;

const LANGUAGES: [&str; 1] = ["en"];

pub struct Text {
    /// The text key-value map.
    text: HashMap<String, String>,
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
        Self { text }
    }

    /// Returns the text.
    pub fn get(&self, key: &str) -> String {
        match self.text.get(&String::from(key)) {
            Some(t) => t.clone(),
            None => panic!("Invalid text key {}", key),
        }
    }
}
