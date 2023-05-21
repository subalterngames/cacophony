use serde::{Deserialize, Serialize};

/// A type of panel.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize, Serialize)]
pub enum PanelType {
    TracksList,
    Music,
    PianoRoll,
    OpenFile,
    WriteSave,
}
