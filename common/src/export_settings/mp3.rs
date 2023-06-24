use crate::Index;
use serde::{Deserialize, Serialize};

pub const QUALITIES: [Quality; 4] = [
    Quality::Mediocre,
    Quality::Standard,
    Quality::Commendable,
    Quality::Superlative,
];

/// .mp3 export quality.
#[derive(Clone, Deserialize, Serialize)]
pub enum Quality {
    Superlative,
    Commendable,
    Standard,
    Mediocre,
}

/// Export settings for .mp3 files.
#[derive(Clone, Deserialize, Serialize)]
pub struct MP3 {
    /// The quality index value.
    pub quality: Index,
    /// The bit rate index.
    pub bit_rate: Index,
}

impl Default for MP3 {
    fn default() -> Self {
        Self {
            bit_rate: Index::new(8, 16),
            quality: Index::new(1, QUALITIES.len()),
        }
    }
}