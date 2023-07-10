use super::FileOrDirectory;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct ChildPaths {
    pub children: Vec<FileOrDirectory>,
    pub selected: Option<usize>,
}

impl ChildPaths {
    /// Set the child paths and selection.
    pub fn set(
        &mut self,
        directory: &Path,
        extensions: &[String],
        previous_directory: Option<PathBuf>,
    ) {
        // Get the paths.
        let children = self.get_paths_in_directory(directory, extensions);
        // Get the folders. This sets the selection.
        let folders: Vec<&FileOrDirectory> = children.iter().filter(|p| !p.is_file).collect();
        // Set the selection index.
        // Try to select the previous directory.
        if let Some(previous_directory) = previous_directory {
            self.selected = children
                .iter()
                .enumerate()
                .filter(|p| p.1.path == previous_directory)
                .map(|p| p.0)
                .next();
        }
        // Try to select a child.
        if self.selected.is_none() {
            self.selected = if children.is_empty() {
                None
            } else {
                match (folders.is_empty(), children.iter().any(|p| p.is_file)) {
                    (true, false) => None,
                    (true, true) => Some(0),
                    (false, true) => Some(0),
                    (false, false) => Some(folders.len()),
                }
            };
        }
        self.children = children;
    }

    /// Get the child paths of a directory.
    fn get_paths_in_directory(
        &self,
        directory: &Path,
        extensions: &[String],
    ) -> Vec<FileOrDirectory> {
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
        paths
    }
}
