pub(crate) use crate::io_command::IOCommand;
pub(crate) use crate::{get_tooltip_with_values, UndoRedoState};
pub(crate) use audio::{Command, Conn};
pub(crate) use common::music_panel_field::MusicPanelField;
pub(crate) use common::{Index, State};
pub(crate) use input::{Input, InputEvent};
pub(crate) use text::{Text, TTS};

/// I guess this is how we do function aliases in Rust.
pub(crate) trait Panel {
    /// Apply panel-specific updates to the state.
    ///
    /// - `conn` The synthesizer-player connection.
    /// - `state` The current `State`.
    /// - `input` Input events, key presses, etc.
    /// - `tts` Text-to-speech.
    ///
    /// Returns: An `UndoRedoState`, implying that we modified `state`, or None.
    fn update(
        &mut self,
        state: &mut State,
        conn: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &Text,
    ) -> Option<UndoRedoState>;
}
