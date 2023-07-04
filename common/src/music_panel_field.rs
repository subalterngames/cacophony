use serde::{Deserialize, Serialize};

/// Enum values defining the music panel fields.
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash, Deserialize, Serialize)]
pub enum MusicPanelField {
    #[default]
    Name,
    BPM,
    Gain,
}
