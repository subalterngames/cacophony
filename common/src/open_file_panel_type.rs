#[derive(Eq, PartialEq)]
pub enum OpenFilePanelType {
    /// Load a SoundFont.
    Soundfont,
    /// Save a file.
    Save,
}
