use directories::UserDirs;
use std::env::current_dir;
use std::fs::create_dir_all;
use std::path::PathBuf;

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
        let soundfonts_directory = user_directory.join("soundfonts");
        if !soundfonts_directory.exists() {
            create_dir_all(&soundfonts_directory).unwrap();
        }
        let saves_directory = user_directory.join("saves");
        if !saves_directory.exists() {
            create_dir_all(&saves_directory).unwrap();
        }
        Self {
            default_ini_path,
            user_directory,
            user_ini_path,
            text_path,
            soundfonts_directory,
            saves_directory,
        }
    }
}

impl Default for Paths {
    fn default() -> Self {
        Self::new()
    }
}
