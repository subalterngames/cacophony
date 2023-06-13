use crate::edit_mode::get_index;
use crate::{deserialize_fraction, serialize_fraction, Fraction, Index, SerializableFraction};
use fraction::{ToPrimitive, Zero};
use serde::{Deserialize, Serialize};
use time::Duration;

/// Converts BPM to seconds.
const BPM_TO_SECONDS: f64 = 60.0;
/// The framerate as a f64 value.
const FRAMERATE: f64 = 44100.0;
const FRAMERATE_U32: u32 = FRAMERATE as u32;
pub const FRAMERATE_U64: u64 = FRAMERATE as u64;
const BPM_TO_SECONDS_U32: u32 = 60;

/// The time state.
#[derive(Clone, Debug)]
pub struct Time {
    /// The time defining the position of the cursor.
    pub cursor: Fraction,
    /// The time at which playback will start.
    pub playback: Fraction,
    /// The current edit mode.
    pub mode: Index,
}

impl Time {
    pub(crate) fn serialize(&self) -> SerializableTime {
        SerializableTime {
            cursor: serialize_fraction(&self.cursor),
            playback: serialize_fraction(&self.playback),
            mode: self.mode,
        }
    }
}

impl Default for Time {
    fn default() -> Self {
        Self {
            cursor: Fraction::zero(),
            playback: Fraction::zero(),
            mode: get_index(),
        }
    }
}

/// Time is a series.
#[derive(Deserialize, Serialize)]
pub(crate) struct SerializableTime {
    /// The time defining the position of the cursor.
    pub(crate) cursor: SerializableFraction,
    /// The time at which playback will start.
    pub(crate) playback: SerializableFraction,
    /// The current edit mode.
    pub(crate) mode: Index,
}

impl SerializableTime {
    /// Returns a deserialized `Viewport`.
    pub(crate) fn deserialize(&self) -> Time {
        Time {
            cursor: deserialize_fraction(&self.cursor),
            playback: deserialize_fraction(&self.playback),
            mode: self.mode,
        }
    }
}

/// Converts a number of samples to a bar length.
pub fn samples_to_bar(samples: u64, bpm: u32) -> Fraction {
    Fraction::new(samples as u32, FRAMERATE_U32) * Fraction::new(bpm, BPM_TO_SECONDS_U32)
}

/// Converts a beats value (bar length) to a time value in seconds.
pub fn bar_to_seconds(bar: &Fraction, bpm: u32) -> f64 {
    bar.to_f64().unwrap() / (bpm as f64 / BPM_TO_SECONDS)
}

/// Converts a beats value (bar length) to duration.
pub fn bar_to_duration(bar: &Fraction, bpm: u32) -> Duration {
    Duration::seconds_f64(bar_to_seconds(bar, bpm))
}

/// Returns the number of samples in a bar.
pub fn bar_to_samples(bar: &Fraction, bpm: u32) -> u64 {
    (bar_to_seconds(bar, bpm) * FRAMERATE) as u64
}

/// Returns a time string of the bar length.
pub fn bar_to_time_string(bar: &Fraction, bpm: u32) -> String {
    let time = Duration::seconds_f64(bar_to_seconds(bar, bpm));
    format!(
        "{:02}:{:02}:{:02}.{:03}",
        time.whole_hours(),
        time.whole_minutes(),
        time.whole_seconds() % 60,
        time.subsec_milliseconds()
    )
}
