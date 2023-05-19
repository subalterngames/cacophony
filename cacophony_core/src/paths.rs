use directories::UserDirs;
use std::env::current_dir;
use std::path::PathBuf;

const CONFIG_FILENAME: &str = "config.ini";

/// Cached file paths.
pub struct Paths {
    /// The path to the default .ini file.
    pub default_ini_path: PathBuf,
    /// The path to the data/ directory.
    pub data_directory: PathBuf,
    /// The path to the user directory.
    pub user_directory: PathBuf,
    /// The path to the user-defined .ini file. Might not exist.
    pub user_ini_path: PathBuf,
}

impl Paths {
    pub fn new() -> Self {
        let user_directory = match UserDirs::new() {
            Some(user_dirs) => match user_dirs.document_dir() {
                Some(documents) => documents.join("cacophony"),
                None => user_dirs.home_dir().join("cacophony"),
            },
            None => {
                if cfg!(windows) {
                    PathBuf::from("C:/").join("cacophony")
                } else {
                    PathBuf::from("/").join("cacophony")
                }
            }
        };
        let data_directory = current_dir().unwrap().join("data");
        let user_ini_path = user_directory.join(CONFIG_FILENAME);
        let default_ini_path = data_directory.join(CONFIG_FILENAME);
        Self {
            default_ini_path,
            data_directory,
            user_directory,
            user_ini_path,
        }
    }
}

impl Default for Paths {
    fn default() -> Self {
        Self::new()
    }
}
