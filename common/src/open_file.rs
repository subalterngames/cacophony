use crate::Paths;
use std::path::{Path, PathBuf};

const SOUNDFONT_EXTENSIONS: [&str; 2] = ["sf2", "sf3"];
const SAVE_FILE_EXTENSIONS: [&str; 1] = ["cac"];

#[derive(Eq, PartialEq)]
enum OpenFileType {
    /// Read or write a save file.
    Save,
    /// Read a SoundFont.
    SoundFont,
}

/// Cached data for a file or directory.
struct FileOrDirectory {
    /// The file.
    path: PathBuf,
    /// If true, this is a file.
    is_file: bool,
}

impl FileOrDirectory {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            is_file: path.is_file(),
        }
    }
}

pub struct OpenFile {
    /// Valid file extensions.
    extensions: Vec<String>,
    /// The current directory we're in.
    pub directory: PathBuf,
    /// This defines what we're using the panel for.
    open_file_type: OpenFileType,
    /// The index of the selected file or folder.
    pub selected: Option<usize>,
    /// The folders and files in the directory.
    paths: Vec<FileOrDirectory>,
}

impl OpenFile {
    fn new(
        open_file_type: OpenFileType,
        extensions: &[&str],
        open_file: Option<&OpenFile>,
        paths: &Paths,
    ) -> Self {
        // Get the initial working directory.
        let directory = match open_file {
            Some(open_file) => {
                // If there is an existing open-file and its type is my type, then use it.
                if open_file.open_file_type == open_file_type {
                    open_file.directory.clone()
                } else {
                    paths.soundfonts_directory.clone()
                }
            }
            None => paths.soundfonts_directory.clone(),
        };
        // Get the extensions.
        let extensions: Vec<String> = extensions.iter().map(|s| s.to_string()).collect();
        // Get the selected child and the children.
        let (selected, paths) = OpenFile::get_paths(&directory, &extensions);
        // Return me.
        Self {
            extensions,
            directory,
            open_file_type,
            selected,
            paths,
        }
    }

    /// Returns an `OpenFile` that can read SoundFonts.
    pub fn soundfont(open_file: Option<&OpenFile>, paths: &Paths) -> Self {
        Self::new(
            OpenFileType::SoundFont,
            &SOUNDFONT_EXTENSIONS,
            open_file,
            paths,
        )
    }

    /// Returns an `OpenFile` that can read rwrite save files.
    pub fn save(open_file: Option<&OpenFile>, paths: &Paths) -> Self {
        Self::new(OpenFileType::Save, &SAVE_FILE_EXTENSIONS, open_file, paths)
    }

    /// Set a new working directory.
    pub fn set_directory(&mut self, directory: &Path) {
        self.directory = directory.to_path_buf();
        let (selected, paths) = OpenFile::get_paths(&self.directory, &self.extensions);
        self.selected = selected;
        self.paths = paths;
    }

    /// Returns the child directories as strings.
    pub fn get_child_directories(&self) -> Vec<String> {
        self.paths
            .iter()
            .filter(|p| !p.is_file)
            .map(|p| p.path.to_str())
            .filter(|p| p.is_some())
            .flatten()
            .map(|p| p.to_string())
            .collect()
    }

    /// Returns the child directories as strings.
    pub fn get_child_files(&self) -> Vec<String> {
        self.paths
            .iter()
            .filter(|p| p.is_file)
            .map(|p| p.path.to_str())
            .filter(|p| p.is_some())
            .flatten()
            .map(|p| p.to_string())
            .collect()
    }

    /// Get the child paths.
    fn get_paths(directory: &Path, extensions: &[String]) -> (Option<usize>, Vec<FileOrDirectory>) {
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
                    && extensions.contains(&p.extension().unwrap().to_str().unwrap().to_string())
            })
            .collect();
        files.sort();
        // Get the directories.
        let mut folders: Vec<&PathBuf> = valid_paths.iter().filter(|p| p.is_dir()).collect();
        folders.sort();

        let mut paths: Vec<FileOrDirectory> =
            folders.iter().map(|f| FileOrDirectory::new(f)).collect();
        paths.append(&mut files.iter().map(|f| FileOrDirectory::new(f)).collect());

        // Set the selection index and the visible range.
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
