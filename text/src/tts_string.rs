use crate::token::Token;

/// A text-to-speech string is spoken via a TTS engine and displayed as subtitles.
#[derive(Default, Clone)]
pub struct TtsString {
    /// This string will be fed to the TTS engine.
    pub spoken: String,
    /// These tokens will be displayed on the screen.
    pub tokens: Vec<Token>,
    /// The width and height of the text of this string.
    pub size: [f32; 2]
}

impl TtsString {
    pub(crate) fn is_empty(&self) -> bool {
        self.spoken.is_empty()
    }
}