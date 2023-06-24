/// This determines what we're exporting to.
pub enum ExportType {
    Wav,
    Mid,
}

pub const EXPORT_TYPES: [ExportType; 2] = [ExportType::Wav, ExportType::Mid];
pub const EXPORT_TYPE_STRINGS: [&str; 2] = ["wav", "mid"];
