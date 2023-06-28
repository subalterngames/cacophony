use common::open_file::OpenFileType;
use std::path::PathBuf;

/// Commands for the IO struct.
#[derive(Clone)]
pub(crate) enum IOCommand {
    /// Enable the open-file panel.
    EnableOpenFile(OpenFileType),
    /// Begin to export.
    Export(PathBuf),
    /// Close the open-file panel.
    CloseOpenFile,
}

pub(crate) type IOCommands = Option<Vec<IOCommand>>;
