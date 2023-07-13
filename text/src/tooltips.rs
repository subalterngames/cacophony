use crate::tts_string::TtsString;
use crate::{Text, Token};
use common::hashbrown::HashMap;
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
    /// The regex used to find an arbitrary binding wildcard.
    re_binding: Regex,
    /// The regex used to find wildcard values.
    re_values: RegexMap,
    /// The regex used to find an arbitrary value wildcard.
    re_value: Regex,
}

impl Default for Tooltips {
    fn default() -> Self {
        let re_binding = Regex::new(r"(\\\d+)").unwrap();
        let re_value = Regex::new(r"(%(\d+))").unwrap();
        Self { re_bindings: RegexMap::new(), 
            re_binding,
            re_values: RegexMap::new(),
            re_value
        }
    }
}

impl Tooltips {
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
        self.get_tooltip_with_values(key, events, &vec![], input, text)
    }

    /// Build a tooltip from a text lookup key and a list of events and another list of values.
    /// 
    /// - `key` The text lookup key, for example "TITLE_MAIN_MENU".
    /// - `events` An ordered list of input events. The index is used to find the wildcard in the text, e.g. if the index is 0 then the wildcard is "\0".
    /// - `values` An ordered list of string values. The index is used to find the wildcard in the text, e.g. if the index is 0 then the wildcard is "%0".
    /// - `input` The input manager.
    /// - `text` The text manager.
    /// 
    /// Returns a tooltip `TtsString`.
    pub fn get_tooltip_with_values(
        &mut self,
        key: &str,
        events: &[InputEvent],
        values: &[&str],
        input: &Input,
        text: &Text,
    ) -> TtsString {
        let mut spoken = text.get(key);
        let mut seen_bindings = HashMap::new();
        let mut regexes = HashMap::new();
        // Iterate through each event.
        for (i, event) in events.iter().enumerate() {
            let regex = self.get_regex(&i, true);
            regexes.insert(i, regex.clone());
            // Get the key bindings.
            let bindings = input.get_bindings(event);
            // The replacement string.
            let mut spoken_replacement = String::new();
            let mut seen_replacement = vec![];
            let mut has_qwerty = false;
            // Get the qwerty binding.
            if let Some(qwerty) = bindings.0 {
                has_qwerty = true;
                // Add spoken mods.
                for m in Self::get_mods(qwerty, text, true) {
                    spoken_replacement.push_str(m);
                    spoken_replacement.push(' ');
                }
                // Add seen mod tokens.
                for m in Self::get_mods(qwerty, text, false) {
                    seen_replacement.push(Token::Qwerty(m.to_string()))
                }
                // Add spoken keys.
                for k in Self::get_keys(qwerty, text, true) {
                    spoken_replacement.push_str(k);
                    spoken_replacement.push(' ');
                }
                // Add seen key tokens.
                for k in Self::get_keys(qwerty, text, false) {
                    seen_replacement.push(Token::Qwerty(k.to_string()))
                }
            }
            // Get the MIDI binding.
            if let Some(midi) = bindings.1 {
                if has_qwerty {
                    // Or...
                    spoken_replacement.push_str(&text.get("OR"));
                    seen_replacement.push(Token::Word(text.get("OR").trim().to_string()));
                    // Get the MIDI binding.
                    let midi = match &midi.alias {
                        Some(alias) => alias.clone(),
                        None => text.get_with_values(
                            "MIDI_CONTROL",
                            &[&midi.bytes[0].to_string(), &midi.bytes[1].to_string()],
                        ),
                    };
                    spoken_replacement.push_str(&midi);
                    seen_replacement.push(Token::MIDI(midi));
                }
            }
            // Replace the value.
            spoken = regexes[&i].replace(&spoken, &spoken_replacement).to_string();
            // Add the value to the tokens map.
            seen_bindings.insert(i, seen_replacement);
        }
        // Iterate through each value.
        let mut seen_values = HashMap::new();
        let mut regexes = HashMap::new();
        for (i, value) in values.iter().enumerate() {
            // Get the value regex.
            let regex = self.get_regex(&i, false);
            regexes.insert(i, regex.clone());
            // Replace the value wildcard.
            spoken = regex.replace(&spoken, *value).to_string();
            // Update the scene values.
            seen_values.insert(i, Token::Word(value.to_string()));
        }
        // Tokenize the string.
        let mut tokens = vec![];
        let string = text.get(key);
        for word in string.split_whitespace() {
            if word.is_empty() {
                continue;
            }
            // This is a binding.       
            else if let Some(caps) = self.re_binding.captures(word) {
                let num = str::parse::<usize>(caps.get(2).unwrap().as_str()).unwrap();
                tokens.extend(seen_bindings[&num].clone());
            }
            // This is a value.
            else if let Some(caps) = self.re_value.captures(word) {
                let num = str::parse::<usize>(caps.get(2).unwrap().as_str()).unwrap();
                tokens.push(seen_values[&num].clone());
            }
            // This is a word.
            else {
                tokens.push(Token::Word(word.to_string()));
            }
        }
        TtsString { spoken, tokens }
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