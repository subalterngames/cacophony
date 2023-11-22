use crate::SharedSynth;
use hashbrown::HashMap;
use oxisynth::{SoundFont, SoundFontId};

/// A convenient wrapper for a SoundFont.
pub(crate) struct SoundFontBanks {
    pub(crate) id: SoundFontId,
    /// The banks and their presets.
    pub(crate) banks: HashMap<u32, Vec<u8>>,
}

impl SoundFontBanks {
    pub fn new(font: SoundFont, synth: &mut SharedSynth) -> Self {
        let mut banks: HashMap<u32, Vec<u8>> = HashMap::new();
        (0u32..=128u32).for_each(|b| {
            let presets: Vec<u8> = (0u8..128)
                .filter(|p| font.preset(b, *p).is_some())
                .collect();
            if !presets.is_empty() {
                banks.insert(b, presets);
            }
        });
        let mut synth = synth.lock();
        let id = synth.add_font(font, true);
        Self { id, banks }
    }
}
