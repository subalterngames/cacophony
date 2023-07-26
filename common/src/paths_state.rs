use crate::open_file::*;
use crate::{Index, Paths};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// User-defined, save-file-specific, path data.
/// These paths aren't stored in `State` because:
///
/// 1. Changes to these paths should never go on the undo stack.
/// 2. This struct can be arbitrarily complex, so it shouldn't go on the undo stack.
#[derive(Deserialize, Serialize, Clone, Default)]
pub struct PathsState {
    /// When the SoundFont open-file panel is enabled, it will default to this directory.
    pub soundfonts: FileAndDirectory,
    /// When the user wants to save a file, it will be automatically written here unless they do a save-as.
    pub saves: FileAndDirectory,
    /// When the user wants to export a file, it will be exported to this path.
    pub exports: FileAndDirectory,
    /// The child paths within the current working directory.
    #[serde(skip_serializing, skip_deserializing)]
    pub children: ChildPaths,
    /// The current open-file-type.
    #[serde(skip_serializing, skip_deserializing)]
    pub open_file_type: OpenFileType,
    /// The Windows drives on this system.
    #[serde(skip_serializing, skip_deserializing)]
    windows_drives: Vec<FileOrDirectory>,
}

impl PathsState {
    pub fn new(paths: &Paths) -> Self {
        let soundfonts = FileAndDirectory::new_directory(paths.soundfonts_directory.clone());
        let saves = FileAndDirectory::new_directory(paths.saves_directory.clone());
        let exports = FileAndDirectory::new_directory(paths.export_directory.clone());
        let windows_drives = Self::get_windows_drives();
        Self {
            soundfonts,
            saves,
            exports,
            windows_drives,
            ..Default::default()
        }
    }

    /// Returns the current working directory for the open file type.
    pub fn get_directory(&self) -> &PathBuf {
        match self.open_file_type {
            OpenFileType::Export => &self.exports.directory,
            OpenFileType::ReadSave | OpenFileType::WriteSave => &self.saves.directory,
            OpenFileType::SoundFont => &self.soundfonts.directory,
        }
    }

    /// Returns a string of a given open-file-type's path's filename.
    pub fn get_filename(&self) -> Option<String> {
        match self.open_file_type {
            OpenFileType::Export => Some(self.exports.get_filename()),
            OpenFileType::WriteSave => Some(self.saves.get_filename()),
            _ => None,
        }
    }

    /// Try to go up a directory.
    pub fn up_directory(&mut self, extensions: &[String]) -> bool {
        match self.open_file_type {
            OpenFileType::Export => match &self.exports.directory.parent() {
                Some(parent) => {
                    self.children.set(
                        parent,
                        extensions,
                        Some(self.exports.directory.to_path_buf()),
                    );
                    self.exports.directory = parent.to_path_buf();
                    true
                }
                None => Self::set_children_to_windows_drives(
                    &mut self.exports.directory,
                    &mut self.children,
                    &self.windows_drives,
                ),
            },
            OpenFileType::ReadSave | OpenFileType::WriteSave => {
                match &self.saves.directory.parent() {
                    Some(parent) => {
                        self.children.set(
                            parent,
                            extensions,
                            Some(self.saves.directory.to_path_buf()),
                        );
                        self.saves.directory = parent.to_path_buf();
                        true
                    }
                    None => Self::set_children_to_windows_drives(
                        &mut self.saves.directory,
                        &mut self.children,
                        &self.windows_drives,
                    ),
                }
            }
            OpenFileType::SoundFont => match &self.soundfonts.directory.parent() {
                Some(parent) => {
                    self.children.set(
                        parent,
                        extensions,
                        Some(self.soundfonts.directory.to_path_buf()),
                    );
                    self.soundfonts.directory = parent.to_path_buf();
                    true
                }
                None => Self::set_children_to_windows_drives(
                    &mut self.soundfonts.directory,
                    &mut self.children,
                    &self.windows_drives,
                ),
            },
        }
    }

    /// Try to go down a directory.
    pub fn down_directory(&mut self, extensions: &[String]) -> bool {
        if self.children.children.is_empty() {
            false
        } else {
            match &self.children.selected {
                Some(selected) => {
                    if self.children.children[*selected].is_file {
                        false
                    } else {
                        let cwd0 = match &self.open_file_type {
                            OpenFileType::Export => self.exports.directory.to_path_buf(),
                            OpenFileType::ReadSave | OpenFileType::WriteSave => {
                                self.saves.directory.to_path_buf()
                            }
                            OpenFileType::SoundFont => self.soundfonts.directory.to_path_buf(),
                        };
                        let cwd1 = self.children.children[*selected].path.clone();
                        // Set the children.
                        self.children.set(&cwd1, extensions, Some(cwd0));
                        // Set the directory.
                        match &self.open_file_type {
                            OpenFileType::Export => self.exports.directory = cwd1,
                            OpenFileType::ReadSave | OpenFileType::WriteSave => {
                                self.saves.directory = cwd1
                            }
                            OpenFileType::SoundFont => self.soundfonts.directory = cwd1,
                        }
                        true
                    }
                }
                None => false,
            }
        }
    }

    /// Try to scroll through the children.
    pub fn scroll(&mut self, up: bool) -> bool {
        if self.children.children.is_empty() {
            false
        } else if let Some(selected) = &mut self.children.selected {
            let mut index = Index::new(*selected, self.children.children.len());
            if index.increment_no_loop(!up) {
                *selected = index.get();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Set a filename.
    pub fn set_filename(&mut self, filename: &str) {
        let f = if filename.is_empty() {
            None
        } else {
            Some(filename.to_string())
        };
        match &self.open_file_type {
            OpenFileType::Export => self.exports.filename = f,
            OpenFileType::ReadSave | OpenFileType::WriteSave => self.saves.filename = f,
            OpenFileType::SoundFont => (),
        }
    }

    /// Returns the path of the directory + filename.
    pub fn get_path(&self) -> PathBuf {
        match &self.open_file_type {
            OpenFileType::Export => self.exports.get_path(),
            OpenFileType::ReadSave | OpenFileType::WriteSave => self.saves.get_path(),
            OpenFileType::SoundFont => self.soundfonts.get_path(),
        }
    }

    fn set_children_to_windows_drives(
        path: &mut PathBuf,
        children: &mut ChildPaths,
        windows_drives: &Vec<FileOrDirectory>,
    ) -> bool {
        if cfg!(windows) {
            if windows_drives.iter().any(|p| p.path == *path) {
                // Manually set the children.
                children.children = windows_drives.clone();
                children.selected = Some(0);
                *path = windows_drives[0].path.to_path_buf();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Returns the roots of all valid Windows drives.
    fn get_windows_drives() -> Vec<FileOrDirectory> {
        if cfg!(windows) {
            const LETTERS: [&str; 26] = [
                "A:\\", "B:\\", "C:\\", "D:\\", "E:\\", "F:\\", "G:\\", "H:\\", "I:\\", "J:\\",
                "K:\\", "L:\\", "M:\\", "N:\\", "O:\\", "P:\\", "Q:\\", "R:\\", "S:\\", "T:\\",
                "U:\\", "V:\\", "W:\\", "X:\\", "Y:\\", "Z:\\",
            ];
            let mut drives = vec![];
            for letter in LETTERS {
                let drive = PathBuf::from(letter);
                if drive.exists() {
                    drives.push(FileOrDirectory {
                        path: drive,
                        is_file: false,
                    });
                }
            }
            drives
        } else {
            vec![]
        }
    }
}
