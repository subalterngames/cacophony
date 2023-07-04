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
    /// Sets a value and returns a snapshot of the delta between two states.
    ///
    /// - `f` A function that accepts a `State` parameter and returns a mutable reference value of type T.
    /// - `value` The new value of `f(state)`.
    /// - `state` The current state. This will be cloned, then modified, to create a delta.
    pub fn from_state_value<F, T>(mut f: F, value: T, state: &mut State) -> Self
    where
        F: FnMut(&mut State) -> &mut T,
    {
        let s0 = state.clone();
        *f(state) = value;
        Self::from_states(s0, state)
    }

    /// Calls a function and returns a snapshot of the delta between two states.
    ///
    /// - `f` A function that accepts a `State` parameter and returns a mutable reference value of type T.
    /// - `state` The current state. This will be cloned, then modified, to create a delta.
    pub fn from_state<F>(mut f: F, state: &mut State) -> Self
    where
        F: FnMut(&mut State),
    {
        let s0 = state.clone();
        f(state);
        Self::from_states(s0, state)
    }

    /// Returns a snapshot of the delta between two states.
    ///
    /// - `from_state` The initial state of the delta. This is usually a clone of a `State` prior to modifying the primary `State`.
    /// - `to_state` The final state of the delta. This is a reference to the primary `State`.
    pub fn from_states(from_state: State, to_state: &mut State) -> Self {
        Self {
            from_state: Some(from_state),
            to_state: Some(to_state.clone()),
            ..Default::default()
        }
    }

    /// Returns a snapshot of the delta between to synth states.
    ///
    /// - `from_commands` A list of commands that will revert the `SynthState` to the initial state.
    /// - `to_commands` A list of commands that will set the `SynthState` to the new state. This list will be sent by the `Conn`.
    pub fn from_commands(from_commands: CommandsMessage, to_commands: &CommandsMessage) -> Self {
        Self {
            from_commands: Some(from_commands),
            to_commands: Some(to_commands.clone()),
            ..Default::default()
        }
    }

    /// Returns a snapshot of the delta between two states as well as two synth states.
    ///
    /// - `from_state` The initial state of the delta. This is usually a clone of a `State` prior to modifying the primary `State`.
    /// - `to_state` The final state of the delta. This is a reference to the primary `State`.
    /// - `from_commands` A list of commands that will revert the `SynthState` to the initial state.
    /// - `to_commands` A list of commands that will set the `SynthState` to the new state. This list will be sent by the `Conn`.
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

    /// Returns a snapshot that just contains IOCommands.
    ///
    /// - `io_commands` A list of IOCommands that will be processed by the `IO`.
    pub fn from_io_commands(io_commands: Vec<IOCommand>) -> Self {
        Self {
            io_commands: Some(io_commands),
            ..Default::default()
        }
    }

    /// Returns a snapshot that flips the from/to of `snapshot`. This is used for undo/redo.
    ///
    /// - The Snapshot. Its `from_state` will become the returned Snapshot's `to_state` and vice-versa. Its `from_commands` will become the returned Snapshot's `to_commands` and vice-versa.
    pub fn from_snapshot(snapshot: &Snapshot) -> Self {
        Self {
            from_state: snapshot.to_state.clone(),
            to_state: snapshot.from_state.clone(),
            from_commands: snapshot.to_commands.clone(),
            to_commands: snapshot.from_commands.clone(),
            io_commands: None,
        }
    }
}
