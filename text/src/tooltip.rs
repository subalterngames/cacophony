use crate::tts_string::TtsString;
use crate::{Text, Token};
use common::hashbrown::HashMap;
use input::{Input, InputEvent, QwertyBinding};
use regex::Regex;

type RegexMap = HashMap<usize, Regex>;

pub struct TooltipManager {
    /// The regex used to find bindings.
    re_bindings: RegexMap,
    /// The regex used to find wildcard values.
    re_values: RegexMap,
    tooltips: HashMap<String, TtsString>,
}

impl TooltipManager {
    pub fn get_tooltip<'a>(
        &'a mut self,
        key: &str,
        events: &[InputEvent],
        input: &Input,
        text: &Text,
    ) -> &'a TtsString {
        match &self.tooltips.get(key) {
            Some(tooltip) => *tooltip,
            None => {
                let mut spoken = text.get(key);
                let mut seen = HashMap::new();
                let mut regexes = HashMap::new();
                // Iterate through each event.
                // Iterate through each event.
                for (i, event) in events.iter().enumerate() {
                    regexes.insert(i, self.get_regex(&i, true));
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
                        for m in get_mods(qwerty, text, true) {
                            spoken_replacement.push_str(m);
                            spoken_replacement.push(' ');
                        }
                        // Add seen mod tokens.
                        for m in get_mods(qwerty, text, false) {
                            seen_replacement.push(Token::Qwerty(m.to_string()))
                        }
                        // Add spoken keys.
                        for k in get_keys(qwerty, text, true) {
                            spoken_replacement.push_str(k);
                            spoken_replacement.push(' ');
                        }
                        // Add seen key tokens.
                        for k in get_keys(qwerty, text, false) {
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
                    seen.insert(i, seen_replacement);
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
                        tokens.extend(seen[&num]);
                    }
                    // This is a word.
                    else {
                        tokens.push(Token::Word(word.to_string()));
                    }
                }
                let tts = TtsString { spoken, tokens };
                self.tooltips.insert(key.to_string(), tts);
                &self.tooltips[key]
            }
        }
    }

    fn get_regex(&mut self, i: &usize, bindings: bool) -> &Regex {
        if bindings {
            Self::get_or_insert_regex(i, &mut self.re_bindings)
        }
        else {
            Self::get_or_insert_regex(i, &mut self.re_values) 
        }
    }

    fn get_or_insert_regex<'a>(i: &usize, map: &'a mut RegexMap) -> &'a Regex {
        match &map.get(i) {
            Some(regex) => *regex,
            None => {
                let mut r = r"\\".to_string();
                r.push_str(&i.to_string());
                let regex = Regex::new(r).unwrap();
                map.insert(*i, regex);
                &map[i]
            }
        }
    }
}

/// Replace numbered args with the qwerty/MIDI bindings.
///
/// For example, if:
///
/// - `text.get(key)` is "\0 to cycle to the next panel."
/// - `events` is `&[InputEvent::NextPanel]`.
/// - The qwerty binding of `InputEvent::NewPanel` is `"Page Up"`.
/// - The MIDI binding of `InputEvent::NewPanel` is `[176, 16] up`.
/// - The MIDI alias of `[176, 16] up` is `"Knob 1"`.
///
/// ...Then the returned string is: `"Page up or Knob 1 to cycle to the next panel."`
pub fn get_tooltip(key: &str, events: &[InputEvent], input: &Input, text: &Text) -> TtsString {
    let mut spoken = text.get(key);
    let mut seen = HashMap::new();
    // Iterate through each event.
    for (i, event) in events.iter().enumerate() {
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
            for m in get_mods(qwerty, text, true) {
                spoken_replacement.push_str(m);
                spoken_replacement.push(' ');
            }
            // Add seen mod tokens.
            for m in get_mods(qwerty, text, false) {
                seen_replacement.push(Token::Qwerty(m.to_string()))
            }
            // Add spoken keys.
            for k in get_keys(qwerty, text, true) {
                spoken_replacement.push_str(k);
                spoken_replacement.push(' ');
            }
            // Add seen key tokens.
            for k in get_keys(qwerty, text, false) {
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
        replace(&mut spoken, i, &spoken_replacement);
        // Add the value to the tokens map.
        seen.insert(i, seen_replacement);
    }
    // Tokenize the string.
    let mut tokens = vec![];
    let string = text.get(key);
    for word in string.split_whitespace() {
        if word.is_empty() {
            continue;
        }
    }
    s
}

/// Replace numbered args with the qwerty/MIDI bindings *and* values.
///
/// Values are assumed to be numbered after events, even if they precede events in the string.
///
/// For example, if:
///
/// - `text.get(key)` is "\0 to cycle to the \1."
/// - `events` is `&[InputEvent::NextPanel]`.
/// - `values` is `&["next panel"]`.
/// - The qwerty binding of `InputEvent::NewPanel` is `"Page Up"`.
/// - The MIDI binding of `InputEvent::NewPanel` is `[176, 16] up`.
/// - The MIDI alias of `[176, 16] up` is `"Knob 1"`.
///
/// ...Then the returned string is: `"Page up or Knob 1 to cycle to the next panel."`
pub fn get_tooltip_with_values(
    key: &str,
    events: &[InputEvent],
    values: &[&str],
    input: &Input,
    text: &Text,
) -> String {
    // Get the tooltip.
    let mut s = get_tooltip(key, events, input, text);
    let len = events.len();
    for (i, value) in values.iter().enumerate() {
        // Replace the value.
        replace(&mut s, i + len, value);
    }
    s
}

fn get_mods<'a>(qwerty: &QwertyBinding, text: &'a Text, spoken: bool) -> Vec<&'a str> {
    qwerty
        .mods
        .iter()
        .map(|k| text.get_keycode(k, spoken))
        .collect::<Vec<&str>>()
}

fn get_keys<'a>(qwerty: &QwertyBinding, text: &'a Text, spoken: bool) -> Vec<&'a str> {
    qwerty
        .keys
        .iter()
        .map(|k| text.get_keycode(k, spoken))
        .collect::<Vec<&str>>()
}
