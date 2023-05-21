use crate::Program;
use crate::TimeState;
use hashbrown::HashMap;

/// The state of the synthesizer.
pub struct SynthState {
    /// The program state per channel.
    pub programs: HashMap<u8, Program>,
    /// The current playback time.
    pub time: TimeState,
    /// The current gain.
    pub gain: u8,
}

impl SynthState {
    /// A wrapper for time that returns 0 if there is no time.
    pub(crate) fn time(&self) -> u64 {
        self.time.time.unwrap_or(0)
    }
}

impl Default for SynthState {
    fn default() -> Self {
        Self {
            programs: HashMap::new(),
            time: TimeState::default(),
            gain: 127,
        }
    }
}

impl Clone for SynthState {
    fn clone(&self) -> Self {
        Self {
            programs: self.programs.clone(),
            time: self.time,
            gain: self.gain,
        }
    }
}