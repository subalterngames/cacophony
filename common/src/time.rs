use crate::edit_mode::get_index;
use crate::{Index, U64orF32};
use serde::{Deserialize, Serialize};
use time::Duration;

/// The default BPM.
pub const DEFAULT_BPM: u64 = 120;
/// Converts BPM to seconds.
const BPM_TO_SECONDS: f32 = 60.0;
/// Pulses per quarter note as a u64.
pub const PPQ_U: u64 = 192;
/// Pulses per quarter note.
pub const PPQ_F: f32 = PPQ_U as f32;

/// The time state.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Time {
    /// The time defining the position of the cursor.
    pub cursor: u64,
    /// The time at which playback will start.
    pub playback: u64,
    /// The beats per minute.
    pub bpm: U64orF32,
    /// The current edit mode.
    pub mode: Index,
    /// The framerate.
    pub framerate: U64orF32,
}

impl Time {
    /// Converts pulses per quarter note into seconds.
    pub fn ppq_to_seconds(&self, ppq: u64) -> f32 {
        ppq as f32 * (BPM_TO_SECONDS / (self.bpm.get_f() * PPQ_F))
    }

    /// Converts pulses per quarter note into a quantity of samples.
    pub fn ppq_to_samples(&self, ppq: u64, framerate: f32) -> u64 {
        (self.ppq_to_seconds(ppq) * framerate) as u64
    }

    /// Converts pulses per quarter note into a duration
    pub fn ppq_to_duration(&self, ppq: u64) -> Duration {
        Duration::seconds_f32(self.ppq_to_seconds(ppq))
    }

    /// Converts a quantity of samples into pulses per quarter note.
    pub fn samples_to_ppq(&self, samples: u64) -> u64 {
        ((self.bpm.get_f() * samples as f32) / (BPM_TO_SECONDS * self.framerate.get_f()) * PPQ_F)
            as u64
    }

    /// Returns a time string of pulses per quarter note.
    pub fn ppq_to_string(&self, ppq: u64) -> String {
        let time = self.ppq_to_duration(ppq);
        format!(
            "{:02}:{:02}:{:02}.{:03}",
            time.whole_hours(),
            time.whole_minutes(),
            time.whole_seconds() % 60,
            time.subsec_milliseconds()
        )
    }
}

impl Default for Time {
    fn default() -> Self {
        Self {
            cursor: 0,
            playback: 0,
            bpm: U64orF32::from(DEFAULT_BPM),
            mode: get_index(),
            framerate: U64orF32::from(44100),
        }
    }
}
