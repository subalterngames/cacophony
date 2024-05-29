use crate::{Index, U64orF32, MAX_VOLUME, PPQ_U};
use serde::{Deserialize, Serialize};

/// Booleans and numerical values describing the input state.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InputState {
    /// If true, we will accept musical input.
    pub armed: bool,
    /// If true, we're inputting an alphanumeric string and we should ignore certain key bindings.
    pub alphanumeric_input: bool,
    /// The volume for all new notes.
    pub volume: Index<u8>,
    /// If true, we'll use the volume value.
    pub use_volume: bool,
    /// The input beat in PPQ.
    pub beat: U64orF32,
    /// If true, music is playing or exporting.
    #[serde(skip)]
    pub is_playing: bool,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            armed: false,
            alphanumeric_input: false,
            volume: Index::new(MAX_VOLUME, MAX_VOLUME + 1),
            use_volume: true,
            beat: U64orF32::from(PPQ_U),
            is_playing: false,
        }
    }
}
