/// A text-to-speech string is spoken via a TTS engine and displayed as subtitles.
#[derive(Default, Clone)]
pub struct TtsString {
    /// This string will be spoken by the TTS engine.
    pub spoken: String,
    /// This string will be displayed on the screen.
    pub seen: String,
}

impl From<String> for TtsString {
    fn from(value: String) -> Self {
        Self {
            spoken: value.clone(),
            seen: value,
        }
    }
}

impl From<&str> for TtsString {
    fn from(value: &str) -> Self {
        Self {
            spoken: value.to_string(),
            seen: value.to_string(),
        }
    }
}

impl TtsString {
    // Append the strings of `other` to my strings.
    pub fn push(&mut self, other: &TtsString) {
        if !self.spoken.is_empty() {
            self.spoken.push(' ');
        }
        self.spoken.push_str(&other.spoken);
        if !self.seen.is_empty() {
            self.seen.push(' ');
        }
        self.seen.push_str(&other.seen);
    }
}
