pub(crate) use crate::io_command::IOCommand;
pub(crate) use crate::popup::Popup;
pub(crate) use crate::Snapshot;
pub(crate) use audio::{Command, Conn};
pub(crate) use common::{Index, PathsState, State};
pub(crate) use input::{Input, InputEvent};
pub(crate) use text::{Enqueable, Text, Tooltips, TtsString, TTS};

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
    ///
    /// Returns: An `Snapshot`.
    fn update(
        &mut self,
        state: &mut State,
        conn: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &Text,
        paths_state: &mut PathsState,
    ) -> Option<Snapshot>;

    /// Apply panel-specific updates to the state if alphanumeric input is enabled.
    ///
    /// - `state` The state of the app.
    /// - `input` Input events, key presses, etc.
    /// - `conn` The audio connection.
    ///
    /// Returns: An `Snapshot` and true if something (potentially not included in the snaphot) updated.
    fn update_abc123(
        &mut self,
        state: &mut State,
        input: &Input,
        conn: &mut Conn,
    ) -> (Option<Snapshot>, bool);

    /// Do something when alphanumeric input is disabled.
    ///
    /// - `state` The state of the app.
    /// - `conn` The audio connection.
    fn on_disable_abc123(&mut self, state: &mut State, conn: &mut Conn);

    /// If true, allow the user to toggle alphanumeric input.
    ///
    /// - `state` The state.
    /// - `conn` The audio connection.
    fn allow_alphanumeric_input(&self, state: &State, conn: &Conn) -> bool;

    /// Returns true if we can play music.
    fn allow_play_music(&self) -> bool;
}
