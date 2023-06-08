use crate::panel::*;
mod image;
mod note_names;
mod piano_roll_rows;
use note_names::get_note_names;
use piano_roll_rows::get_piano_roll_rows;
mod top_bar;
use super::FocusableTexture;
use common::time::samples_to_bar;
use common::State;
use common::{Fraction, ToPrimitive, MAX_NOTE, MIN_NOTE};
use text::fraction;
use top_bar::TopBar;

/// Draw the piano roll panel.
pub struct PianoRollPanel {
    /// The panel.
    panel: Panel,
    /// Data for the top bar sub-panel.
    top_bar: TopBar,
    /// The note names textures.
    note_names: FocusableTexture,
    /// The position of the note names.
    note_names_position: [u32; 2],
    /// The height of the piano roll sub-panel.
    piano_roll_height: u32,
    /// The piano roll rows textures.
    piano_roll_rows: FocusableTexture,
    /// The position of the piano roll rows.
    piano_roll_rows_position: [u32; 2],
    /// The (x, y, w, h) values of the piano row rolls rect.
    piano_roll_rows_rect: [f32; 4],
    /// The size of a cell.
    cell_size: [f32; 2],
    /// The y coordinate of the time labels in grid units.
    time_y: u32,
    /// The time horizontal line y pixel coordinate.
    time_horizontal_line_y: f32,
}

impl PianoRollPanel {
    pub fn new(config: &Ini, state: &State, text: &Text, renderer: &Renderer) -> Self {
        let piano_roll_panel_position = get_piano_roll_panel_position(config);
        let piano_roll_panel_size = get_piano_roll_panel_size(config);
        let panel = Panel::new(
            PanelType::PianoRoll,
            piano_roll_panel_position,
            piano_roll_panel_size,
            text,
        );
        let top_bar = TopBar::new(config, text);
        let note_names = get_note_names(config, renderer);
        let note_names_position = [
            panel.rect.position[0] + 1,
            panel.rect.position[1] + PIANO_ROLL_PANEL_TOP_BAR_HEIGHT + 1,
        ];
        let piano_roll_height = (state.view.dn[0] - state.view.dn[1]) as u32;
        let piano_roll_rows = get_piano_roll_rows(config, renderer);
        let piano_roll_rows_position = [
            note_names_position[0] + PIANO_ROLL_PANEL_NOTE_NAMES_WIDTH,
            note_names_position[1],
        ];
        let tex = &piano_roll_rows.get(true);
        let piano_roll_rows_position_f = renderer.grid_to_pixel(piano_roll_rows_position);
        let piano_roll_rows_rect = [
            piano_roll_rows_position_f[0],
            piano_roll_rows_position_f[1],
            tex.width(),
            tex.height(),
        ];
        let cell_size = get_cell_size(config);
        let time_y = note_names_position[1] - 1;
        let time_horizontal_line_y = cell_size[1] * (time_y + 1) as f32;
        Self {
            panel,
            top_bar,
            note_names,
            note_names_position,
            piano_roll_height,
            piano_roll_rows,
            piano_roll_rows_position,
            piano_roll_rows_rect,
            cell_size,
            time_y,
            time_horizontal_line_y,
        }
    }

    /// Converts a time fraction to a note x pixel coordinate.
    fn get_note_x(&self, t: Fraction, state: &State) -> f32 {
        self.piano_roll_rows_rect[0]
            + self.piano_roll_rows_rect[2]
                * ((t - state.view.dt[0]) / state.view.dt[1])
                    .to_f32()
                    .unwrap()
    }

    /// Draw a horizontal line from a time label and optionally a vertical line down the rows.
    fn draw_time_lines(
        &self,
        x: u32,
        time: &Fraction,
        color: &ColorKey,
        state: &State,
        renderer: &Renderer,
    ) {
        // Get the pixel position of the start coordinate.
        let x0 = x as f32 * self.cell_size[0];
        // The time is before the start time.
        let (x1, vertical) = if time < &state.view.dt[0] {
            (self.piano_roll_rows_rect[0], false)
        }
        // The time is after the end time.
        else if time > &state.view.dt[1] {
            (
                self.piano_roll_rows_rect[0] + self.piano_roll_rows_rect[2],
                false,
            )
        }
        // The time is within the viewport.
        else {
            (self.get_note_x(*time, state), true)
        };
        // Draw a horizontal line.
        renderer.horizontal_line_pixel(x0, x1, self.time_horizontal_line_y, color);
        // Draw a vertical line.
        if vertical {
            renderer.vertical_line_pixel(
                x1,
                self.piano_roll_rows_rect[1],
                self.piano_roll_rows_rect[1] + self.piano_roll_rows_rect[3],
                color,
            );
        }
    }
}

impl Drawable for PianoRollPanel {
    fn update(
        &self,
        renderer: &Renderer,
        state: &State,
        conn: &Conn,
        _: &Input,
        text: &Text,
        _: &OpenFile,
    ) {
        let focus = self.panel.has_focus(state);

        // Panel background.
        self.panel.update(focus, renderer);

        let program_exists = match state.music.get_selected_track() {
            Some(track) => conn.state.programs.contains_key(&track.channel),
            None => false,
        };
        let focus = focus && program_exists;

        // Top bar.
        self.top_bar.update(state, renderer, text, focus);

        // Note names.
        let texture = self.note_names.get(focus);
        let rect = [
            0,
            ((MAX_NOTE - MIN_NOTE) - state.view.dn[0]) as u32,
            PIANO_ROLL_PANEL_NOTE_NAMES_WIDTH,
            self.piano_roll_height,
        ];
        renderer.texture(texture, self.note_names_position, Some(rect));

        // Piano roll rows.
        let texture = self.piano_roll_rows.get(focus);
        renderer.texture(texture, self.piano_roll_rows_position, None);

        // Cursor label.
        let cursor_color = if focus {
            ColorKey::TimeCursor
        } else {
            ColorKey::NoFocus
        };
        let cursor_x = self.panel.rect.position[0] + PIANO_ROLL_PANEL_NOTE_NAMES_WIDTH + 1;
        let cursor_string = text.get_with_values(
            "PIANO_ROLL_PANEL_CURSOR_TIME",
            &[&fraction(&state.time.cursor)],
        );
        let cursor_string_width = cursor_string.chars().count() as u32;
        let playback_x = cursor_x + cursor_string_width + 3;
        let cursor_label = Label {
            text: cursor_string,
            position: [cursor_x, self.time_y],
        };
        renderer.text(&cursor_label, &cursor_color);

        // Cursor horizontal line.
        let cursor_line_x0 = cursor_x + cursor_string_width / 2;

        // Playback label.
        let playback_color = if focus {
            ColorKey::TimePlayback
        } else {
            ColorKey::NoFocus
        };
        let playback_string = text.get_with_values(
            "PIANO_ROLL_PANEL_PLAYBACK_TIME",
            &[&fraction(&state.time.playback)],
        );
        let playback_line_x0 = playback_x + playback_string.chars().count() as u32 / 2;
        let playback_label = Label {
            text: playback_string,
            position: [playback_x, self.time_y],
        };
        renderer.text(&playback_label, &playback_color);

        // Time delta label.
        let dt_string = text.get_with_values(
            "PIANO_ROLL_PANEL_VIEW_DT",
            &[&fraction(&state.view.dt[0]), &fraction(&state.view.dt[1])],
        );
        let dt_x = self.panel.rect.position[0] + self.panel.rect.size[0]
            - dt_string.chars().count() as u32
            - 1;
        let dt_label = Label {
            text: dt_string,
            position: [dt_x, self.time_y],
        };
        renderer.text(&dt_label, &Renderer::get_key_color(focus));

        // Notes.
        if let Some(track) = state.music.get_selected_track() {
            // Get the end times to avoid redundant math.
            let end_times: Vec<Fraction> =
                track.notes.iter().map(|n| n.start + n.duration).collect();
            // Get the selected notes.
            let selected = match state.select_mode.get_notes(&state.music) {
                Some(selected) => selected,
                None => vec![],
            };
            // Get any notes being played.
            let playing = match conn.state.time.music {
                true => match conn.state.time.time {
                    Some(time) => {
                        let time = samples_to_bar(time, state.music.bpm);
                        track
                            .notes
                            .iter()
                            .enumerate()
                            .filter(|n| n.1.start <= time && end_times[n.0] >= time)
                            .map(|n| n.1)
                            .collect()
                    }
                    None => vec![],
                },
                false => vec![],
            };
            // Draw the notes.
            for note in track.notes.iter().enumerate() {
                // Ignore notes that aren't in the viewport.
                if end_times[note.0] < state.view.dt[0]
                    || note.1.start > state.view.dt[1]
                    || note.1.note > state.view.dn[0]
                    || note.1.note < state.view.dn[1]
                {
                    continue;
                }

                // Get the start time of the note. This could be the start of the viewport.
                let t0 = if note.1.start < state.view.dt[0] {
                    state.view.dt[0]
                } else {
                    note.1.start
                };

                // Get the x coordinate.
                let x = self.get_note_x(t0, state);

                // Get the end time of the note. This could be the end of the viewport.
                let t1 = if end_times[note.0] > state.view.dt[1] {
                    state.view.dt[1]
                } else {
                    end_times[note.0]
                };

                // Get the end x coordinate.
                let x1 = self.get_note_x(t1, state);

                // Get the width.
                let w = if x1 <= x { 1.0 } else { x1 - x };

                // Get the y value from the pitch.
                let y = self.piano_roll_rows_rect[1]
                    + ((state.view.dn[0] - note.1.note) as f32) * self.cell_size[1];

                // Get the color.
                let color = match focus {
                    true => match playing.contains(&note.1) {
                        true => &ColorKey::NotePlaying,
                        false => match selected.contains(&note.1) {
                            true => &ColorKey::NoteSelected,
                            false => &ColorKey::Note,
                        },
                    },
                    false => &ColorKey::NoFocus,
                };

                renderer.rectangle_pixel([x, y], [w, self.cell_size[1]], color)
            }
        }

        // Draw time lines.
        self.draw_time_lines(
            cursor_line_x0,
            &state.time.cursor,
            &cursor_color,
            state,
            renderer,
        );
        self.draw_time_lines(
            playback_line_x0,
            &state.time.playback,
            &playback_color,
            state,
            renderer,
        );
    }
}
