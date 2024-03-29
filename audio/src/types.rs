use crate::export::ExportState;
use crate::midi_event_queue::MidiEventQueue;
use crate::play_state::PlayState;
use crate::Command;
use oxisynth::Synth;
use parking_lot::Mutex;
use std::sync::Arc;

/// Type alias for an audio messages.
pub type AudioMessage = (f32, f32);
/// Type alias for a commands message.
pub type CommandsMessage = Vec<Command>;
/// Type alias for an audio buffer.
pub(crate) type AudioBuffer = [Vec<f32>; 2];
pub(crate) type SharedSynth = Arc<Mutex<Synth>>;
pub type SharedExportState = Arc<Mutex<ExportState>>;
pub(crate) type SharedMidiEventQueue = Arc<Mutex<MidiEventQueue>>;
pub type SharedPlayState = Arc<Mutex<PlayState>>;
pub(crate) type SharedSample = Arc<Mutex<AudioMessage>>;
