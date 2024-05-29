use super::{Extension, FileOrDirectory};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// A collection of child paths and a selected child path.
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct ChildPaths {
    /// The child paths from the parent directory.
    pub children: Vec<FileOrDirectory>,
    /// The index of the selected child in `children`, if any.
    pub selected: Option<usize>,
}

impl ChildPaths {
    /// Set the child paths and selection.
    ///
    /// - `directory` The current parent directory.
    /// - `extension` The extension of valid files.
    /// - `previous_directory` This is used to set the selection to the directory we just moved up from.
    pub fn set(
        &mut self,
        directory: &Path,
        extension: &Extension,
        previous_directory: Option<PathBuf>,
    ) {
        // Get the paths.
        let children = self.get_paths_in_directory(directory, extension);
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
                    (false, false) => Some(folders.len() - 1),
                    (_, _) => Some(0),
                }
            };
        }
        self.children = children;
    }

    /// Get the child paths of a directory.
    ///
    /// - `directory` The current parent directory.
    /// - `extension` The extension of valid files.
    fn get_paths_in_directory(
        &self,
        directory: &Path,
        extension: &Extension,
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
                    && extension.to_str(false)
                        == p.extension().unwrap().to_str().unwrap().to_lowercase()
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


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::open_file::Extension;
    use super::ChildPaths;

    #[test]
    fn test_child_paths() {
        // SoundFont.
        let sf_directory = PathBuf::from("../data");
        assert!(sf_directory.exists());
        let mut child_paths = ChildPaths::default();
        child_paths.set(&sf_directory, &Extension::Sf2, None);
        assert_eq!(child_paths.children.len(), 1);
        let f = &child_paths.children[0];
        assert!(f.is_file);
        assert_eq!(f.stem, "CT1MBGMRSV1.06.sf2");
        assert!(child_paths.selected.is_some());
        assert!(child_paths.selected.unwrap() == 0);
        // Anything else.
        
    }
}