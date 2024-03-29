use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A channel's program.
#[derive(Serialize, Deserialize)]
pub struct Program {
    /// The path to the current track's SoundFont.
    pub path: PathBuf,
    /// The total number of banks.
    pub num_banks: usize,
    /// The index of the bank in `banks`.
    pub bank_index: usize,
    /// The actual bank value.
    pub bank: u32,
    /// The total number of presets in the bank.
    pub num_presets: usize,
    /// The preset number.
    pub preset: u8,
    /// The index of the preset in `presets`.
    pub preset_index: usize,
    /// The name of the preset.
    pub preset_name: String,
}

impl Clone for Program {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            num_banks: self.num_banks,
            bank_index: self.bank_index,
            bank: self.bank,
            num_presets: self.num_presets,
            preset: self.preset,
            preset_index: self.preset_index,
            preset_name: self.preset_name.clone(),
        }
    }
}
