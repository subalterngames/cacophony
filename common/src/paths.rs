use directories::UserDirs;
use std::env::current_dir;
use std::fs::{copy, create_dir_all};
use std::path::{Path, PathBuf};

const CONFIG_FILENAME: &str = "config.ini";

/// Cached file paths.
pub struct Paths {
    /// The path to the default .ini file.
    pub default_ini_path: PathBuf,
    /// The path to the user directory.
    pub user_directory: PathBuf,
    /// The path to the user-defined .ini file. Might not exist.
    pub user_ini_path: PathBuf,
    /// The path to text.csv.
    pub text_path: PathBuf,
    /// The default SoundFont directory.
    pub soundfonts_directory: PathBuf,
    /// The default save file directory.
    pub saves_directory: PathBuf,
    /// The path to the exported audio files.
    pub export_directory: PathBuf,
    /// The path to the splash image.
    pub splash_path: PathBuf,
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
        let text_path = data_directory.join("text.csv");
        // Get or create the default user sub-directories.
        let soundfonts_directory = get_directory("sound_fonts", &user_directory);
        let saves_directory = get_directory("saves", &user_directory);
        let export_directory = get_directory("exports", &user_directory);
        let splash_path = data_directory.join("splash.png");
        Self {
            default_ini_path,
            user_directory,
            user_ini_path,
            text_path,
            soundfonts_directory,
            saves_directory,
            export_directory,
            splash_path
        }
    }

    /// Create the user .ini file by copying the default .ini file.
    pub fn create_user_config(&self) {
        let path = PathBuf::from(&self.user_directory)
            .join(CONFIG_FILENAME)
            .to_str()
            .unwrap()
            .to_string();
        copy(&self.default_ini_path, path).unwrap();
    }
}

impl Default for Paths {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns a directory. Creates the directory if it doesn't exist.
fn get_directory(folder: &str, user_directory: &Path) -> PathBuf {
    let directory = user_directory.join(folder);
    if !directory.exists() {
        create_dir_all(&directory).unwrap();
    }
    directory
}
