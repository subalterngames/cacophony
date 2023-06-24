use crate::Index;
use serde::{Deserialize, Serialize};

const BIT_RATES: [u16; 16] = [
    8, 16, 24, 32, 40, 48, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320,
];

/// Export settings for .mp3 files.
#[derive(Eq, PartialEq, Copy, Clone, Deserialize, Serialize)]
pub struct MP3 {
    /// The quality index value.
    pub quality: Index,
    /// The bit rate index.
    pub bit_rate: Index,
}

impl MP3 {
    pub fn get_bit_rate(&self) -> u16 {
        BIT_RATES[self.bit_rate.get()]
    }
}

impl Default for MP3 {
    fn default() -> Self {
        Self {
            bit_rate: Index::new(8, 16),
            quality: Index::new(0, 10),
        }
    }
}
