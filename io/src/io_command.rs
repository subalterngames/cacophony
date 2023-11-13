use common::open_file::OpenFileType;

/// Commands for the IO struct.
#[derive(Clone)]
pub(crate) enum IOCommand {
    /// Enable the open-file panel.
    EnableOpenFile(OpenFileType),
    /// Begin to export.
    Export,
    /// Close the open-file panel.
    CloseOpenFile,
    /// Quit the application.
    Quit,
}

pub(crate) type IOCommands = Option<Vec<IOCommand>>;
