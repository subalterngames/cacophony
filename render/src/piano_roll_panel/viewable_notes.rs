use crate::panel::*;
use common::*;

/// A viewable note.
struct ViewableNote<'a> {
    /// The note.
    note: &'a Note,
    /// Cached end time of the note.
    end: Fraction,
    /// The x pixel coordinate of the note.
    x: f32,
}

/// Render information for all notes that are in the viewport.
/// This information is shared between the piano roll and volume sub-panels.
pub(crate) struct ViewableNotes<'a> {
    /// The notes that are in view.
    notes: Vec<ViewableNote<'a>>,
    /// Cached viewport dt.
    dt: &'a [Fraction; 2],
    /// The x pixel coordinate of the viewport.
    x: f32,
    /// The width in pixels of the viewport.
    w: f32,
}

impl<'a> ViewableNotes<'a> {
    pub(crate) fn new(x: f32, w: f32, state: &'a State) -> Self {
        let dt = &state.view.dt;
        let notes = match &state.music.get_selected_track() {
            Some(track) => {
                let mut notes = vec![];
                for note in track.notes.iter() {
                    // Get the end time.
                    let end = note.start + note.duration;
                    // Is the note in view?
                    if !(end >= dt[0]
                        && note.start <= dt[1]
                        && note.note <= state.view.dn[0]
                        && note.note >= state.view.dn[1])
                    {
                        continue;
                    }
                    // Get the start time of the note. This could be the start of the viewport.
                    let t = if note.start < dt[0] {
                        dt[0]
                    } else {
                        note.start
                    };
                    let x = get_note_x(t, x, w, dt);
                    notes.push(ViewableNote { note, end, x });
                }
                notes
            }
            None => vec![],
        };
        Self { notes, x, w, dt }
    }

    /// Returns the number of viewable notes.
    pub(crate) fn get_num(&self) -> usize {
        self.notes.len()
    }

    /// Returns the x coordinate of the note at `index`.
    pub(crate) fn get_note_x(&self, index: usize) -> f32 {
        self.notes[index].x
    }

    /// Returns the width of the note at `index`.
    pub(crate) fn get_note_w(&self, x: f32, index: usize) -> f32 {
        let t = if self.notes[index].end > self.dt[1] {
            self.dt[1]
        } else {
            self.notes[index].end
        };
        let x1 = get_note_x(t, self.x, self.w, self.dt);
        if x1 <= x { 1.0 } else { x1 - x }
    }

    /// Returns the note at `index`.
    pub(crate) fn get_note(&self, index: usize) -> &'a Note {
        self.notes[index].note
    }
}

/// Returns the x pixel coordinate corresonding with time `t` within the viewport defined by `x`, `w` and `dt`.
pub(crate) fn get_note_x(t: Fraction, x: f32, w: f32, dt: &[Fraction; 2]) -> f32 {
    x + w * ((t - dt[0]) / dt[1]).to_f32().unwrap()
}
