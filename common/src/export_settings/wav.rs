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
    /// If true, mono. If false, stereo.
    pub mono: bool,
    /// The sample format.
    pub sample_format: SampleFormat,
}

impl Default for Wav {
    fn default() -> Self {
        Self {
            mono: false,
            sample_format: SampleFormat::I16,
        }
    }
}
