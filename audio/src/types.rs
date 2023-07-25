use crate::exporter::Exporter;
use crate::Command;
use parking_lot::Mutex;
use std::sync::Arc;

/// Type alias for an audio messages.
pub type AudioMessage = (f32, f32);
/// Type alias for a commands message.
pub type CommandsMessage = Vec<Command>;
/// Type alias for an audio buffer.
pub(crate) type AudioBuffer = [Vec<f32>; 2];
/// The exporter.
pub type SharedExporter = Arc<Mutex<Exporter>>;
