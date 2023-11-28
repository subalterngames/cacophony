use crate::soundfont_banks::SoundFontBanks;
use oxisynth::{GeneratorType, Synth};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

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
    #[serde(skip)]
    pub(crate) chorus: f32,
    #[serde(skip)]
    pub(crate) pan: f32,
    #[serde(skip)]
    pub(crate) reverb: f32,
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
            chorus: self.chorus,
            pan: self.pan,
            reverb: self.reverb,
        }
    }
}

impl Program {
    pub(crate) fn new(channel: u8, synth: &Synth, path: &Path, soundfont: &SoundFontBanks) -> Self {
        let chorus = synth.gen(channel, GeneratorType::ChorusSend).unwrap();
        let pan = synth.gen(channel, GeneratorType::Pan).unwrap();
        let reverb = synth.gen(channel, GeneratorType::ReverbSend).unwrap();

        // Get the bank info.
        let mut banks: Vec<u32> = soundfont.banks.keys().copied().collect();
        banks.sort();
        let bank = banks[0];
        let preset = soundfont.banks[&bank][0];
        // Select the default program.
        let preset_name = synth.channel_preset(channel).unwrap().name().to_string();
        Self {
            path: path.to_path_buf(),
            num_banks: banks.len(),
            bank_index: 0,
            bank,
            num_presets: soundfont.banks[&bank].len(),
            preset_index: 0,
            preset_name,
            preset,
            chorus,
            pan,
            reverb,
        }
    }
}
