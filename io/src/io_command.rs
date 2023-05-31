use crate::open_file::open_file_type::OpenFileType;
use std::path::PathBuf;

/// Commands for the IO struct.
#[derive(Clone)]
pub(crate) enum IOCommand {
    /// Enable the open-file panel.
    EnableOpenFile(OpenFileType),
    /// Disable the open-file panel.
    DisableOpenFile,
    /// Set the save file path.
    SetPath(Option<PathBuf>),
}

pub(crate) type IOCommands = Option<Vec<IOCommand>>;
