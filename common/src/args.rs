use crate::get_default_data_folder;
use clap::Parser;
use std::path::PathBuf;

/// Command-line arguments.
#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Open the project from disk.
    #[arg(value_name = "FILE")]
    pub file: Option<PathBuf>,
    /// Directory where Cacophony data files reside.
    ///
    /// Uses './data' if not set.
    #[arg(short, long, value_name = "DIR", env = "CACOPHONY_DATA_DIR", default_value = get_default_data_folder().into_os_string())]
    pub data_directory: PathBuf,
    /// Make the window fullscreen.
    ///
    /// Uses 'fullscreen' under '[RENDER]' in 'config.ini' if not set.
    ///
    /// Applied after displaying the splash-screen
    #[arg(short, long, env = "CACOPHONY_FULLSCREEN")]
    pub fullscreen: bool,
    /// A path to a file of events that will be executed sequentially when the simulation starts.
    ///
    /// This is meant to be used for debugging.
    #[arg(short, long)]
    pub events: Option<PathBuf>,
}
