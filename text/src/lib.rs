mod text;
mod tts;
pub use self::tts::TTS;
use common::Fraction;
pub use text::Text;

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

/// Truncate a string to fit a specified length.
///
/// - `string` The string.
/// - `length` The maximum length of the string.
/// - `left` If true, remove characters from the left. Example: `"ABCDEFG" -> `"DEFG"`. If false, remove characters from the right. Example: `"ABCDEFG" -> `"ABCD"`.
pub fn truncate(string: &str, length: usize, left: bool) -> String {
    let len = string.chars().count();
    if len <= length {
        string.to_string()
    }
    // Remove characters on the left.
    else if left {
        string[len - length..len].to_string()
    }
    // Remove characters on the right.
    else {
        string[0..length].to_string()
    }
}
