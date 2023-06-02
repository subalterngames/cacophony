use open_file::OpenFileType;
use std::path::PathBuf;

/// Commands for the IO struct.
#[derive(Clone)]
pub(crate) enum IOCommand {
    /// Enable the open-file panel.
    EnableOpenFile(OpenFileType),
    /// Disable the open-file panel.
    DisableOpenFile,
    /// Set the save file path.
    SetSavePath(Option<PathBuf>),
    /// Set the export file path.
    SetExportPath(PathBuf),
}

pub(crate) type IOCommands = Option<Vec<IOCommand>>;
