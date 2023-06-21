use crate::{Paths, PPQ_F, PPQ_U};
use ini::{Ini, Properties};
use serde_json::from_str;
use std::fmt::Display;
use std::str::FromStr;

/// Load the config file.
pub fn load() -> Ini {
    let paths = Paths::default();
    let path = if paths.user_ini_path.exists() {
        paths.user_ini_path
    } else {
        paths.default_ini_path
    };
    match Ini::load_from_file(&path) {
        Ok(ini) => ini,
        Err(error) => panic!("Error loading confi.ini from {:?}: {}", path, error),
    }
}

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

/// Parse a 1 or 0 as a boolean.
pub fn parse_bool(properties: &Properties, key: &str) -> bool {
    match properties.get(key) {
        Some(value) => match value {
            "1" => true,
            "0" => false,
            _ => panic!("Invalid boolean value {} {}", key, value),
        },
        None => panic!("Missing key {}", key),
    }
}

/// Parse a PPQ value from a string.
pub fn parse_ppq(properties: &Properties, key: &str) -> u64 {
    match properties.get(key) {
        Some(value) => value_to_ppq(key, value),
        None => panic!("Missing key {}", key),
    }
}

/// Parse a list of fraction strings to PPQ values.
pub fn parse_fractions(properties: &Properties, key: &str) -> Vec<u64> {
    match properties.get(key) {
        Some(value) => match from_str::<Vec<&str>>(value) {
            Ok(value) => value.iter().map(|v| value_to_ppq(key, v)).collect(),
            Err(error) => panic!(
                "Error parsing list of fractions {} for key {}: {}",
                value, key, error
            ),
        },
        None => panic!("Missing key {}", key),
    }
}

/// Parse a value string as a PPQ value.
fn value_to_ppq(key: &str, value: &str) -> u64 {
    // Is this formatted like a fraction, e.g. "1/2"?
    match value.contains('/') {
        true => {
            let nd: Vec<&str> = value.split('/').collect();
            match nd[0].parse::<f32>() {
                Ok(n) => match nd[1].parse::<f32>() {
                    Ok(d) => ((n / d) * PPQ_F) as u64,
                    Err(error) => panic!(
                        "Invalid denominator in fraction {} for key {}: {}",
                        value, key, error
                    ),
                },
                Err(error) => panic!(
                    "Invalid numerator in fraction {} for key {}: {}",
                    value, key, error
                ),
            }
        }
        // Is this formated like a decimal, e.g. "0.5"?
        false => match value.contains('.') {
            true => match value.parse::<f32>() {
                Ok(value) => (value * PPQ_F) as u64,
                Err(error) => panic!("Invalid value {} for key {}: {}", value, key, error),
            },
            // Is it formatted like an integer, e.g. "1"?
            false => match value.parse::<u64>() {
                Ok(value) => value * PPQ_U,
                Err(error) => panic!("Invalid value {} for key {}: {}", value, key, error),
            },
        },
    }
}
