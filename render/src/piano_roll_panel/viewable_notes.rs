use crate::panel::*;
use common::*;

/// A viewable note.
pub(crate) struct ViewableNote<'a> {
    /// The note.
    pub note: &'a Note,
    /// The x pixel coordinate of the note.
    pub x: f32,
    /// If true, this note is being played.
    pub playing: bool,
    /// If true, this note is selected.
    pub selected: bool,
    /// The color of this note.
    pub color: ColorKey,
}

/// Render information for all notes that are in the viewport.
/// This information is shared between the piano roll and volume sub-panels.
pub(crate) struct ViewableNotes<'a> {
    /// The notes that are in view.
    pub notes: Vec<ViewableNote<'a>>,
    /// Cached viewport dt in PPQ.
    dt: [U64orF32; 2],
    /// The x pixel coordinate of the viewport.
    x: f32,
    /// The width in pixels of the viewport.
    w: f32,
}

impl<'a> ViewableNotes<'a> {
    /// - `xw` The x and w pixel values of the rectangle where notes can be rendered.
    /// - `state` The app state.
    /// - `conn` The audio conn.
    /// - `focus` If true, the piano roll panel has focus.
    /// - `dn` The range of viewable note pitches.
    pub fn new(xw: [f32; 2], state: &'a State, conn: &Conn, focus: bool, dn: [u8; 2]) -> Self {
        match state.music.get_selected_track() {
            Some(track) => Self::new_from_track(xw, track, state, conn, focus, dn),
            None => Self {
                x: xw[0],
                w: xw[1],
                notes: vec![],
                dt: Self::get_dt(&state.view.dt),
            },
        }
    }

    /// - `xw` The x and w pixel values of the rectangle where notes can be rendered.
    /// - `track` The track.
    /// - `state` The app state.
    /// - `conn` The audio conn.
    /// - `focus` If true, the piano roll panel has focus.
    /// - `dn` The range of viewable note pitches.
    pub fn new_from_track(
        xw: [f32; 2],
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
                .map(|time| state.time.samples_to_ppq(time, conn.framerate)),
            false => None,
        };

        let dt = Self::get_dt(&state.view.dt);
        // Get the selected notes.
        let selected = match state.select_mode.get_notes(&state.music) {
            Some(selected) => selected,
            None => vec![],
        };
        let mut notes = vec![];
        for note in track.notes.iter() {
            // Is the note in view?
            if !(note.end >= dt[0].get_u()
                && note.start <= dt[1].get_u()
                && note.note <= dn[0]
                && note.note >= dn[1])
            {
                continue;
            }
            // Get the start time of the note. This could be the start of the viewport.
            let t = if note.start < dt[0].get_u() {
                dt[0].get_u()
            } else {
                note.start
            };
            // Get the x coordinate of the note.
            let x = get_note_x(t, xw[0], xw[1], &dt);
            // Is this note in the selection?
            let selected = selected.contains(&note);
            // Is this note being played?
            let playing = match playtime {
                Some(playtime) => note.start <= playtime && note.end >= playtime,
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
                x,
                color,
                selected,
                playing,
            });
        }
        Self {
            notes,
            x: xw[0],
            w: xw[1],
            dt,
        }
    }

    /// Returns the width of a note.
    pub fn get_note_w(&self, note: &ViewableNote) -> f32 {
        let t1 = if note.note.end > self.dt[1].get_u() {
            self.dt[1].get_u()
        } else {
            note.note.end
        };
        (get_note_x(t1, self.x, self.w, &self.dt) - note.x).clamp(1.0, f32::MAX)
    }

    /// Returns the x pixel coordinate corresonding with time `t` within the viewport defined by `x`, `w` and `dt`.
    pub fn get_note_x(&self, t: u64, x: f32, w: f32) -> f32 {
        get_note_x(t, x, w, &self.dt)
    }

    /// Returns `dt` as `U64orF32`.
    pub fn get_dt(dt: &[u64; 2]) -> [U64orF32; 2] {
        [U64orF32::from(dt[0]), U64orF32::from(dt[1])]
    }
}

/// Returns the x pixel coordinate corresonding with time `t` within the viewport defined by `x`, `w` and `dt`.
pub(crate) fn get_note_x(t: u64, x: f32, w: f32, dt: &[U64orF32; 2]) -> f32 {
    x + w * ((t as f32 - dt[0].get_f()) / (dt[1].get_f() - dt[0].get_f()))
}
