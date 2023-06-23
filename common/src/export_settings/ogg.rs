use serde::{Deserialize, Serialize};

/// Export settings for .ogg files.
#[derive(Clone, Deserialize, Serialize)]
pub struct Ogg {
    /// The framerate (sample rate).
    pub framerate: u32,
    /// The quality value.
    pub quality: u8,
}

impl Default for Ogg {
    fn default() -> Self {
        Self {
            framerate: 44100,
            quality: 5,
        }
    }
}
