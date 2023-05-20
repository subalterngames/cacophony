use cacophony_input::{Input, InputEvent};
use cacophony_text::Text;

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
pub fn get_tooltip(key: &str, events: &[InputEvent], input: &Input, text: &Text) -> String {
    let mut s = text.get(key);
    // Iterate through each event.
    for (i, event) in events.iter().enumerate() {
        // Get the key bindings.
        let bindings = input.get_bindings(event);
        // The replacement string.
        let mut r = String::new();
        let mut has_qwerty = false;
        // Get the qwerty binding.
        if let Some(qwerty) = bindings.0 {
            has_qwerty = true;
            // Get mods and keys.
            let mods = qwerty
                .mods
                .iter()
                .map(|k| text.get_keycode(k))
                .collect::<Vec<String>>()
                .join(" ");
            let keys = qwerty
                .keys
                .iter()
                .map(|k| text.get_keycode(k))
                .collect::<Vec<String>>()
                .join(" ");
            let num_keys = keys.chars().count();
            // Add the mods to the string.
            if mods.chars().count() > 0 {
                r.push_str(&mods);
                // Add a space for the keys.
                if num_keys > 0 {
                    r.push(' ');
                }
            }
            // Add the keys.
            if num_keys > 0 {
                r.push_str(&keys);
            }
        }
        // Get the MIDI binding.
        if let Some(midi) = bindings.1 {
            if has_qwerty {
                r.push_str(&text.get("OR"));
                let midi = match &midi.alias {
                    Some(alias) => alias.clone(),
                    None => text.get_with_values(
                        "MIDI_CONTROL",
                        &[&midi.bytes[0].to_string(), &midi.bytes[1].to_string()],
                    ),
                };
                r.push_str(&midi);
            }
        }
        // Replace the value.
        replace(&mut s, i, &r);
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

/// Replace a numbered arg, e.g. \0, with a value.
fn replace(s: &mut String, i: usize, to: &str) {
    let mut n = "\\".to_string();
    n.push_str(&i.to_string());
    *s = s.replace(&n, to);
}
