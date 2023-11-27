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

/// Handle alphanumeric input for the exporter.
///
/// - `f` A closure to modify a string, e.g. `|e| &mut e.metadata.title`.
/// - `input` The input state. This is used to check if alphanumeric input is allowed.
/// - `exporter` The exporter state.
pub(crate) fn update_exporter<F, T>(mut f: F, input: &Input, exporter: &mut Exporter) -> bool
where
    F: FnMut(&mut Exporter) -> &mut T,
    T: Clone + AlphanumericModifiable,
{
    let value = f(exporter);
    value.modify(input)
}

/// Do something with an exporter when alphanumeric input is disabled.
///
/// - `f` A closure to modify a string, e.g. `|e| &mut e.metadata.title`.
/// - `input` The input state. This is used to check if alphanumeric input is allowed.
/// - `exporter` The exporter state.
/// - `default_value` If the current value of `f` isn't valid, set it to this.
pub(crate) fn on_disable_exporter<F, T>(mut f: F, exporter: &mut Exporter, default_value: T)
where
    F: FnMut(&mut Exporter) -> &mut T,
    T: Clone + AlphanumericModifiable,
{
    let v = f(exporter);
    // If the value is empty, set a default value.
    if !v.is_valid() {
        *v = default_value;
    }
}

/// Handle alphanumeric input for the app state.
///
///
/// - `f` A closure to modify a string, e.g. `|e| &mut e.metadata.title`.
/// - `state` The app state.
/// - `input` The input state. This is used to check if alphanumeric input is allowed.
///
/// Returns a snapshot.
pub(crate) fn update_state<F, T>(mut f: F, state: &mut State, input: &Input) -> Option<Snapshot>
where
    F: FnMut(&mut State) -> &mut T,
    T: Clone + AlphanumericModifiable,
{
    let mut value = f(state).clone();
    if value.modify(input) {
        Some(Snapshot::from_state_value(f, value, state))
    } else {
        None
    }
}

pub(crate) fn on_disable_state<F, T>(mut f: F, state: &mut State, default_value: T)
where
    F: FnMut(&mut State) -> &mut T,
    T: Clone + AlphanumericModifiable,
{
    let v = f(state);
    // Don't allow an empty value.
    if !v.is_valid() {
        *v = default_value;
    }
}
