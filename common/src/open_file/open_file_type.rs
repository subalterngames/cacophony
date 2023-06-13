const SOUNDFONT_EXTENSIONS: [&str; 2] = ["sf2", "sf3"];
const SAVE_FILE_EXTENSIONS: [&str; 1] = ["cac"];
const EXPORT_FILE_EXTENSIONS: [&str; 1] = ["wav"];

/// This defines the files we care about and what we can do with them.
#[derive(Eq, PartialEq, Clone, Default)]
pub enum OpenFileType {
    /// Read a save file.
    ReadSave,
    /// Read a SoundFont.
    #[default]
    SoundFont,
    /// Write a save file.
    WriteSave,
    /// Set the export path.
    Export,
}

impl OpenFileType {
    /// Returns the file extensions associated with this open-file-type.
    pub fn get_extensions(&self) -> &[&str] {
        match self {
            Self::Export => &EXPORT_FILE_EXTENSIONS,
            Self::ReadSave | Self::WriteSave => &SAVE_FILE_EXTENSIONS,
            Self::SoundFont => &SOUNDFONT_EXTENSIONS,
        }
    }
}
