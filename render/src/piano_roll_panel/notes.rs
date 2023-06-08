use common::{Fraction, Note, State, ToPrimitive};

/// Returns true if the note is in the viewport.
pub(crate) fn is_note_in_view(note: &Note, end: Fraction, state: &State) -> bool {
    end >= state.view.dt[0] && note.start <= state.view.dt[1] && note.note <= state.view.dn[0] && note.note >= state.view.dn[1]
}

/// Returns the left pixel coordinate of a note. `rect` is the panel or sub-panel rect.
pub(crate) fn get_note_x0(note: &Note, rect: &[f32; 4], state: &State) -> f32 {
    // Get the start time of the note. This could be the start of the viewport.
    let t = if note.start < state.view.dt[0] {
        state.view.dt[0]
    } 
    else {
        note.start
    };
    get_note_x(t, rect, state)
}

/// Returns the right pixel coordinate of a note. `rect` is the panel or sub-panel rect.
pub(crate) fn get_note_x1(end: Fraction, rect: &[f32; 4], state: &State) -> f32 {
    // Get the end time of the note. This could be the end of the viewport.
    let t = if end > state.view.dt[1] {
        state.view.dt[1]
    } else {
        end
    };
    get_note_x(t, rect, state)
}

pub(crate) fn get_note_x(t: Fraction, rect: &[f32; 4], state: &State) -> f32 {
    rect[0]
    + rect[2]
        * ((t - state.view.dt[0]) / state.view.dt[1])
            .to_f32()
            .unwrap()
}