use super::OpenFileType;
use super::FileOrDirectory;
use crate::Paths;
use std::path::PathBuf;

const SOUNDFONT_EXTENSIONS: [&str; 2] = ["sf2", "sf3"];
const SAVE_FILE_EXTENSIONS: [&str; 1] = ["cac"];
const EXPORT_FILE_EXTENSIONS: [&str; 1] = ["wav"];

/// Data for an open-file panel.
#[derive(Default)]
pub struct OpenFile {
    /// Valid file extensions.
    pub extensions: Vec<String>,
    /// The current directory we're in.
    pub directory: PathBuf,
    /// This defines what we're using the panel for.
    pub open_file_type: OpenFileType,
    /// The index of the selected file or folder.
    pub selected: Option<usize>,
    /// The folders and files in the directory.
    pub paths: Vec<FileOrDirectory>,
    /// The filename. This is used for write operations.
    pub filename: Option<String>,
}

impl OpenFile {
    /// Enable the panel.
    pub fn enable(&mut self) {
        // Get the selected child and the children.
        let (selected, paths) = self.get_paths();
        self.selected = selected;
        self.paths = paths;
    }

    /// Enable a panel that can read SoundFonts.
    pub fn soundfont(&mut self, paths: &Paths) {
        // Get the initial working directory.
        if self.open_file_type != OpenFileType::SoundFont {
            self.directory = paths.soundfonts_directory.clone();
        }
        self.open_file_type = OpenFileType::SoundFont;
        self.extensions = SOUNDFONT_EXTENSIONS.iter().map(|s| s.to_string()).collect();
        self.filename = None;
        self.enable();
    }

    /// Enable a panel that can read save files.
    pub fn read_save(&mut self, paths: &Paths) {
        self.filename = None;
        self.enable_as_save(paths);
        self.open_file_type = OpenFileType::ReadSave;
    }

    /// Enable a panel that can write save files.
    pub fn write_save(&mut self, paths: &Paths) {
        self.filename = Some(String::new());
        self.enable_as_save(paths);
        self.open_file_type = OpenFileType::WriteSave;
    }

    pub fn export(&mut self, paths: &Paths) {
        self.filename = Some(String::new());
        if self.open_file_type != OpenFileType::Export {
            self.directory = paths.export_directory.clone();
        }
        self.open_file_type = OpenFileType::Export;
        self.extensions = EXPORT_FILE_EXTENSIONS
            .iter()
            .map(|s| s.to_string())
            .collect();
        self.enable();
    }

    pub fn enable_as_save(&mut self, paths: &Paths) {
        // Get the initial working directory.
        if self.open_file_type != OpenFileType::ReadSave
            && self.open_file_type != OpenFileType::WriteSave
        {
            self.directory = paths.saves_directory.clone();
        }
        self.extensions = SAVE_FILE_EXTENSIONS.iter().map(|s| s.to_string()).collect();
        self.enable();
    }

    /// Get the child paths.
    pub fn get_paths(&self) -> (Option<usize>, Vec<FileOrDirectory>) {
        // Find all valid paths.
        let valid_paths: Vec<PathBuf> = match self.directory.read_dir() {
            Ok(read) => read
                .filter(|e| e.is_ok())
                .map(|e| e.unwrap().path())
                .filter(|p| p.is_file() || p.read_dir().is_ok())
                .collect(),
            Err(_) => vec![],
        };
        // Get the files.
        let mut files: Vec<&PathBuf> = valid_paths
            .iter()
            .filter(|p| {
                p.is_file()
                    && p.extension().is_some()
                    && self
                        .extensions
                        .contains(&p.extension().unwrap().to_str().unwrap().to_string())
            })
            .collect();
        files.sort();
        // Get the directories.
        let mut folders: Vec<&PathBuf> = valid_paths.iter().filter(|p| p.is_dir()).collect();
        folders.sort();

        let mut paths: Vec<FileOrDirectory> =
            folders.iter().map(|f| FileOrDirectory::new(f)).collect();
        paths.append(&mut files.iter().map(|f| FileOrDirectory::new(f)).collect());

        // Set the selection index.
        let selected: Option<usize> = match !paths.is_empty() {
            true => {
                // Start at the first file.
                match (!folders.is_empty(), !files.is_empty()) {
                    (true, true) => Some(folders.len()),
                    (true, false) => Some(0),
                    (false, true) => Some(0),
                    (false, false) => None,
                }
            }
            false => None,
        };
        (selected, paths)
    }
}
