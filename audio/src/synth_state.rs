use crate::Program;
use common::MAX_VOLUME;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

/// The state of the synthesizer.
#[derive(Serialize, Deserialize)]
pub struct SynthState {
    /// The program state per channel.
    pub programs: HashMap<u8, Program>,
    /// The current gain.
    pub gain: u8,
}

impl Default for SynthState {
    fn default() -> Self {
        Self {
            programs: HashMap::new(),
            gain: MAX_VOLUME,
        }
    }
}

impl Clone for SynthState {
    fn clone(&self) -> Self {
        Self {
            programs: self.programs.clone(),
            gain: self.gain,
        }
    }
}
