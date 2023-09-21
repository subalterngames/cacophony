use crate::{Text, TtsString};
use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use input::{Input, InputEvent, QwertyBinding};
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
        Self {
            tooltips,
            re_bindings,
            re_values,
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
        match self.tooltips.entry(key.to_string()) {
            Entry::Occupied(o) => o.get().clone(),
            Entry::Vacant(v) => {
                let mut seen = text.get(key);
                let mut spoken = seen.clone();
                for (i, event) in events.iter().enumerate() {
                    // Get the key bindings.
                    Self::event_to_text(
                        &self.re_bindings[i],
                        event,
                        input,
                        text,
                        &mut spoken,
                        &mut seen,
                    );
                }
                v.insert(TtsString { spoken, seen }).clone()
            }
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
        &self,
        key: &str,
        events: &[InputEvent],
        values: &[&str],
        input: &Input,
        text: &Text,
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
            Self::event_to_text(regex, event, input, text, &mut spoken, &mut seen);
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

    fn event_to_text(
        regex: &Regex,
        event: &InputEvent,
        input: &Input,
        text: &Text,
        spoken: &mut String,
        seen: &mut String,
    ) {
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
            for m in Self::get_mods(qwerty, true, text) {
                spoken_replacement.push(m.to_string());
            }
            // Add seen mod tokens.
            for m in Self::get_mods(qwerty, false, text) {
                seen_replacement.push(m.to_string());
            }
            // Add spoken keys.
            for k in Self::get_keys(qwerty, true, text) {
                spoken_replacement.push(k.to_string());
            }
            // Add seen key tokens.
            for k in Self::get_keys(qwerty, false, text) {
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
        *spoken = regex
            .replace(spoken, &spoken_replacement.join(" "))
            .to_string();
        *seen = regex.replace(seen, &seen_replacement.join(" ")).to_string();
    }

    /// Returns a qwerty binding's mods as strings.
    ///
    /// The strings may be different depending on the value of `spoken` i.e. whether this is meant to be spoken or seen.
    fn get_mods<'a>(qwerty: &QwertyBinding, spoken: bool, text: &'a Text) -> Vec<&'a str> {
        qwerty
            .mods
            .iter()
            .map(|k| text.get_keycode(k, spoken))
            .collect::<Vec<&str>>()
    }

    /// Returns a qwerty binding's keys as strings.
    ///
    /// The strings may be different depending on the value of `spoken` i.e. whether this is meant to be spoken or seen.
    fn get_keys<'a>(qwerty: &QwertyBinding, spoken: bool, text: &'a Text) -> Vec<&'a str> {
        qwerty
            .keys
            .iter()
            .map(|k| text.get_keycode(k, spoken))
            .collect::<Vec<&str>>()
    }
}
