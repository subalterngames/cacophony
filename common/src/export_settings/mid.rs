use serde::{Deserialize, Serialize};

/// Export settings for .wav files.
#[derive(Clone, Deserialize, Serialize)]
pub struct Mid {
    /// If true, included copyright information.
    pub copyright: bool,
}

impl Default for Mid {
    fn default() -> Self {
        Self { copyright: true }
    }
}
