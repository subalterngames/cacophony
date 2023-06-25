/// This determines what we're exporting to.
#[derive(Eq, PartialEq)]
pub enum ExportType {
    Wav,
    Mid,
    MP3,
    Ogg,
}

pub const EXPORT_TYPES: [ExportType; 4] = [
    ExportType::Wav,
    ExportType::Mid,
    ExportType::MP3,
    ExportType::Ogg,
];
pub const EXPORT_TYPE_STRINGS: [&str; 4] = ["wav", "mid", "mp3", "ogg"];
