use super::FileOrDirectory;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A directory and, optionally, a filename.
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct FileAndDirectory {
    /// The file's directory.
    pub directory: FileOrDirectory,
    /// The filename, if any.
    pub filename: Option<String>,
}

impl FileAndDirectory {
    /// Create from a `PathBuf` directory; we assume here that this isn't a file.
    pub fn new_directory(directory: PathBuf) -> Self {
        Self {
            directory: FileOrDirectory::new(&directory),
            filename: None,
        }
    }

    /// Create from a `PathBuf` that may or may not be a file.
    pub fn new_path(path: PathBuf) -> Self {
        let directory = FileOrDirectory::new(&path.parent().unwrap());
        let filename = Some(path.file_name().unwrap().to_str().unwrap().to_string());
        Self {
            directory,
            filename,
        }
    }

    /// Returns the path of the directory + filename.
    pub fn get_path(&self) -> PathBuf {
        match &self.filename {
            Some(filename) => self.directory.path.join(filename),
            None => panic!("No filename for: {:?}", self.directory.path),
        }
    }

    /// Returns the path of the directory + filename.
    pub fn try_get_path(&self) -> Option<PathBuf> {
        self.filename
            .as_ref()
            .map(|filename| self.directory.path.join(filename))
    }

    /// Returns the filename if there is one or an empty string if there isn't.
    pub fn get_filename(&self) -> String {
        match &self.filename {
            Some(string) => string.clone(),
            None => String::new(),
        }
    }
}
