use serde::{Deserialize, Serialize};

/// .mp3 export quality.
#[derive(Clone, Deserialize, Serialize)]
pub enum Quality {
    Superlative,
    Commendable,
    Standard,
    Mediocre,
}

/// Export settings for .mp3 files.
#[derive(Clone, Deserialize, Serialize)]
pub struct MP3 {
    /// The framerate (sample rate).
    pub framerate: u32,
    /// The quality value.
    pub quality: Quality,
}

impl Default for MP3 {
    fn default() -> Self {
        Self {
            framerate: 44100,
            quality: Quality::Standard,
        }
    }
}
