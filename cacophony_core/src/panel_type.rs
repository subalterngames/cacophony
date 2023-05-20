/// A type of panel.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum PanelType {
    TracksList,
    Music,
    PianoRoll,
    OpenFile,
    WriteSave,
}
