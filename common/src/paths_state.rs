use crate::Paths;
use crate::open_file::*;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// User-defined, save-file-specific, path data.
/// These paths aren't stored in `State` because:
///
/// 1. Changes to these paths should never go on the undo stack.
/// 2. This struct can be arbitrarily complex, so it shouldn't go on the undo stack.
#[derive(Deserialize, Serialize, Clone, Default)]
pub struct PathsState {
    /// When the SoundFont open-file panel is enabled, it will default to this directory.
    pub soundfonts_directory: PathBuf,
    /// When the user wants to save a file, it will be automatically written here unless they do a save-as.
    pub save_path: Option<PathBuf>,
    /// When the user wants to export a file, it will be exported to this path.
    pub export_path: Option<PathBuf>,
    /// The child paths within the current working directory.
    pub children: Option<Vec<FileOrDirectory>>,
    /// The index of the selected file.
    pub selected: Option<usize>,
}

impl PathsState {
    pub fn new(paths: &Paths) -> Self {
        Self { soundfonts_directory: paths.soundfonts_directory.clone(), ..Default::default() }
    }

    /// Returns the current working directory for the open file type.
    pub fn get_directory(&self, open_file_type: &OpenFileType, paths: &Paths) -> PathBuf {
        match open_file_type {
            OpenFileType::Export => PathsState::get_parent_directory(&self.export_path, &paths.export_directory),
            OpenFileType::ReadSave | OpenFileType::WriteSave => PathsState::get_parent_directory(&self.save_path, &paths.saves_directory),
            OpenFileType::SoundFont => self.soundfonts_directory.clone()
        }
    }

    /// Returns a string of a given open-file-type's path's filename.
    pub fn get_filename(&self, open_file_type: &OpenFileType) -> Option<String> {
        match open_file_type {
            OpenFileType::Export => PathsState::get_filename_from_path(&self.export_path),
            OpenFileType::ReadSave | OpenFileType::WriteSave =>PathsState::get_filename_from_path(&self.save_path),
            OpenFileType::SoundFont => None
        }
    }

    pub fn set_path(&mut self, filename: &str, open_file_type: &OpenFileType, paths: &Paths) {
        match open_file_type {
            OpenFileType::Export => self.export_path = Some(self.get_directory(open_file_type, paths).join(filename)),
            OpenFileType::ReadSave | OpenFileType::WriteSave => self.save_path = Some(self.get_directory(open_file_type, paths).join(filename)),
            OpenFileType::SoundFont => ()
        }
    }

    fn get_parent_directory(path: &Option<PathBuf>, default_directory: &Path) -> PathBuf {
        match path {
            Some(path) => match path.parent() {
                Some(parent) => parent.to_path_buf(),
                None => default_directory.to_path_buf()
            }
            None => default_directory.to_path_buf()
        }
    }

    fn get_filename_from_path(path: &Option<PathBuf>) -> Option<String> {
        match path {
            Some(path) => match path.file_name() {
                Some(filename) => match filename.to_str() {
                    Some(string) => Some(String::from(string)),
                    None => None
                }
                None => None
            }
            None => None
        }
    }
}
