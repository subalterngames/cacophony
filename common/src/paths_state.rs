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
    /// When the user wants to import a MIDI file, this is the path.
    pub midis: FileAndDirectory,
    /// The child paths within the current working directory.
    #[serde(skip_serializing, skip_deserializing)]
    pub children: ChildPaths,
    /// The current open-file-type.
    #[serde(skip_serializing, skip_deserializing)]
    pub open_file_type: OpenFileType,
}

impl PathsState {
    pub fn new(paths: &Paths) -> Self {
        let soundfonts = FileAndDirectory::new_directory(paths.soundfonts_directory.clone());
        let saves = FileAndDirectory::new_directory(paths.saves_directory.clone());
        let exports = FileAndDirectory::new_directory(paths.export_directory.clone());
        let midis = FileAndDirectory::new_directory(paths.user_directory.clone());
        Self {
            soundfonts,
            saves,
            exports,
            midis,
            ..Default::default()
        }
    }

    /// Returns the current working directory for the open file type.
    pub fn get_directory(&self) -> &FileOrDirectory {
        match self.open_file_type {
            OpenFileType::Export => &self.exports.directory,
            OpenFileType::ReadSave | OpenFileType::WriteSave => &self.saves.directory,
            OpenFileType::SoundFont => &self.soundfonts.directory,
            OpenFileType::ImportMidi => &self.midis.directory,
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
    pub fn up_directory(&mut self, extension: &Extension) -> bool {
        match self.open_file_type {
            OpenFileType::Export => {
                Self::up_directory_type(&mut self.exports.directory, &mut self.children, extension)
            }
            OpenFileType::ReadSave | OpenFileType::WriteSave => {
                Self::up_directory_type(&mut self.saves.directory, &mut self.children, extension)
            }
            OpenFileType::SoundFont => Self::up_directory_type(
                &mut self.soundfonts.directory,
                &mut self.children,
                extension,
            ),
            OpenFileType::ImportMidi => {
                Self::up_directory_type(&mut self.midis.directory, &mut self.children, extension)
            }
        }
    }

    /// Try to go down a directory.
    pub fn down_directory(&mut self, extension: &Extension) -> bool {
        if self.children.children.is_empty() {
            false
        } else {
            match &self.children.selected {
                Some(selected) => {
                    if self.children.children[*selected].is_file {
                        false
                    } else {
                        let cwd0 = match &self.open_file_type {
                            OpenFileType::Export => self.exports.directory.path.to_path_buf(),
                            OpenFileType::ReadSave | OpenFileType::WriteSave => {
                                self.saves.directory.path.to_path_buf()
                            }
                            OpenFileType::SoundFont => self.soundfonts.directory.path.to_path_buf(),
                            OpenFileType::ImportMidi => self.midis.directory.path.to_path_buf(),
                        };
                        let cwd1 = self.children.children[*selected].path.clone();
                        // Set the children.
                        self.children.set(&cwd1, extension, Some(cwd0));
                        // Set the directory.
                        match &self.open_file_type {
                            OpenFileType::Export => {
                                self.exports.directory = FileOrDirectory::new(&cwd1)
                            }
                            OpenFileType::ReadSave | OpenFileType::WriteSave => {
                                self.saves.directory = FileOrDirectory::new(&cwd1)
                            }
                            OpenFileType::SoundFont => {
                                self.soundfonts.directory = FileOrDirectory::new(&cwd1)
                            }
                            OpenFileType::ImportMidi => {
                                self.midis.directory = FileOrDirectory::new(&cwd1)
                            }
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
            OpenFileType::ImportMidi => self.midis.filename = f,
        }
    }

    /// Returns the path of the directory + filename.
    pub fn get_path(&self) -> PathBuf {
        match &self.open_file_type {
            OpenFileType::Export => self.exports.get_path(),
            OpenFileType::ReadSave | OpenFileType::WriteSave => self.saves.get_path(),
            OpenFileType::SoundFont => self.soundfonts.get_path(),
            OpenFileType::ImportMidi => self.midis.get_path(),
        }
    }

    /// Go up a directory, given an open-file type.
    fn up_directory_type(
        directory: &mut FileOrDirectory,
        children: &mut ChildPaths,
        extension: &Extension,
    ) -> bool {
        match &directory.path.parent() {
            Some(parent) => {
                children.set(parent, extension, Some(directory.path.to_path_buf()));
                *directory = FileOrDirectory::new(parent);
                true
            }
            None => false,
        }
    }
}
