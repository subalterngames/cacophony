use cacophony_core::{PanelType, State};
use cacophony_input::{Input, InputEvent};
use cacophony_text::Text;
pub mod tooltip;

/// The maximum size of the undo stack.
const MAX_UNDOS: usize = 100;

pub struct IO {
    undo: Vec<State>,
    redo: Vec<State>,
}

impl IO {
    pub fn update(&mut self, state: &mut State, input: &mut Input, text: &Text) {
        // Update the input state.
        input.update(state);

        // Undo.
        if input.happened(&InputEvent::Undo) && !self.undo.is_empty() {
            // Pop s1.
            let s1 = self.undo.remove(0);
            // Push s1 to the redo stack.
            self.redo.push(s1.clone());
            // Assign s1 to state.
            *state = s1;
        } else if input.happened(&InputEvent::Redo) && !self.redo.is_empty() {
            // Pop s1.
            let s1 = self.redo.remove(0);
            // Push s1 to the undo stack.
            self.undo.push(s1.clone());
            // Assign s1 to state.
            *state = s1;
        }

        // Cycle panels.
        if input.happened(&InputEvent::NextPanel) {
            state.focus.increment(true);
            self.push_undo(state);
        } else if input.happened(&InputEvent::PreviousPanel) {
            state.focus.increment(false);
            self.push_undo(state);
        }
    }

    /// Push this state to the undo stack and clear the redo stack.
    fn push_undo(&mut self, state: &State) {
        self.undo.push(state.clone());
        self.redo.clear();
        // Remove an undo if there are too many.
        if self.undo.len() > MAX_UNDOS {
            self.undo.remove(0);
        }
    }
}
