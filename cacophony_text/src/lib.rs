mod tts;
pub use self::tts::TTS;
use cacophony_core::Fraction;

/// Converts a fraction to a string.
pub fn fraction(fraction: &Fraction) -> String {
    if fraction.is_nan() {
        return "NaN".to_string();
    }
    let numer = fraction.numer().unwrap();
    match fraction.denom().unwrap() {
        // If the denominator is 1, return a whole number.
        1 => numer.to_string(),
        // Format as a fraction.
        other => format!("{}/{}", numer, other),
    }
}
