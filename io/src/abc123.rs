use crate::panel::*;

/// Handle alphanumeric input.
///
/// Toggle alphanumeric input. If alphanumeric input is disabled, check if the value is empty and set a default value if it is.
/// Otherwise, allow the user to type.
///
/// - `f` A closure to modify a string, e.g. `|e| &mut e.metadata.title`.
/// - `input` The input state. This is used to check if alphanumeric input is allowed.
/// - `conn` The audio conn. This is used to set the exporter state.
/// - `exporter` The exporter state.
///
/// Returns a snapshot.
pub(crate) fn abc123_str_exporter<F, S>(
    mut f: F,
    input: &Input,
    state: &mut State,
    conn: &mut Conn,
    exporter: &mut Exporter,
    default_value: &str,
) -> Option<Snapshot>
where
    F: FnMut(&mut Exporter) -> &mut String,
{
    // Toggle alphanumeric input on or off.
    if input.happened(&InputEvent::ToggleAlphanumericInput) {
        // Toggle off alphanumeric input and possibly set the string.
        if state.input.alphanumeric_input {
            let s0 = state.clone();
            state.input.alphanumeric_input = false;
            // Don't allow an empty name.
            if f(exporter).is_empty() {
                Some(Snapshot::from_exporter_value(
                    f,
                    default_value.to_string(),
                    conn,
                    exporter,
                ))
            } else {
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
        let mut value = f(exporter).clone();
        if input.modify_string_abc123(&mut value) {
            Some(Snapshot::from_exporter_value(f, value, conn, exporter))
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

/// Set a value in the local exporter and then copy it to the conn's SynthState.
fn set_exporter_value<F, T>(
    mut f: F,
    value: T,
    conn: &mut Conn,
    exporter: &mut Exporter,
) -> Option<Snapshot>
where
    F: FnMut(&mut Exporter) -> &mut T,
{
    let c0 = vec![Command::SetExporter {
        exporter: Box::new(exporter.clone()),
    }];
    *f(exporter) = value;
    let c1 = vec![Command::SetExporter {
        exporter: Box::new(exporter.clone()),
    }];
    let snapshot = Some(Snapshot::from_commands(c0, &c1));
    conn.send(c1);
    snapshot
}

fn set_state_value<F, T>(mut f: F, value: T, state: &mut State) -> Option<Snapshot>
where
    F: FnMut(&mut State) -> &mut T,
{
    Some(Snapshot::from_state_value(f, value, state))
}
