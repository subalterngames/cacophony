use crate::Paths;
mod file_or_directory;
mod open_file_type;
pub use file_or_directory::FileOrDirectory;
pub use open_file_type::OpenFileType;
use std::path::{Path, PathBuf};

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
    pub open_file_type: Option<OpenFileType>,
    /// The index of the selected file or folder.
    pub selected: Option<usize>,
    /// The folders and files in the directory.
    pub paths: Vec<FileOrDirectory>,
    /// If true, this was enabled on this frame.
    pub enabled: bool,
}

impl OpenFile {
    /// Enable the panel.
    pub fn enable(&mut self) {
        // Get the selected child and the children.
        let (selected, paths) = self.get_paths();
        self.selected = selected;
        self.paths = paths;
        self.enabled = true;
    }

    /// Enable a panel that can read SoundFonts.
    pub fn soundfont(&mut self, paths: &Paths) {
        let set_directory = match &self.open_file_type {
            Some(oft) => *oft != OpenFileType::SoundFont,
            None => true,
        };
        self.open_file_type = Some(OpenFileType::SoundFont);
        // Get the initial working directory.
        if set_directory {
            self.directory = paths.soundfonts_directory.clone();
        }
        self.extensions = SOUNDFONT_EXTENSIONS.iter().map(|s| s.to_string()).collect();
        self.enable();
    }

    /// Enable a panel that can read save files.
    pub fn read_save(&mut self, paths: &Paths) {
        self.enable_as_save(paths);
        self.open_file_type = Some(OpenFileType::ReadSave);
    }

    /// Enable a panel that can write save files.
    pub fn write_save(&mut self, paths: &Paths) {
        self.enable_as_save(paths);
        self.open_file_type = Some(OpenFileType::WriteSave);
    }

    pub fn export(&mut self, paths: &Paths) {
        let set_directory = match &self.open_file_type {
            Some(oft) => *oft != OpenFileType::Export,
            None => true,
        };
        if set_directory {
            self.directory = paths.export_directory.clone();
        }
        self.open_file_type = Some(OpenFileType::Export);
        self.extensions = EXPORT_FILE_EXTENSIONS
            .iter()
            .map(|s| s.to_string())
            .collect();
        self.enable();
    }

    pub fn enable_as_save(&mut self, paths: &Paths) {
        let set_directory = match &self.open_file_type {
            Some(oft) => *oft != OpenFileType::ReadSave && *oft != OpenFileType::WriteSave,
            None => true,
        };
        // Get the initial working directory.
        if set_directory {
            self.directory = paths.saves_directory.clone();
        }
        self.extensions = SAVE_FILE_EXTENSIONS.iter().map(|s| s.to_string()).collect();
        self.enable();
    }

    /// Go up to the parent directory.
    pub fn up_directory(&mut self) {
        // There is a parent directory.
        if let Some(parent) = &self.directory.parent() {
            let paths = self.get_paths_in_directory(parent);
            let selected = paths
                .iter()
                .enumerate()
                .filter(|p| p.1.path == self.directory)
                .map(|p| p.0)
                .next();
            self.paths = paths;
            self.selected = selected;
            self.directory = parent.to_path_buf();
        }
    }

    /// Get the child paths.
    pub fn get_paths(&self) -> (Option<usize>, Vec<FileOrDirectory>) {
        let paths = self.get_paths_in_directory(&self.directory);
        let folders: Vec<&FileOrDirectory> = paths.iter().filter(|p| !p.is_file).collect();
        // Set the selection index.
        let selected: Option<usize> = match !paths.is_empty() {
            true => {
                // Start at the first file.
                match (folders.is_empty(), !paths.iter().any(|p| p.is_file)) {
                    (true, true) => None,
                    (true, false) => Some(0),
                    (false, true) => Some(0),
                    (false, false) => Some(folders.len()),
                }
            }
            false => None,
        };
        (selected, paths)
    }

    /// Get the child paths of a directory.
    fn get_paths_in_directory(&self, directory: &Path) -> Vec<FileOrDirectory> {
        // Find all valid paths.
        let valid_paths: Vec<PathBuf> = match directory.read_dir() {
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
        paths
    }
}
