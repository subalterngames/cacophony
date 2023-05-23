use crate::{IOCommands, State};
use audio::CommandsMessage;

/// A state that can be undone. Includes the global state and audio commands.
#[derive(Clone)]
pub(crate) struct UndoState {
    /// The state.
    pub(crate) state: Option<State>,
    /// A list of commands to send to the audio connection.
    pub(crate) commands: Option<CommandsMessage>,
    /// A list of commands to send to the `IO` state.
    pub(crate) io_commands: IOCommands,
}

impl From<State> for UndoState {
    fn from(value: State) -> Self {
        Self {
            state: Some(value),
            commands: None,
            io_commands: None,
        }
    }
}

impl From<CommandsMessage> for UndoState {
    fn from(value: CommandsMessage) -> Self {
        Self {
            state: None,
            commands: Some(value),
            io_commands: None,
        }
    }
}

/// An undo state and a redo state.
pub(crate) struct UndoRedoState {
    pub(crate) undo: UndoState,
    pub(crate) redo: UndoState,
}

impl From<(State, &State)> for UndoRedoState {
    fn from(value: (State, &State)) -> Self {
        let undo = UndoState::from(value.0);
        let redo = UndoState::from(value.1.clone());
        Self { undo, redo }
    }
}

impl From<(State, &mut State)> for UndoRedoState {
    fn from(value: (State, &mut State)) -> Self {
        let undo = UndoState::from(value.0);
        let redo = UndoState::from(value.1.clone());
        Self { undo, redo }
    }
}

impl From<(CommandsMessage, &CommandsMessage)> for UndoRedoState {
    fn from(value: (CommandsMessage, &CommandsMessage)) -> Self {
        let undo = UndoState::from(value.0);
        let redo = UndoState::from(value.1.clone());
        Self { undo, redo }
    }
}

impl From<(State, CommandsMessage, &mut State, &CommandsMessage)> for UndoRedoState {
    fn from(value: (State, CommandsMessage, &mut State, &CommandsMessage)) -> Self {
        let undo = UndoState {
            state: Some(value.0),
            commands: Some(value.1),
            io_commands: None,
        };
        let redo = UndoState {
            state: Some(value.2.clone()),
            commands: Some(value.3.clone()),
            io_commands: None,
        };
        Self { undo, redo }
    }
}

impl From<(UndoState, &UndoState)> for UndoRedoState {
    fn from(value: (UndoState, &UndoState)) -> Self {
        Self {
            undo: value.0,
            redo: value.1.clone(),
        }
    }
}

impl From<IOCommands> for UndoRedoState {
    fn from(value: IOCommands) -> Self {
        let undo = UndoState {
            state: None,
            commands: None,
            io_commands: value,
        };
        let redo = UndoState {
            state: None,
            commands: None,
            io_commands: None,
        };
        Self { undo, redo }
    }
}
