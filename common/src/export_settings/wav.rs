use serde::{Deserialize, Serialize};

/// The sample format for the .wav file.
#[derive(Clone, Deserialize, Serialize)]
pub enum SampleFormat {
    I8,
    I16,
    I32,
    F32,
}

/// Export settings for .wav files.
#[derive(Clone, Deserialize, Serialize)]
pub struct Wav {
    /// The framerate (sample rate).
    pub framerate: u32,
    /// If true, mono. If false, stereo.
    pub mono: bool,
    /// The sample format.
    pub sample_format: SampleFormat,
}

impl Default for Wav {
    fn default() -> Self {
        Self {
            framerate: 44100,
            mono: false,
            sample_format: SampleFormat::I16,
        }
    }
}
