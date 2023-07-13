//! This crate handles three related, but separate, tasks:
//!
//! 1. `Text` stores localized text. Throughout Cacophony, all strings that will be spoken or drawn are referenced via lookup keys. The text data is in `data/text.csv`.
//! 2. `TTS` converts text-to-speech strings into spoken audio.
//! 3. This crate also contains language-agnostic string manipulation functions e.g. `truncate`.

mod text;
mod tts;
mod value_map;
pub use self::tts::TTS;
use common::{PPQ_F, PPQ_U};
use std::path::Path;
pub use text::Text;
pub use value_map::*;
mod token;
mod tooltips;
mod tts_string;
pub use token::Token;

pub fn ppq_to_string(ppq: u64) -> String {
    // This is a whole note.
    if ppq % PPQ_U == 0 {
        (ppq / PPQ_U).to_string()
    } else {
        match ppq {
            288 => "3/2".to_string(),
            96 => "1/2".to_string(),
            64 => "1/3".to_string(),
            48 => "1/4".to_string(),
            32 => "1/6".to_string(),
            24 => "1/8".to_string(),
            12 => "1/16".to_string(),
            6 => "1/32".to_string(),
            other => format!("{:.2}", (other as f32 / PPQ_F)),
        }
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

/// Returns the file name of a path.
pub fn get_file_name(path: &Path) -> String {
    match path.file_name() {
        Some(filename) => match filename.to_str() {
            Some(s) => s.to_string(),
            None => panic!("Invalid filename: {:?}", filename),
        },
        None => panic!("Not a file: {:?}", path),
    }
}

/// Returns the file name of a path without the extension.
pub fn get_file_name_no_ex(path: &Path) -> String {
    match path.file_stem() {
        Some(filename) => match path.to_str() {
            Some(s) => s.to_string(),
            None => panic!("Invalid filename: {:?}", filename),
        },
        None => panic!("Not a file: {:?}", path),
    }
}

/// Returns the folder name of a path.
pub fn get_folder_name(path: &Path) -> String {
    let components = path.components();
    match components.last() {
        Some(s) => match s.as_os_str().to_str() {
            Some(s) => s.to_string(),
            None => panic!("Invalid folder name: {:?}", s),
        },
        None => panic!("No path components: {:?}", path),
    }
}

/// Push a space to the string if there is none.
pub fn push_space(s: &mut String) {
    if let Some(last) = s.chars().last() {
        if last != ' ' {
            s.push(' ')
        }
    } else {
        s.push(' ');
    }
}
