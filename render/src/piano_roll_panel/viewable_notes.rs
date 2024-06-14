use crate::panel::*;
use audio::play_state::PlayState;
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
    /// The number of pulses in 1 pixel.
    pub pulses_per_pixel: u64,
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
                pulses_per_pixel: Self::get_pulses_per_pixel(&dt, w),
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
        let pulses_per_pixel = Self::get_pulses_per_pixel(&dt, w);
        // Get any notes being played.
        let playtime = match *conn.play_state.lock() {
            PlayState::Playing(time) => Some(state.time.samples_to_ppq(time, conn.framerate)),
            _ => None,
        };

        // Get the selected notes.
        let selected = state
            .select_mode
            .get_notes(&state.music)
            .unwrap_or_default();
        let mut notes = vec![];
        for note in track.notes.iter() {
            // Is the note in view?
            if note.end <= dt[0].get_u() || note.start >= dt[1].get_u() {
                continue;
            }
            // Get the start time of the note. This could be the start of the viewport.
            let t = if note.start < dt[0].get_u() {
                dt[0].get_u()
            } else {
                note.start
            };
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
            // Add the note.
            notes.push(ViewableNote {
                note,
                x: x_note,
                color,
                selected,
                playing,
                in_pitch_range,
            });
        }
        Self {
            notes,
            dt,
            pulses_per_pixel,
        }
    }

    /// Returns the width of a note.
    pub fn get_note_w(&self, note: &ViewableNote) -> f32 {
        let t0 = if note.note.start < self.dt[0].get_u() {
            self.dt[0].get_u()
        } else {
            note.note.start
        };
        let t1 = if note.note.end > self.dt[1].get_u() {
            self.dt[1].get_u()
        } else {
            note.note.end
        };
        ((t1 - t0) / self.pulses_per_pixel) as f32
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
