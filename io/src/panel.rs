use crate::UndoState;
use audio::Conn;
use common::State;
use input::Input;
use text::{Text, TTS};

/// I guess this is how we do function aliases in Rust.
pub(crate) trait Panel {
    /// Apply panel-specific updates to the state.
    ///
    /// - `conn` The synthesizer-player connection.
    /// - `state` The current `State`.
    /// - `input` Input events, key presses, etc.
    /// - `tts` Text-to-speech.
    ///
    /// Returns: A new `UndoState` or None.
    fn update(
        &mut self,
        state: &State,
        conn: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &Text,
    ) -> Option<UndoState>;
}
