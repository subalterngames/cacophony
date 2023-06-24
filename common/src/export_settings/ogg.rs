use crate::Index;
use serde::{Deserialize, Serialize};

/// Export settings for .ogg files.
#[derive(Clone, Deserialize, Serialize)]
pub struct Ogg {
    /// The quality value.
    pub quality: Index,
}

impl Default for Ogg {
    fn default() -> Self {
        Self {
            quality: Index::new(4, 10),
        }
    }
}
