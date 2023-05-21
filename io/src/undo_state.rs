use audio::CommandsMessage;
use common::State;

/// A state that can be undone. Includes the global state and audio commands.
#[derive(Clone)]
pub(crate) struct UndoState {
    /// The state.
    pub(crate) state: State,
    /// A list of commands to send.
    pub(crate) commands: CommandsMessage,
}

impl From<State> for UndoState {
    fn from(value: State) -> Self {
        Self {
            state: value,
            commands: vec![],
        }
    }
}
