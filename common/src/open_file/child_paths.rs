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
                .filter(|p| {
                    p.1.stem
                        == previous_directory
                            .components()
                            .last()
                            .unwrap()
                            .as_os_str()
                            .to_str()
                            .unwrap()
                })
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
    use std::{fs::canonicalize, path::PathBuf};

    use super::ChildPaths;
    use crate::open_file::{Extension, FileOrDirectory};

    #[test]
    fn test_sf2_child_paths() {
        let sf_directory = PathBuf::from("../data");
        assert!(sf_directory.exists());
        let mut child_paths = ChildPaths::default();
        child_paths.set(&sf_directory, &Extension::Sf2, None);
        assert_eq!(child_paths.children.len(), 1);
        let f = &child_paths.children[0];
        assert!(f.is_file);
        assert_eq!(f.stem, "CT1MBGMRSV1.06.sf2");
        assert!(child_paths.selected.is_some());
        assert_eq!(child_paths.selected.unwrap(), 0);
        // There shouldn't be any save files.
        child_paths.set(&sf_directory, &Extension::Cac, None);
        assert!(child_paths.children.is_empty());
        assert!(child_paths.selected.is_some());
        assert_eq!(child_paths.selected.unwrap(), 0);
        let parent_directory = canonicalize(sf_directory.parent().unwrap()).unwrap();
        assert_eq!(
            parent_directory
                .components()
                .last()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap(),
            "cacophony"
        );
        // Set a different directory.
        child_paths.set(&parent_directory, &Extension::Sf2, None);
        assert!(!child_paths.children.is_empty());
        assert!(child_paths
            .children
            .iter()
            .filter(|c| c.is_file)
            .collect::<Vec<&FileOrDirectory>>()
            .is_empty());
        // Ignore any folders that have names beginning with a period because they won't all appear in the GitHub workflow.
        child_paths
            .children
            .retain(|f| match f.stem.chars().next() {
                Some(ch) => ch != '.',
                None => false,
            });
        assert!(child_paths.children.len() > 0);
        assert!(child_paths.selected.is_some());
        assert_eq!(child_paths.selected.unwrap(), 0);
        // Go "up" a directory.
        child_paths.set(
            &parent_directory,
            &Extension::Sf2,
            Some(sf_directory.clone()),
        );
        // Test the selection.
        assert_eq!(
            child_paths.children[child_paths.selected.unwrap()].stem,
            "data"
        );
    }

    #[test]
    fn test_cac_child_paths() {
        let cac_directory = PathBuf::from("../test_files/child_paths");
        assert!(cac_directory.exists());
        let mut child_paths = ChildPaths::default();
        child_paths.set(&cac_directory, &Extension::Cac, None);
        assert_eq!(child_paths.children.len(), 3);
        test_cac_file(&child_paths, child_paths.selected.unwrap(), "test_0.cac");
        test_cac_file(&child_paths, 1, "test_1.cac");
        test_cac_file(&child_paths, 2, "test_2.CAC");
    }

    fn test_cac_file(child_paths: &ChildPaths, index: usize, filename: &str) {
        let f = &child_paths.children[index];
        assert!(f.is_file);
        assert_eq!(f.stem, filename);
    }
}
