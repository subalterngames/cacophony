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
    /// If true, the note is within the viewport's pitch range (`dn`).
    /// We need this because we want to render note rectangles only for notes in the pitch range, but we also want to render volume lines for notes beyond the pitch range.
    pub in_pitch_range: bool,
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
    /// - `x` The x pixel coordinate of the note's position.
    /// - `w` The pixel width of the note.
    /// - `state` The app state.
    /// - `conn` The audio conn.
    /// - `focus` If true, the piano roll panel has focus.
    /// - `dt` The time delta.
    pub fn new(
        x: f32,
        w: f32,
        state: &'a State,
        conn: &Conn,
        focus: bool,
        dt: [U64orF32; 2],
    ) -> Self {
        match state.music.get_selected_track() {
            Some(track) => Self::new_from_track(x, w, track, state, conn, focus, dt, state.view.dn),
            None => Self {
                x,
                w,
                notes: vec![],
                dt,
            },
        }
    }

    /// - `x` The x pixel coordinate of the note's position.
    /// - `w` The pixel width of the note.
    /// - `xw` The x and w pixel values of the rectangle where notes can be rendered.
    /// - `track` The track.
    /// - `state` The app state.
    /// - `conn` The audio conn.
    /// - `focus` If true, the piano roll panel has focus.
    /// - `dt` The time delta.
    /// - `dn` The range of viewable note pitches.
    #[allow(clippy::too_many_arguments)]
    pub fn new_from_track(
        x: f32,
        w: f32,
        track: &'a MidiTrack,
        state: &'a State,
        conn: &Conn,
        focus: bool,
        dt: [U64orF32; 2],
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

        // Get the selected notes.
        let selected = match state.select_mode.get_notes(&state.music) {
            Some(selected) => selected,
            None => vec![],
        };
        let mut notes = vec![];
        for note in track.notes.iter() {
            // Is the note in view?
            if !(note.end >= dt[0].get_u() && note.start <= dt[1].get_u()) {
                continue;
            }
            // Get the start time of the note. This could be the start of the viewport.
            let t = if note.start < dt[0].get_u() {
                dt[0].get_u()
            } else {
                note.start
            };
            // Get the x coordinate of the note.
            let x = get_note_x(t, x, w, &dt);
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
            let in_pitch_range = note.note <= dn[0] && note.note > dn[1];
            // Add the note.
            notes.push(ViewableNote {
                note,
                x,
                color,
                selected,
                playing,
                in_pitch_range,
            });
        }
        Self { notes, x, w, dt }
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
}

/// Returns the x pixel coordinate corresonding with time `t` within the viewport defined by `x`, `w` and `dt`.
pub(crate) fn get_note_x(t: u64, x: f32, w: f32, dt: &[U64orF32; 2]) -> f32 {
    x + w * ((t as f32 - dt[0].get_f()) / (dt[1].get_f() - dt[0].get_f()))
}