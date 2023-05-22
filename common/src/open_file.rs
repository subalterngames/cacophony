use crate::{State, Paths, Index, PanelType};
use crate::panel_type::DEFAULT_PANELS;
use std::path::{Path, PathBuf};

const SOUNDFONT_EXTENSIONS: [&str; 2] = ["sf2", "sf3"];
const SAVE_FILE_EXTENSIONS: [&str; 1] = ["cac"];

#[derive(Eq, PartialEq, Clone)]
pub enum OpenFileType {
    /// Read a save file.
    ReadSave,
    /// Read a SoundFont.
    SoundFont,
    /// Write a save file.
    WriteSave,
}

/// Cached data for a file or directory.
#[derive(Clone)]
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


/// Data for an open-file panel.
#[derive(Clone)]
pub struct OpenFile {
    /// Valid file extensions.
    extensions: Vec<String>,
    /// The current directory we're in.
    pub directory: PathBuf,
    /// This defines what we're using the panel for.
    pub open_file_type: OpenFileType,
    /// The index of the selected file or folder.
    pub selected: Option<usize>,
    /// The folders and files in the directory.
    pub paths: Vec<FileOrDirectory>,
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

    /// Enable a panel that can read SoundFonts.
    pub fn soundfont(open_file: Option<&OpenFile>, paths: &Paths, state: &mut State) {
        state.open_file = Some(Self::new(
            OpenFileType::SoundFont,
            &SOUNDFONT_EXTENSIONS,
            open_file,
            paths,
        ));
        OpenFile::enable(state);
    }

    /// Enable a panel that can read save files.
    pub fn read_save(open_file: Option<&OpenFile>, paths: &Paths, state: &mut State) {
        state.open_file = Some(Self::new(OpenFileType::ReadSave, &SAVE_FILE_EXTENSIONS, open_file, paths));
        OpenFile::enable(state);
    }

    /// Enable a panel that can write save files.
    pub fn write_save(open_file: Option<&OpenFile>, paths: &Paths, state: &mut State) {
        state.open_file = Some(Self::new(OpenFileType::WriteSave, &SAVE_FILE_EXTENSIONS, open_file, paths));
        OpenFile::enable(state);
    }

    /// Go up to the parent directory.
    pub fn up_directory(&mut self) {
        if let Some(parent) = self.directory.parent() {
            self.directory = parent.to_path_buf();
            let (selected, paths) = OpenFile::get_paths(&self.directory, &self.extensions);
            self.selected = selected;
            self.paths = paths;
        }
    }

    /// Go down to a child directory.
    pub fn down_directory(&mut self) {
        if let Some(selected) = self.selected {
            if !self.paths[selected].is_file {
                self.directory = self.paths[selected].path.clone();
                let (selected, paths) = OpenFile::get_paths(&self.directory, &self.extensions);
                self.selected = selected;
                self.paths = paths;
            }
        }
    }

    /// Scroll up.
    pub fn previous_path(&mut self) {
        if let Some(selected) = self.selected {
            if selected > 0 {
                self.selected = Some(selected - 1);
            }
        }
    }

    /// Scroll down.
    pub fn next_path(&mut self) {
        if let Some(selected) = self.selected {
            if selected < self.paths.len() - 1 {
                self.selected = Some(selected + 1);
            }
        }
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

    /// Returns the number of child files and directories.
    pub fn get_num_children(&self) -> usize {
        self.paths.len()
    }

    fn enable(state: &mut State) {
        // Clear all active panels.
        state.panels.clear();
        // Make this the only active panel.
        state.panels.push(PanelType::OpenFile);
        // Set a new index.
        state.focus = Index::new(0, 1);
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
