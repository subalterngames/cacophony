use crate::{Text, TtsString};
use input::{Input, InputEvent};
use hashbrown::HashMap;
use hashbrown::hash_map::Entry;
use regex::Regex;

const NUM_REGEXES: usize = 16;

type Regexes = [Regex; NUM_REGEXES];

pub struct Tooltips {
    /// The map of keys and tooltips.
    tooltips: HashMap<String, TtsString>,
    /// The regex used to find bindings.
    re_bindings: Regexes,
    /// The regex used to find wildcard values.
    re_values: Regexes,
}

impl Default for Tooltips {
    fn default() -> Self {
        let tooltips = HashMap::new();
        let re_bindings = Self::get_regexes("\\");
        let re_values = Self::get_regexes("%");
        Self { tooltips, re_bindings, re_values }
    }
}

impl Tooltips {
    /// Build a tooltip from a text lookup key and a list of events.
    ///
    /// - `key` The text lookup key, for example "TITLE_MAIN_MENU".
    /// - `events` An ordered list of input events. These will be inserted in the order that the binding wildcards are found.
    /// - `input` The input manager.
    ///
    /// Returns a tooltip `TtsString`.
    pub fn get_tooltip(&mut self, key: &str, events: &[InputEvent], text: &Text, input: &Input) -> &TtsString {
        match self.tooltips.entry(key.to_string()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let t = self.get_tooltip_with_values(key, events, &[], text, input);
                v.insert(t)
            }
        }
    }

    /// Build a tooltip from a text lookup key and a list of events and another list of values.
    ///
    /// - `key` The text lookup key, for example "TITLE_MAIN_MENU".
    /// - `events` An ordered list of input events. The index is used to find the wildcard in the text, e.g. if the index is 0 then the wildcard is "\0".
    /// - `values` An ordered list of string values. The index is used to find the wildcard in the text, e.g. if the index is 0 then the wildcard is "%0".
    /// - `input` The input manager.
    ///
    /// Returns a list of text-to-speech strings.
    pub fn get_tooltip_with_values(
        &self,
        key: &str,
        events: &[InputEvent],
        values: &[&str],
        text: &Text,
        input: &Input,
    ) -> TtsString {
        // Get the string with the wildcards.
        let raw_string = text.get(key);
        let mut spoken = raw_string.clone();
        let mut seen = raw_string;
        let mut regexes = HashMap::new();
        // Iterate through each event.
        for (i, event) in events.iter().enumerate() {
            let regex = &self.re_bindings[i];
            regexes.insert(i, regex.clone());
            // Get the key bindings.
            let bindings = input.get_bindings(event);
            // The replacement string.
            let mut spoken_replacement = vec![];
            let mut seen_replacement = vec![];
            let mut has_qwerty = false;
            // Get the qwerty binding.
            if let Some(qwerty) = bindings.0 {
                has_qwerty = true;
                // Add spoken mods.
                for m in text.get_mods(qwerty, true) {
                    spoken_replacement.push(m.to_string());
                }
                // Add seen mod tokens.
                for m in text.get_mods(qwerty, false) {
                    seen_replacement.push(m.to_string());
                }
                // Add spoken keys.
                for k in text.get_keys(qwerty, true) {
                    spoken_replacement.push(k.to_string());
                }
                // Add seen key tokens.
                for k in text.get_keys(qwerty, false) {
                    seen_replacement.push(k.to_string());
                }
            }
            // Get the MIDI binding.
            if let Some(midi) = bindings.1 {
                if has_qwerty {
                    // Or...
                    let or_str = text.get("OR").trim().to_string();
                    spoken_replacement.push(or_str.clone());
                    seen_replacement.push(or_str.clone());
                    // Get the MIDI binding.
                    let midi = match &midi.alias {
                        Some(alias) => alias.clone(),
                        None => text.get_with_values(
                            "MIDI_CONTROL",
                            &[&midi.bytes[0].to_string(), &midi.bytes[1].to_string()],
                        ),
                    };
                    spoken_replacement.push(midi.clone());
                    seen_replacement.push(midi);
                }
            }
            // Replace.
            spoken = regexes[&i]
                .replace(&spoken, &spoken_replacement.join(" "))
                .to_string();
            seen = regexes[&i]
                .replace(&seen, &seen_replacement.join(" "))
                .to_string();
        }
        // Iterate through each value.
        let mut regexes = HashMap::new();
        for (i, value) in values.iter().enumerate() {
            // Get the value regex.
            let regex = &self.re_values[i];
            regexes.insert(i, regex.clone());
            // Replace the value wildcard.
            spoken = regex.replace(&spoken, *value).to_string();
            seen = regex.replace(&seen, *value).to_string();
        }
        TtsString { spoken, seen }
    }

    fn get_regexes(prefix: &str) -> Regexes {
        [0; NUM_REGEXES].map(|i| Regex::new(&format!("{}{}", prefix, i)).unwrap())
    }
}