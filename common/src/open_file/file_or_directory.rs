use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Cached data for a file or directory because is_file() is a little too slow for my taste.
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct FileOrDirectory {
    /// The file.
    pub path: PathBuf,
    /// If true, this is a file.
    pub is_file: bool,
}

impl FileOrDirectory {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            is_file: path.is_file(),
        }
    }
}
