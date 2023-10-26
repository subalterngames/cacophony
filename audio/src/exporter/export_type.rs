use common::open_file::Extension;
use serde::{Deserialize, Serialize};

/// This determines what we're exporting to.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize, Serialize, Default)]
pub enum ExportType {
    #[default]
    Wav,
    Mid,
    MP3,
    Ogg,
    Flac,
}

impl From<ExportType> for Extension {
    fn from(val: ExportType) -> Self {
        match val {
            ExportType::Wav => Extension::Wav,
            ExportType::Mid => Extension::Mid,
            ExportType::MP3 => Extension::MP3,
            ExportType::Ogg => Extension::Ogg,
            ExportType::Flac => Extension::Flac,
        }
    }
}
