/// A text-to-speech string is spoken via a TTS engine and displayed as subtitles.
#[derive(Default, Clone)]
pub struct TtsString {
    /// This string will be spoken by the TTS engine.
    pub spoken: String,
    /// This string will be displayed on the screen.
    pub seen: String,
}