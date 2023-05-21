use crate::Fraction;
use fraction::ToPrimitive;
use time::Duration;

/// Converts BPM to seconds.
const BPM_TO_SECONDS: f64 = 60.0;
/// The framerate as a f64 value.
const FRAMERATE: f64 = 44100.0;

/// Converts a time duration into a bar length.
pub fn duration_to_bar(time: Duration, bpm: u32) -> Fraction {
    Fraction::from(time.as_seconds_f64() * (bpm as f64 / BPM_TO_SECONDS))
}

/// Converts a number of samples to a bar length.
pub fn samples_to_bar(samples: u64, bpm: u32) -> Fraction {
    duration_to_bar(Duration::seconds_f64(samples as f64 / FRAMERATE), bpm)
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
