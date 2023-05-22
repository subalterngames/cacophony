use crate::text::truncate;
use std::path::{Path, PathBuf};

#[derive(Eq, PartialEq)]
pub enum OpenFilePanelType {
    /// Load a SoundFont.
    Soundfont,
    /// Save a file.
    Save,
}

/// Cached data for a file or directory.
pub struct FileOrDirectory {
    /// The file.
    pub path: PathBuf,
    /// If true, this is a file.
    pub is_file: bool,
    /// The string representation of the path.
    pub as_string: String,
}

impl FileOrDirectory {
    pub fn new(path: &Path, length: usize) -> Self {
        let mut as_string = match path.to_str() {
            Some(s) => s.to_string(),
            None => String::new(),
        };
        as_string = truncate(&as_string, length, true);
        Self {
            path: path.to_path_buf(),
            is_file: path.is_file(),
            as_string,
        }
    }
}
