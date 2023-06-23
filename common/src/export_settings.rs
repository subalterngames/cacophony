use serde::{Deserialize, Serialize};
mod metadata;
mod mid;
mod mp3;
mod multi_file;
mod ogg;
mod wav;
pub use metadata::*;
pub use mid::*;
pub use mp3::*;
pub use multi_file::*;
pub use ogg::*;
pub use wav::*;

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct ExportSettings {
    pub metadata: Metadata,
    pub mid: Mid,
    pub mp3: MP3,
    pub multi_file: MultiFile,
    pub ogg: Ogg,
    pub wav: Wav,
}
