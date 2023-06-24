use serde::{Deserialize, Serialize};
mod metadata;
mod mid;
mod mp3;
mod multi_file;
mod ogg;
mod wav;
use crate::time::DEFAULT_FRAMERATE;
use crate::U64orF32;
pub use metadata::*;
pub use mid::*;
pub use mp3::*;
pub use multi_file::*;
pub use ogg::*;
pub use wav::*;

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct ExportSettings {
    pub framerate: U64orF32,
    pub metadata: Metadata,
    pub mid: Mid,
    pub mp3: MP3,
    pub multi_file: MultiFile,
    pub ogg: Ogg,
    pub wav: Wav,
}

impl ExportSettings {
    pub fn new() -> Self {
        Self {
            framerate: U64orF32::from(DEFAULT_FRAMERATE),
            ..Default::default()
        }
    }
}
