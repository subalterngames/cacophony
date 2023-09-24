use serde::{Deserialize, Serialize};

/// This determines what we're exporting to.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize, Serialize, Default)]
pub enum ExportType {
    #[default]
    Wav,
    Mid,
    MP3,
    Ogg,
}

impl ExportType {
    /// Returns the file extension associated with the export type.
    ///
    /// - `period` If true, the extension starts with a ".", e.g. ".wav".
    pub fn get_extension(&self, period: bool) -> &str {
        match self {
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
                    "mid "
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
        }
    }
}
