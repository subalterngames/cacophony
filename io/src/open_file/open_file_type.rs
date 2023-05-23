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
}