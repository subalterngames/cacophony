use std::fmt::Display;
use std::str::FromStr;
use ini::Properties;


/// Parse a string `value` and returns an enum of type `T`.
fn string_to_enum<T>(value: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    match value.parse::<T>() {
        Ok(value) => value,
        Err(error) => panic!("Failed to parse {}", error),
    }
}

/// Parse a config key-value string pair into a value of type T.
/// 
/// - `properties` The `Ini` properties.
/// - `key` the key portion of the key-value pair.
pub fn parse<T>(properties: &Properties, key: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    match properties.get(key) {
        Some(value) => string_to_enum(value),
        None => panic!("Missing key {}", key),
    }
}