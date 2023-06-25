use crate::Command;

/// Type alias for an audio messages.
pub type AudioMessage = (f32, f32);
/// Type alias for a commands message.
pub type CommandsMessage = Vec<Command>;
/// Type alias for an audio buffer.
pub(crate) type AudioBuffer = [Vec<f32>; 2];
