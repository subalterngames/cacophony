use crate::open_file::*;
use crate::panel::*;
use std::path::{Path, PathBuf};

/// Data for an open-file panel.
pub struct OpenFilePanel {
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