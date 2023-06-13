use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A current working directory and, optionally, a filename.
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct FileAndDirectory {
    /// The current working directory.
    pub directory: PathBuf,
    /// The filename.
    pub filename: Option<String>,
}

impl FileAndDirectory {
    pub fn new_directory(directory: PathBuf) -> Self {
        Self {
            directory,
            filename: None,
        }
    }

    pub fn new_path(path: PathBuf) -> Self {
        let directory = path.parent().unwrap().to_path_buf();
        let filename = Some(path.file_name().unwrap().to_str().unwrap().to_string());
        Self {
            directory,
            filename,
        }
    }

    /// Returns the path of the directory + filename.
    pub fn get_path(&self) -> PathBuf {
        match &self.filename {
            Some(filename) => self.directory.join(filename),
            None => panic!("No filename for: {:?}", self.directory),
        }
    }

    /// Returns the path of the directory + filename.
    pub fn try_get_path(&self) -> Option<PathBuf> {
        self.filename
            .as_ref()
            .map(|filename| self.directory.join(filename))
    }

    pub fn get_filename(&self) -> String {
        match &self.filename {
            Some(string) => string.clone(),
            None => String::new(),
        }
    }
}
