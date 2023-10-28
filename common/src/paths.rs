use directories::UserDirs;
use std::env::{current_dir, current_exe};
use std::fs::{copy, create_dir_all};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const CONFIG_FILENAME: &str = "config.ini";

/// Global reference to paths.
static PATHS: OnceLock<Paths> = OnceLock::new();

/// Cached file paths. Unlike `PathsState`, this is meant to only include static data.
#[derive(Debug)]
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
    /// The path to the default soundfont in data/
    pub default_soundfont_path: PathBuf,
    /// The path to the data/ directory itself.
    pub data_directory: PathBuf,
}

impl Paths {
    /// Setup the paths, needs to be be called at least once.
    pub fn init(data_directory_from_cli: &Path) {
        let data_directory = get_data_directory(data_directory_from_cli);
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
        let user_ini_path = user_directory.join(CONFIG_FILENAME);
        let default_ini_path = data_directory.join(CONFIG_FILENAME);
        let text_path = data_directory.join("text.csv");
        // Get or create the default user sub-directories.
        let soundfonts_directory = get_directory("soundfonts", &user_directory);
        let saves_directory = get_directory("saves", &user_directory);
        let export_directory = get_directory("exports", &user_directory);
        let splash_path = data_directory.join("splash.png");
        let default_soundfont_path = data_directory.join("CT1MBGMRSV1.06.sf2");
        PATHS
            .set(Self {
                default_ini_path,
                user_directory,
                user_ini_path,
                text_path,
                soundfonts_directory,
                saves_directory,
                export_directory,
                splash_path,
                default_soundfont_path,
                data_directory,
            })
            .unwrap();
    }

    /// Get a reference to the paths, panics when not initialized.
    pub fn get() -> &'static Self {
        PATHS.get().expect("Paths need to be initialzed first")
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

/// Returns the path to the data directory.
pub fn get_data_directory(data_directory: &Path) -> PathBuf {
    // Try to get the directory that's passed first.
    if data_directory.exists() {
        data_directory.to_path_buf()
    }
    // Maybe we're in a .app bundle.
    else if cfg!(target_os = "macos") {
        let data_directory = current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
            .join("../Resources/data");
        if data_directory.exists() {
            data_directory
        } else {
            panic!("Failed to get data directory: {:?}", data_directory)
        }
    } else {
        panic!("Failed to get data directory: {:?}", data_directory)
    }
}

/// Default directory for looking at the 'data/' folder.
pub fn default_data_folder() -> PathBuf {
    current_dir().unwrap().join("data")
}

/// Returns a directory. Creates the directory if it doesn't exist.
fn get_directory(folder: &str, user_directory: &Path) -> PathBuf {
    let directory = user_directory.join(folder);
    if !directory.exists() {
        create_dir_all(&directory).unwrap();
    }
    directory
}
