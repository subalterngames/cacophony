pub(crate) use crate::io_command::IOCommand;
pub(crate) use crate::popup::Popup;
pub(crate) use crate::Snapshot;
pub(crate) use audio::SharedExporter;
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
        exporter: &mut SharedExporter,
    ) -> Option<Snapshot>;

    /// Apply panel-specific updates to the state if alphanumeric input is enabled.
    ///
    /// - `state` The state of the app.
    /// - `input` Input events, key presses, etc.
    /// - `exporter` Export settings.
    ///
    /// Returns: An `Snapshot` and true if something (potentially not included in the snaphot) updated.
    fn update_abc123(
        &mut self,
        state: &mut State,
        input: &Input,
        exporter: &mut SharedExporter,
    ) -> (Option<Snapshot>, bool);

    /// Do something when alphanumeric input is disabled.
    ///
    /// - `state` The state of the app.
    /// - `exporter` Export settings.
    fn on_disable_abc123(&mut self, state: &mut State, exporter: &mut SharedExporter);

    /// If true, allow the user to toggle alphanumeric input.
    ///
    /// - `state` The state.
    /// - `exporter` Export settings.
    fn allow_alphanumeric_input(&self, state: &State, exporter: &SharedExporter) -> bool;

    /// Returns true if we can play music.
    fn allow_play_music(&self) -> bool;
}
