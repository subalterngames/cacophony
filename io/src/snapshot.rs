use crate::{IOCommand, IOCommands, State};
use audio::CommandsMessage;

/// A snapshot of a state delta.
#[derive(Default)]
pub(crate) struct Snapshot {
    /// The state before changes were applied.
    pub(crate) from_state: Option<State>,
    /// The state after changes were applied.
    to_state: Option<State>,
    /// Commands that need to be sent to revert to the state before changes were applied.
    pub(crate) from_commands: Option<CommandsMessage>,
    /// Commands  that need to be sent to apply changes.
    to_commands: Option<CommandsMessage>,
    /// A list of commands to send to the `IO` state.
    pub(crate) io_commands: IOCommands,
}

impl Snapshot {
    /// Returns a snapshot of the delta between two states.
    pub fn from_states(from_state: State, to_state: &mut State) -> Self {
        Self {
            from_state: Some(from_state),
            to_state: Some(to_state.clone()),
            ..Default::default()
        }
    }

    /// Returns a snapshot of the delta implied by `to_commands`.
    /// `from_commands` are the commands required to revert the audio conn state.
    pub fn from_commands(from_commands: CommandsMessage, to_commands: &CommandsMessage) -> Self {
        Self {
            from_commands: Some(from_commands),
            to_commands: Some(to_commands.clone()),
            ..Default::default()
        }
    }

    /// Returns a snapshot of the delta between two states as well as two lists of commands.
    pub fn from_states_and_commands(
        from_state: State,
        to_state: &mut State,
        from_commands: CommandsMessage,
        to_commands: &CommandsMessage,
    ) -> Self {
        Self {
            from_state: Some(from_state),
            to_state: Some(to_state.clone()),
            from_commands: Some(from_commands),
            to_commands: Some(to_commands.clone()),
            io_commands: None,
        }
    }

    /// Returns a snapshot of the delta implied by `to_commands`.
    /// `from_commands` are the commands required to revert the audio conn state.
    /// Include some IOCommands for spice.
    pub fn from_commands_and_io_commands(
        from_commands: CommandsMessage,
        to_commands: &CommandsMessage,
        io_commands: Vec<IOCommand>,
    ) -> Self {
        Self {
            from_commands: Some(from_commands),
            to_commands: Some(to_commands.clone()),
            io_commands: Some(io_commands),
            ..Default::default()
        }
    }

    /// Returns a snapshot that just contains IOCommands.
    pub fn from_io_commands(io_commands: Vec<IOCommand>) -> Self {
        Self {
            io_commands: Some(io_commands),
            ..Default::default()
        }
    }

    /// Returns a snapshot that flips the from/to of `snapshot`. This is used for undo/redo.
    pub fn flip(snapshot: &Snapshot) -> Self {
        Self {
            from_state: snapshot.to_state.clone(),
            to_state: snapshot.from_state.clone(),
            from_commands: snapshot.to_commands.clone(),
            to_commands: snapshot.from_commands.clone(),
            io_commands: None,
        }
    }
}
