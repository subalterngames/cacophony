use crate::token::Token;

/// A text-to-speech string is spoken via a TTS engine and displayed as subtitles.
pub struct TtsString {
    /// This string will be fed to the TTS engine.
    pub spoken: String,
    /// These tokens will be displayed on the screen.
    pub tokens: Vec<Token>,
}
