use crate::open_file::open_file_type::OpenFileType;

/// Commands for the IO struct.
pub(crate) enum IOCommand {
    /// Enable the open-file panel.
    EnableOpenFile(OpenFileType),
    /// Disable the open-file panel.
    DisableOpenFile,
}

pub(crate) type IOCommands = Option<Vec<IOCommand>>;
