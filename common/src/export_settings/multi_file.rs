use serde::{Deserialize, Serialize};

/// How should we name files of separate tracks?
#[derive(Clone, Default, Deserialize, Serialize)]
pub enum MultiFile {
    /// Preset name suffix.
    Preset,
    /// Channel integer suffix.
    Channel,
    /// Channel, then preset name.
    #[default]
    ChannelAndPreset,
    /// A custom suffix.
    Custom(String),
}
