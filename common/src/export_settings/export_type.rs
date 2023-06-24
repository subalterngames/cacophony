/// This determines what we're exporting to.
pub enum ExportType {
    Wav,
    Mid,
    MP3
}

pub const EXPORT_TYPES: [ExportType; 3] = [ExportType::Wav, ExportType::Mid, ExportType::MP3];
pub const EXPORT_TYPE_STRINGS: [&str; 3] = ["wav", "mid", "mp3"];
