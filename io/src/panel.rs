pub(crate) use crate::io_command::IOCommand;
pub(crate) use crate::Snapshot;
pub(crate) use audio::exporter::Exporter;
pub(crate) use audio::{Command, Conn};
pub(crate) use common::{Index, PathsState, State};
pub(crate) use input::{Input, InputEvent};
pub(crate) use text::TtsString;
pub(crate) use text::{Enqueable, Text, TTS};

/// A panel can be updated, and can update the rest of the app state.
pub(crate) trait Panel {
    /// Apply panel-specific updates to the state.
    ///
    /// - `state` The state of the app.
    /// - `conn` The synthesizer-player connection.
    /// - `input` Input events, key presses, etc.
    /// - `tts` Text-to-speech.
    /// - `text` The text.
    /// - `paths_state` Dynamic path data.
    /// - `exporter` Export settings.
    ///
    /// Returns: An `Snapshot`.
    #[allow(clippy::too_many_arguments)]
    fn update(
        &mut self,
        state: &mut State,
        conn: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &mut Text,
        paths_state: &mut PathsState,
        exporter: &mut Exporter,
    ) -> Option<Snapshot>;
}
