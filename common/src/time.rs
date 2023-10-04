use crate::edit_mode::*;
use crate::U64orF32;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// The default BPM.
pub const DEFAULT_BPM: u64 = 120;
/// Converts BPM to seconds.
const BPM_TO_SECONDS: f32 = 60.0;
/// Pulses per quarter note as a u64.
pub const PPQ_U: u64 = 192;
/// Pulses per quarter note.
pub const PPQ_F: f32 = PPQ_U as f32;
/// The default framerate.
pub const DEFAULT_FRAMERATE: u64 = 44100;

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
    pub mode: IndexedEditModes,
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
        Duration::from_secs_f32(self.ppq_to_seconds(ppq))
    }

    /// Converts a quantity of samples into pulses per quarter note.
    pub fn samples_to_ppq(&self, samples: u64, framerate: f32) -> u64 {
        ((self.bpm.get_f() * samples as f32) / (BPM_TO_SECONDS * framerate) * PPQ_F) as u64
    }
}

impl Default for Time {
    fn default() -> Self {
        Self {
            cursor: 0,
            playback: 0,
            bpm: U64orF32::from(DEFAULT_BPM),
            mode: EditMode::indexed(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::time::*;

    #[test]
    fn time() {
        let mut time = Time::default();

        // PPQ to seconds.

        ppq_seconds(0, 0.0, &time);
        ppq_seconds(PPQ_U, 0.5, &time);
        ppq_seconds(288, 0.75, &time);

        time.bpm = U64orF32::from(60);

        ppq_seconds(0, 0.0, &time);
        ppq_seconds(PPQ_U, 1.0, &time);
        ppq_seconds(288, 1.5, &time);

        time.bpm = U64orF32::from(DEFAULT_BPM);

        let framerate: f32 = 44100.0;

        // PPQ to samples.

        ppq_samples(0, 0, framerate, &time);
        ppq_samples(PPQ_U, 22050, framerate, &time);
        ppq_samples(288, 33075, framerate, &time);

        time.bpm = U64orF32::from(60);

        ppq_samples(PPQ_U, 44100, framerate, &time);
        ppq_samples(288, 66150, framerate, &time);

        ppq_samples(PPQ_U, 48000, 48000.0, &time);
        time.bpm = U64orF32::from(DEFAULT_BPM);
        ppq_samples(PPQ_U, 24000, 48000.0, &time);

        // Samples to PPQ.
        samples_ppq(0, 0, framerate, &time);
        samples_ppq(22050, PPQ_U, framerate, &time);
        samples_ppq(44100, PPQ_U * 2, framerate, &time);
    }

    fn ppq_seconds(ppq: u64, f: f32, time: &Time) {
        let t = time.ppq_to_seconds(ppq);
        assert_eq!(t, f, "{} {}", t, f);
    }

    fn ppq_samples(ppq: u64, v: u64, framerate: f32, time: &Time) {
        let s = time.ppq_to_samples(ppq, framerate);
        assert_eq!(s, v, "{} {} {} {}", ppq, s, v, framerate)
    }

    fn samples_ppq(samples: u64, v: u64, framerate: f32, time: &Time) {
        let ppq = time.samples_to_ppq(samples, framerate);
        assert_eq!(ppq, v, "{} {} {}", ppq, v, samples);
    }
}
