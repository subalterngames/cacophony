use crate::tts_string::TtsString;
use crate::Text;
use common::hashbrown::HashMap;
use common::hashbrown::hash_map::Entry;
use input::{Input, InputEvent, QwertyBinding};
use regex::Regex;

type RegexMap = HashMap<usize, Regex>;

/// Create tooltips and manage the regex's that find them.
/// 
/// There are two wilcards we're looking for:
/// 
/// 1. Bindings are MIDI/qwerty bindings. They look like this: "\0".
/// 2. Values are string wildcards that get replaced by a variable, e.g. the BPM. They look like this: "%0".
pub struct Tooltips {
    /// The regex used to find bindings.
    re_bindings: RegexMap,
    /// The regex used to find wildcard values.
    re_values: RegexMap,
    /// A hashmap of cached tooltips. Key = the lookup key.
    strings: HashMap<String, TtsString>
}

impl Tooltips {
    pub(crate) fn new() -> Self {
        Self { re_bindings: RegexMap::new(), 
            re_values: RegexMap::new(),
            strings: HashMap::new()
        }
    }

    /// Build a tooltip from a text lookup key and a list of events.
    /// 
    /// - `key` The text lookup key, for example "TITLE_MAIN_MENU".
    /// - `events` An ordered list of input events. These will be inserted in the order that the binding wildcards are found.
    /// - `input` The input manager.
    /// - `text` The text manager.
    /// 
    /// Returns a tooltip `TtsString`.
    pub fn get_tooltip(
        &mut self,
        key: &str,
        events: &[InputEvent],
        input: &Input,
        text: &Text,
    ) -> TtsString {
        if let Entry::Occupied(o) = self.strings.entry(key.to_string()) {
            o.get().clone()
        }
        else {
            let t = self.get_tooltip_with_values(key, events, &vec![], input, text);
            self.strings.insert(key.to_string(), t);
            self.strings[key].clone()
        }
    }

    /// Build a tooltip from a text lookup key and a list of events and another list of values.
    /// 
    /// - `key` The text lookup key, for example "TITLE_MAIN_MENU".
    /// - `events` An ordered list of input events. The index is used to find the wildcard in the text, e.g. if the index is 0 then the wildcard is "\0".
    /// - `values` An ordered list of string values. The index is used to find the wildcard in the text, e.g. if the index is 0 then the wildcard is "%0".
    /// - `input` The input manager.
    /// - `text` The text manager.
    /// 
    /// Returns a list of text-to-speech strings.
    pub fn get_tooltip_with_values(
        &mut self,
        key: &str,
        events: &[InputEvent],
        values: &[&str],
        input: &Input,
        text: &Text,
    ) -> TtsString {
        // Get the string with the wildcards.
        let raw_string = text.get(key);
        let mut spoken = raw_string.clone();
        let mut seen = raw_string.clone();
        let mut regexes = HashMap::new();
        // Iterate through each event.
        for (i, event) in events.iter().enumerate() {
            let regex = self.get_regex(&i, true);
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
                for m in Self::get_mods(qwerty, text, true) {
                    spoken_replacement.push(m.to_string());
                }
                // Add seen mod tokens.
                for m in Self::get_mods(qwerty, text, false) {
                    seen_replacement.push(m.to_string());
                }
                // Add spoken keys.
                for k in Self::get_keys(qwerty, text, true) {
                    spoken_replacement.push(k.to_string());
                }
                // Add seen key tokens.
                for k in Self::get_keys(qwerty, text, false) {
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
            spoken = regexes[&i].replace(&spoken, &spoken_replacement.join(" ")).to_string();
            seen = regexes[&i].replace(&seen, &seen_replacement.join(" ")).to_string();
        }
        // Iterate through each value.
        let mut regexes = HashMap::new();
        for (i, value) in values.iter().enumerate() {
            // Get the value regex.
            let regex = self.get_regex(&i, false);
            regexes.insert(i, regex.clone());
            // Replace the value wildcard.
            spoken = regex.replace(&spoken, *value).to_string();
            seen = regex.replace(&seen, *value).to_string();
        }
        TtsString { spoken, seen }
    }

    /// Returns a regex that searches for wildcard `i`. Creates a regext if there is none.
    /// 
    /// - `i` The wildcard value.
    /// - `bindings` If true, add a regex to `self.re_bindings`. If false add a regex to `self.re_values`.
    fn get_regex<'a>(&'a mut self, i: &usize, bindings: bool) -> &'a Regex {
        if bindings {
            Self::get_or_insert_regex(i, &mut self.re_bindings, r"\\")
        }
        else {
            Self::get_or_insert_regex(i, &mut self.re_values, "%") 
        }
    }

    // Get or insert a regex in a HashMap.
    fn get_or_insert_regex<'a>(i: &usize, map: &'a mut RegexMap, prefix: &str) -> &'a Regex {
        map.entry(*i).or_insert(Self::get_regex_from_index(i, prefix))
    }

    /// Returns a regex generated from index `i` and a string `prefix`.
    fn get_regex_from_index(i: &usize, prefix: &str) -> Regex {
        let mut r = prefix.to_string();
        r.push_str(&i.to_string());
        Regex::new(&r).unwrap()
    }

    /// Returns a qwerty binding's mods as strings. 
    /// 
    /// The strings may be different depending on the value of `spoken` i.e. whether this is meant to be spoken or seen.
    fn get_mods<'a>(qwerty: &QwertyBinding, text: &'a Text, spoken: bool) -> Vec<&'a str> {
        qwerty
            .mods
            .iter()
            .map(|k| text.get_keycode(k, spoken))
            .collect::<Vec<&str>>()
    }

    /// Returns a qwerty binding's keys as strings. 
    /// 
    /// The strings may be different depending on the value of `spoken` i.e. whether this is meant to be spoken or seen.
    fn get_keys<'a>(qwerty: &QwertyBinding, text: &'a Text, spoken: bool) -> Vec<&'a str> {
        qwerty
            .keys
            .iter()
            .map(|k| text.get_keycode(k, spoken))
            .collect::<Vec<&str>>()
    }
}