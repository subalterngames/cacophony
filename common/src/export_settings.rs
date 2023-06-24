mod export_type;
use serde::{Deserialize, Serialize};
mod metadata;
mod mid;
mod mp3;
mod multi_file;
mod ogg;
use crate::time::DEFAULT_FRAMERATE;
use crate::Index;
use crate::U64orF32;
pub use export_type::*;
pub use metadata::*;
pub use mid::*;
pub use mp3::*;
pub use multi_file::*;
pub use ogg::*;

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct ExportSettings {
    pub framerate: U64orF32,
    pub metadata: Metadata,
    pub mid: Mid,
    pub mp3: MP3,
    pub multi_file: MultiFile,
    pub ogg: Ogg,
    /// The export type.
    pub export_type: Index,
}

impl ExportSettings {
    pub fn new() -> Self {
        Self {
            framerate: U64orF32::from(DEFAULT_FRAMERATE),
            export_type: Index::new(0, EXPORT_TYPES.len()),
            ..Default::default()
        }
    }
}
