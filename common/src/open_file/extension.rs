use serde::{Deserialize, Serialize};

/// Enum values for file extensions used in Cacophony.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize, Serialize)]
pub enum Extension {
    Cac,
    Sf2,
    Wav,
    Mid,
    MP3,
    Ogg,
    Flac,
}

impl Extension {
    /// Returns the file extension associated with the export type.
    ///
    /// - `period` If true, the extension starts with a ".", e.g. ".wav".
    pub fn to_str(&self, period: bool) -> &str {
        match self {
            Self::Cac => {
                if period {
                    ".cac"
                } else {
                    "cac"
                }
            }
            Self::Sf2 => {
                if period {
                    ".sf2"
                } else {
                    "sf2"
                }
            }
            Self::Wav => {
                if period {
                    ".wav"
                } else {
                    "wav"
                }
            }
            Self::Mid => {
                if period {
                    ".mid"
                } else {
                    "mid"
                }
            }
            Self::MP3 => {
                if period {
                    ".mp3"
                } else {
                    "mp3"
                }
            }
            Self::Ogg => {
                if period {
                    ".ogg"
                } else {
                    "ogg"
                }
            }
            Self::Flac => {
                if period {
                    ".flac"
                } else {
                    "flac"
                }
            }
        }
    }
}
