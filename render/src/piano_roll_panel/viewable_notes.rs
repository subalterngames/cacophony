use crate::panel::*;
use common::time::samples_to_bar;
use common::*;

/// A viewable note.
pub(crate) struct ViewableNote<'a> {
    /// The note.
    pub note: &'a Note,
    /// Cached end time of the note.
    pub end: Fraction,
    /// The x pixel coordinate of the note.
    pub(crate) x: f32,
    /// If true, this note is being played.
    pub(crate) playing: bool,
    /// If true, this note is selected.
    pub(crate) selected: bool,
    /// The color of this note.
    pub(crate) color: ColorKey,
}

/// Render information for all notes that are in the viewport.
/// This information is shared between the piano roll and volume sub-panels.
pub(crate) struct ViewableNotes<'a> {
    /// The notes that are in view.
    pub(crate) notes: Vec<ViewableNote<'a>>,
    /// Cached viewport dt.
    dt: &'a [Fraction; 2],
    /// The x pixel coordinate of the viewport.
    x: f32,
    /// The width in pixels of the viewport.
    w: f32,
}

impl<'a> ViewableNotes<'a> {
    pub(crate) fn new(
        x: f32,
        w: f32,
        state: &'a State,
        conn: &Conn,
        focus: bool,
        dn: [u8; 2],
    ) -> Self {
        match state.music.get_selected_track() {
            Some(track) => Self::new_from_track(x, w, track, state, conn, focus, dn),
            None => Self {
                x,
                w,
                notes: vec![],
                dt: &state.view.dt,
            },
        }
    }

    pub(crate) fn new_from_track(
        x: f32,
        w: f32,
        track: &'a MidiTrack,
        state: &'a State,
        conn: &Conn,
        focus: bool,
        dn: [u8; 2],
    ) -> Self {
        // Get any notes being played.
        let playtime = match conn.state.time.music {
            true => conn
                .state
                .time
                .time
                .map(|time| samples_to_bar(time, state.music.bpm)),
            false => None,
        };

        let dt = &state.view.dt;
        // Get the selected notes.
        let selected = match state.select_mode.get_notes(&state.music) {
            Some(selected) => selected,
            None => vec![],
        };
        let mut notes = vec![];
        for note in track.notes.iter() {
            // Get the end time.
            let end = note.start + note.duration;
            // Is the note in view?
            if !(end >= dt[0] && note.start <= dt[1] && note.note <= dn[0] && note.note >= dn[1]) {
                continue;
            }
            // Get the start time of the note. This could be the start of the viewport.
            let t = if note.start < dt[0] {
                dt[0]
            } else {
                note.start
            };
            // Get the x coordinate of the note.
            let x = get_note_x(t, x, w, dt);
            // Is this note in the selection?
            let selected = selected.contains(&note);
            // Is this note being played?
            let playing = match playtime {
                Some(playtime) => note.start <= playtime && end >= playtime,
                None => false,
            };
            // Get the color of the note.
            let color = if focus {
                if playing {
                    ColorKey::NotePlaying
                } else if selected {
                    ColorKey::NoteSelected
                } else {
                    ColorKey::Note
                }
            } else {
                ColorKey::NoFocus
            };
            // Add the note.
            notes.push(ViewableNote {
                note,
                end,
                x,
                color,
                selected,
                playing,
            });
        }
        Self { notes, x, w, dt }
    }

    /// Returns the width of the note at `index`.
    pub(crate) fn get_note_w(&self, note: &ViewableNote) -> f32 {
        let t = if note.end > self.dt[1] {
            self.dt[1]
        } else {
            note.end
        };
        let x1 = get_note_x(t, self.x, self.w, self.dt);
        if x1 <= note.x {
            1.0
        } else {
            x1 - note.x
        }
    }
}

/// Returns the x pixel coordinate corresonding with time `t` within the viewport defined by `x`, `w` and `dt`.
pub(crate) fn get_note_x(t: Fraction, x: f32, w: f32, dt: &[Fraction; 2]) -> f32 {
    x + w * ((t - dt[0]) / dt[1]).to_f32().unwrap()
}
