use ini::{Ini, Properties};
use std::fmt::Display;
use std::str::FromStr;
use crate::Paths;

/// Load the config file.
pub fn load() -> Ini {
    let paths = Paths::new();
    let path = if paths.user_ini_path.exists() {
        paths.user_ini_path
    }
    else {
        paths.default_ini_path
    };
    match Ini::load_from_file(&path) {
        Ok(ini) => ini,
        Err(error) => panic!("Error loading confi.ini from {:?}: {}", path, error)
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
