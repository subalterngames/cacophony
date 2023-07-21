use crate::panel::*;
use audio::exporter::Exporter;
use common::U64orF32;

/// A type that can be modified by user alphanumeric input.
pub(crate) trait AlphanumericModifiable {
    /// Returns true if the value is "valid" i.e. we don't need to set it to a default.
    fn is_valid(&self) -> bool;

    /// Modify the value.
    fn modify(&mut self, input: &Input) -> bool;
}

impl AlphanumericModifiable for String {
    fn is_valid(&self) -> bool {
        !self.is_empty()
    }

    fn modify(&mut self, input: &Input) -> bool {
        input.modify_string_abc123(self)
    }
}

impl AlphanumericModifiable for Option<String> {
    fn is_valid(&self) -> bool {
        self.is_some()
    }

    fn modify(&mut self, input: &Input) -> bool {
        let mut value = match self {
            Some(string) => string.clone(),
            None => String::new(),
        };
        if input.modify_string_abc123(&mut value) {
            *self = if value.is_empty() { None } else { Some(value) };
            true
        } else {
            false
        }
    }
}

impl AlphanumericModifiable for u64 {
    fn is_valid(&self) -> bool {
        *self > 0
    }

    fn modify(&mut self, input: &Input) -> bool {
        input.modify_u64(self)
    }
}

impl AlphanumericModifiable for U64orF32 {
    fn is_valid(&self) -> bool {
        self.get_u() > 0
    }

    fn modify(&mut self, input: &Input) -> bool {
        let mut u = self.get_u();
        if input.modify_u64(&mut u) {
            self.set(u);
            true
        } else {
            false
        }
    }
}

/// Handle alphanumeric input for a shared exporter.
///
/// Toggle alphanumeric input. If alphanumeric input is disabled, check if the value is empty and set a default value if it is.
/// Otherwise, allow the user to type.
///
/// - `f` A closure to modify a string, e.g. `|e| &mut e.metadata.title`.
/// - `state` The app state.
/// - `input` The input state. This is used to check if alphanumeric input is allowed.
/// - `exporter` The exporter state.
///
/// Returns a snapshot.
pub(crate) fn abc123_shared_exporter<F, T>(
    f: F,
    state: &mut State,
    input: &Input,
    exporter: &mut SharedExporter,
    default_value: T,
) -> Option<Snapshot>
where
    F: FnMut(&mut Exporter) -> &mut T,
    T: Clone + AlphanumericModifiable,
{
    let mut ex = exporter.lock();
    abc123_exporter(f, state, input, &mut ex, default_value)
}

/// Handle alphanumeric input for the exporter.
///
/// Toggle alphanumeric input. If alphanumeric input is disabled, check if the value is empty and set a default value if it is.
/// Otherwise, allow the user to type.
///
/// - `f` A closure to modify a string, e.g. `|e| &mut e.metadata.title`.
/// - `state` The app state.
/// - `input` The input state. This is used to check if alphanumeric input is allowed.
/// - `exporter` The exporter state.
///
/// Returns a snapshot.
pub(crate) fn abc123_exporter<F, T>(
    mut f: F,
    state: &mut State,
    input: &Input,
    exporter: &mut Exporter,
    default_value: T,
) -> Option<Snapshot>
where
    F: FnMut(&mut Exporter) -> &mut T,
    T: Clone + AlphanumericModifiable,
{
    // Toggle alphanumeric input on or off.
    if input.happened(&InputEvent::ToggleAlphanumericInput) {
        // Toggle off alphanumeric input and possibly set the string.
        if state.input.alphanumeric_input {
            let s0 = state.clone();
            state.input.alphanumeric_input = false;
            // If the value is empty, set a default value.
            if !f(exporter).is_valid() {
                *f(exporter) = default_value;
            }
            Some(Snapshot::from_states(s0, state))
        }
        // Toggle on alphanumeric input.
        else {
            set_alphanumeric_input(state, true)
        }
    }
    // Modify the value.
    else if state.input.alphanumeric_input {
        let mut value = f(exporter).clone();
        value.modify(input);
        None
    } else {
        None
    }
}

/// Handle alphanumeric input for the app state.
///
/// Toggle alphanumeric input. If alphanumeric input is disabled, check if the value is empty and set a default value if it is.
/// Otherwise, allow the user to type.
///
/// - `f` A closure to modify a string, e.g. `|e| &mut e.metadata.title`.
/// - `state` The app state.
/// - `input` The input state. This is used to check if alphanumeric input is allowed.
///
/// Returns a snapshot.
pub(crate) fn abc123_state<F, T>(
    mut f: F,
    state: &mut State,
    input: &Input,
    default_value: T,
) -> Option<Snapshot>
where
    F: FnMut(&mut State) -> &mut T,
    T: Clone + AlphanumericModifiable,
{
    // Toggle alphanumeric input on or off.
    if input.happened(&InputEvent::ToggleAlphanumericInput) {
        // Toggle off alphanumeric input and possibly set the string.
        if state.input.alphanumeric_input {
            let s0 = state.clone();
            state.input.alphanumeric_input = false;
            // Don't allow an empty value.
            if f(state).is_valid() {
                Some(Snapshot::from_states(s0, state))
            } else {
                *f(state) = default_value;
                Some(Snapshot::from_states(s0, state))
            }
        }
        // Toggle on alphanumeric input.
        else {
            set_alphanumeric_input(state, true)
        }
    }
    // Modify the value.
    else if state.input.alphanumeric_input {
        let mut value = f(state).clone();
        if value.modify(input) {
            Some(Snapshot::from_state_value(f, value, state))
        } else {
            None
        }
    } else {
        None
    }
}

/// Set whether alphanumeric input is allowed.
fn set_alphanumeric_input(state: &mut State, value: bool) -> Option<Snapshot> {
    Some(Snapshot::from_state_value(
        |s| &mut s.input.alphanumeric_input,
        value,
        state,
    ))
}
