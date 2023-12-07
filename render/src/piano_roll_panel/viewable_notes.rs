use crate::panel::*;
use audio::play_state::PlayState;
use common::*;

/// A viewable note.
pub(crate) struct ViewableNote {
    /// The note.
    pub note: Note,
    /// The x pixel coordinate of the note.
    pub x: f32,
    /// The pixel width of the note.
    pub w: f32,
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
#[derive(Default)]
pub(crate) struct ViewableNotes {
    /// The notes that are in view.
    pub notes: Vec<ViewableNote>,
    /// The number of pulses in 1 pixel.
    pub pulses_per_pixel: u64,
}

impl ViewableNotes {
    /// - `x` The x pixel coordinate of the note's position.
    /// - `w` The pixel width of the note.
    /// - `state` The app state.
    /// - `conn` The audio conn.
    /// - `focus` If true, the piano roll panel has focus.
    /// - `dt` The time delta.
    pub fn new(x: f32, w: f32, state: &State, conn: &Conn, focus: bool, dt: [U64orF32; 2]) -> Self {
        match state.music.get_selected_track() {
            Some(track) => Self::new_from_track(x, w, track, state, conn, focus, dt, state.view.dn),
            None => Self {
                pulses_per_pixel: Self::get_pulses_per_pixel(&dt, w),
                notes: vec![],
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
        track: &MidiTrack,
        state: &State,
        conn: &Conn,
        focus: bool,
        dt: [U64orF32; 2],
        dn: [u8; 2],
    ) -> Self {
        let pulses_per_pixel = Self::get_pulses_per_pixel(&dt, w);
        // Get any notes being played.
        let playtime = match *conn.play_state.lock() {
            PlayState::Playing(time) => Some(state.time.samples_to_ppq(time, conn.framerate)),
            _ => None,
        };

        // Get the selected notes.
        let selected = match state.selection.get_selection(&state.music) {
            Some((notes, _)) => notes,
            None => vec![],
        };
        let (t0, t1) = (dt[0].get_u(), dt[1].get_u());
        let mut notes = vec![];
        for note in track.notes.iter() {
            // Is the note in view?
            if note.end <= t0 || note.start >= t1 {
                continue;
            }
            // Get the start time of the note. This could be the start of the viewport.
            let t = if note.start < t0 { t1 } else { note.start };
            // Get the x coordinate of the note.
            let x_note = Self::get_note_x(t, pulses_per_pixel, x, &dt);
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
            // Get the width of the note.
            let note_t0 = if note.start < t0 { t0 } else { note.start };
            let note_t1 = if note.end > t1 { t1 } else { note.end };
            let w_note = ((note_t1 - note_t0) / pulses_per_pixel) as f32;
            // Add the note.
            notes.push(ViewableNote {
                note: *note,
                x: x_note,
                w: w_note,
                color,
                selected,
                playing,
                in_pitch_range,
            });
        }
        Self {
            notes,
            pulses_per_pixel,
        }
    }

    /// Returns the x pixel coordinate corresonding with time `t` within the viewport defined by `x`, `w` and `dt`.
    ///
    /// - `t` The time in PPQ.
    /// - `ppp` The number of pulses in 1 pixel.
    pub fn get_note_x(t: u64, ppp: u64, x: f32, dt: &[U64orF32; 2]) -> f32 {
        if t >= dt[0].get_u() {
            x + ((t - dt[0].get_u()) / ppp) as f32
        } else {
            x + ((dt[0].get_u() - t) / ppp) as f32
        }
    }

    /// Returns the number of pulses in 1 pixel.
    ///
    /// - `dt` The start and end time in PPQ.
    /// - `w` The width of the view in pixels.
    pub fn get_pulses_per_pixel(dt: &[U64orF32; 2], w: f32) -> u64 {
        (((dt[1].get_f() - dt[0].get_f()) / w) as u64).clamp(1, u64::MAX)
    }
}
