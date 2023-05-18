use serde::{Deserialize, Serialize};

/// Booleans and numerical values describing the input state.
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct InputState {
    /// If true, we will accept musical input.
    pub armed: bool,
    /// If true, we're inputting an alphanumeric string and we should ignore certain key bindings.
    pub alphanumeric_input: bool,
}
