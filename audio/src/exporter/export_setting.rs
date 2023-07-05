use serde::{Deserialize, Serialize};

/// Enum values for export settings.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Default, Deserialize, Serialize)]
pub enum ExportSetting {
    #[default]
    Framerate,
    Title,
    Artist,
    Copyright,
    Album,
    TrackNumber,
    Genre,
    Comment,
    Mp3BitRate,
    Mp3Quality,
    OggQuality,
    MultiFile,
    MultiFileSuffix,
}
