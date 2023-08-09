use crate::fraction::*;
use crate::Paths;
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
fn string_to_value<T>(value: &str) -> T
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
        Some(value) => string_to_value(value),
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

/// Parse a list of fraction strings to PPQ values.
pub fn parse_fractions(properties: &Properties, key: &str) -> Vec<f32> {
    match properties.get(key) {
        Some(value) => match from_str::<Vec<&str>>(value) {
            Ok(value) => value.iter().map(|v| parse_float_kv(key, v)).collect(),
            Err(error) => panic!(
                "Error parsing list of fractions {} for key {}: {}",
                value, key, error
            ),
        },
        None => panic!("Missing key {}", key),
    }
}

/// Parse a value string as a float.
pub fn parse_float(properties: &Properties, key: &str) -> f32 {
    match properties.get(key) {
        Some(value) => parse_float_kv(key, value),
        None => panic!("Missing key {}", key),
    }
}

/// Parse a value string as a fraction.
pub fn parse_fraction(properties: &Properties, key: &str) -> Fraction {
    match properties.get(key) {
        Some(value) => parse_fraction_kv(key, value),
        None => panic!("Missing key {}", key),
    }
}

/// Parse a value string as a float.
fn parse_float_kv(key: &str, value: &str) -> f32 {
    // Is this formatted like a fraction, e.g. "1/2"?
    match value.contains('/') {
        true => {
            let nd: Vec<&str> = value.split('/').collect();
            match nd[0].parse::<f32>() {
                Ok(n) => match nd[1].parse::<f32>() {
                    Ok(d) => n / d,
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
        // Is this formated like a decimal, e.g. "0.5" or "5"?
        false => match value.parse::<f32>() {
            Ok(value) => value,
            Err(error) => panic!("Invalid value {} for key {}: {}", value, key, error),
        },
    }
}

/// Parse a value string as a Fraction.
fn parse_fraction_kv(key: &str, value: &str) -> Fraction {
    // Is this formatted like a fraction, e.g. "1/2"?
    match value.contains('/') {
        true => {
            let nd: Vec<&str> = value.split('/').collect();
            match nd[0].parse::<u64>() {
                Ok(n) => match nd[1].parse::<u64>() {
                    Ok(d) => Fraction::new(n, d),
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
        // Is this formated like a decimal, e.g. "0.5" or "5"?
        false => match value.parse::<u64>() {
            Ok(value) => Fraction::from(value),
            Err(error) => panic!("Invalid value {} for key {}: {}", value, key, error),
        },
    }
}
