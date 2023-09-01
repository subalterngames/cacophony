use crate::panel::*;
mod image;
mod piano_roll_rows;
use piano_roll_rows::get_piano_roll_rows;
mod multi_track;
mod top_bar;
mod viewable_notes;
mod volume;
use super::FocusableTexture;
use common::{SelectMode, State, U64orF32, NOTE_NAMES, PPQ_U};
use hashbrown::HashSet;
use multi_track::MultiTrack;
use text::ppq_to_string;
use top_bar::TopBar;
use viewable_notes::*;
use volume::Volume;

const TIME_PADDING: u32 = 3;

/// Draw the piano roll panel.
pub struct PianoRollPanel {
    /// The panel used in single-track mode.
    panel_single_track: Panel,
    /// The panel used in multi-track mode.
    panel_multi_track: Panel,
    /// Data for the top bar sub-panel.
    top_bar: TopBar,
    /// The volume sub-panel.
    volume: Volume,
    /// The multi-track sub-panel.
    multi_track: MultiTrack,
    /// The position of the note names.
    note_name_positions: Vec<[u32; 2]>,
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
    /// The bottom y coordinates for time lines in single- and multi- track modes.
    time_line_bottoms: [f32; 2],
}

impl PianoRollPanel {
    pub fn new(config: &Ini, text: &Text, renderer: &Renderer) -> Self {
        let piano_roll_panel_position = get_piano_roll_panel_position(config);
        let piano_roll_panel_size = get_piano_roll_panel_size(config);
        let panel_single_track = Panel::new(
            PanelType::PianoRoll,
            piano_roll_panel_position,
            piano_roll_panel_size,
            text,
        );
        let top_bar = TopBar::new(config, text);
        let note_names_position = [
            panel_single_track.rect.position[0] + 1,
            panel_single_track.rect.position[1] + PIANO_ROLL_PANEL_TOP_BAR_HEIGHT + 1,
        ];
        let viewport_height = get_viewport_size(config)[1];
        let note_name_positions: Vec<[u32; 2]> = (note_names_position[1]
            ..=note_names_position[1] + viewport_height)
            .map(|y| [note_names_position[0], y])
            .collect();
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
        let volume = Volume::new(config, text, renderer);
        let multi_track = MultiTrack::new(config, renderer);
        let mut panel_multi_track = panel_single_track.clone();
        panel_multi_track.rect.size[1] += volume.rect.size[1];
        let volume_size_f = renderer.grid_to_pixel(volume.rect.size);
        let time_line_bottom_single_track = piano_roll_rows_rect[1] + piano_roll_rows_rect[3];
        let time_line_bottoms = [
            time_line_bottom_single_track,
            time_line_bottom_single_track + volume_size_f[1],
        ];
        Self {
            panel_single_track,
            panel_multi_track,
            top_bar,
            note_name_positions,
            piano_roll_rows,
            piano_roll_rows_position,
            piano_roll_rows_rect,
            cell_size,
            time_y,
            time_horizontal_line_y,
            volume,
            multi_track,
            time_line_bottoms,
        }
    }

    /// Draw a horizontal line from a time label and optionally a vertical line down the rows.
    fn draw_time_lines(
        &self,
        x: u32,
        time: u64,
        color: &ColorKey,
        single_track: bool,
        renderer: &Renderer,
        dt: &[U64orF32; 2],
    ) {
        // Get the pixel position of the start coordinate.
        let x0 = x as f32 * self.cell_size[0];
        // The time is before the start time.
        let (x1, vertical) = if time < dt[0].get_u() {
            (self.piano_roll_rows_rect[0], false)
        }
        // The time is after the end time.
        else if time > dt[1].get_u() {
            (
                self.piano_roll_rows_rect[0] + self.piano_roll_rows_rect[2],
                false,
            )
        }
        // The time is within the viewport.
        else {
            (
                get_note_x(
                    time,
                    self.piano_roll_rows_rect[0],
                    self.piano_roll_rows_rect[2],
                    dt,
                ),
                true,
            )
        };
        // Draw a horizontal line.
        renderer.horizontal_line_pixel(x0, x1, self.time_horizontal_line_y, color);
        // Draw a vertical line.
        if vertical {
            renderer.vertical_line_pixel(
                x1,
                self.piano_roll_rows_rect[1],
                if single_track {
                    self.time_line_bottoms[0]
                } else {
                    self.time_line_bottoms[1]
                },
                color,
            );
        }
    }

    /// If music isn't playing, this returns `state.view.dt`.
    /// Otherwise, this returns a view delta that has been moved to include the current playback time.
    fn get_view_dt(state: &State, conn: &Conn) -> [u64; 2] {
        let dt = [state.view.dt[0], state.view.dt[1]];
        if conn.state.time.music {
            match conn.state.time.time {
                Some(time) => {
                    let time_ppq = state.time.samples_to_ppq(time, conn.framerate);
                    let delta = dt[1] - dt[0];
                    // This is maybe not the best way to round, but it gets the job done!
                    let t0 = (time_ppq / delta) * delta;
                    let t1 = t0 + delta;
                    [t0, t1]
                }
                None => dt,
            }
        }
        // If there is no music playing, just use the "actual" view.
        else {
            dt
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
        _: &PathsState,
        _: &SharedExporter,
    ) {
        let panel = if state.view.single_track {
            &self.panel_single_track
        } else {
            &self.panel_multi_track
        };
        let focus = panel.has_focus(state);

        // Panel background.
        panel.update(focus, renderer);

        let program_exists = match state.music.get_selected_track() {
            Some(track) => conn.state.programs.contains_key(&track.channel),
            None => false,
        };
        let focus = focus && program_exists;

        // Top bar.
        self.top_bar.update(state, renderer, text, focus);

        let dt = Self::get_view_dt(state, conn).map(U64orF32::from);

        if state.view.single_track {
            // Piano roll rows.
            let texture = &self.piano_roll_rows.get(focus);
            renderer.texture(texture, self.piano_roll_rows_position, None);
            // Get the viewable notes.
            let notes = ViewableNotes::new(
                self.piano_roll_rows_rect[0],
                self.piano_roll_rows_rect[2],
                state,
                conn,
                focus,
                dt,
            );
            // Draw the selection background.
            let selected = notes
                .notes
                .iter()
                .filter(|n| n.selected && n.in_pitch_range)
                .collect::<Vec<&ViewableNote>>();
            // Get the start and end of the selection.
            if let Some(select_0) = selected
                .iter()
                .min_by(|a, b| a.note.start.cmp(&b.note.start))
            {
                if let Some(select_1) = selected.iter().max_by(|a, b| a.note.end.cmp(&b.note.end)) {
                    let color = if focus {
                        ColorKey::SelectedNotesBackground
                    } else {
                        ColorKey::NoFocus
                    };
                    let x1 = notes.get_note_x(
                        select_1.note.end,
                        self.piano_roll_rows_rect[0],
                        self.piano_roll_rows_rect[2],
                    );
                    renderer.rectangle_pixel(
                        [select_0.x, self.piano_roll_rows_rect[1]],
                        [x1 - select_0.x, self.piano_roll_rows_rect[3]],
                        &color,
                    )
                }
            }

            let in_pitch_range: Vec<&ViewableNote> =
                notes.notes.iter().filter(|n| n.in_pitch_range).collect();
            let selected_pitches: Vec<u8> = selected
                .iter()
                .map(|n| n.note.note)
                .collect::<HashSet<u8>>()
                .into_iter()
                .collect();

            // Draw the notes.
            for note in in_pitch_range.iter() {
                let w = notes.get_note_w(note);
                // Get the y value from the pitch.
                let y = self.piano_roll_rows_rect[1]
                    + ((state.view.dn[0] - note.note.note) as f32) * self.cell_size[1];
                renderer.rectangle_pixel([note.x, y], [w, self.cell_size[1]], &note.color)
            }
            // Volume.
            self.volume.update(&notes, renderer, state);
            // Note names.
            let note_name_color = if focus {
                &ColorKey::Separator
            } else {
                &ColorKey::NoFocus
            };
            for (position, pitch) in self
                .note_name_positions
                .iter()
                .zip((state.view.dn[1] + 1..state.view.dn[0] + 1).rev())
            {
                let note_name = LabelRef::new(*position, NOTE_NAMES[127 - pitch as usize]);
                let note_name_color = if selected_pitches.contains(&pitch) {
                    &ColorKey::NoteSelected
                } else {
                    note_name_color
                };
                renderer.text_ref(&note_name, note_name_color);
            }
        }

        // Cursor label.
        let cursor_color = if focus {
            ColorKey::TimeCursor
        } else {
            ColorKey::NoFocus
        };
        let cursor_x = panel.rect.position[0] + PIANO_ROLL_PANEL_NOTE_NAMES_WIDTH + 1;
        let cursor_string = text.get_with_values(
            "PIANO_ROLL_PANEL_CURSOR_TIME",
            &[&ppq_to_string(state.time.cursor)],
        );
        let cursor_string_width = cursor_string.chars().count() as u32;
        let playback_x = cursor_x + cursor_string_width + TIME_PADDING;
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
            &[&ppq_to_string(state.time.playback)],
        );
        let playback_string_width = playback_string.chars().count() as u32;
        let playback_line_x0 = playback_x + playback_string_width / 2;
        let selection_x = playback_x + playback_string_width + TIME_PADDING;
        let (selection_string, selected) = match &state.select_mode {
            SelectMode::Single(index) => match index {
                Some(index) => {
                    let note = &state.music.get_selected_track().unwrap().notes[*index];
                    (
                        text.get_with_values(
                            "PIANO_ROLL_PANEL_SELECTED_SINGLE",
                            &[note.get_name(), &(note.start / PPQ_U).to_string()],
                        ),
                        true,
                    )
                }
                None => (text.get("PIANO_ROLL_PANEL_SELECTED_NONE"), false),
            },
            SelectMode::Many(indices) => match indices {
                Some(_) => {
                    let mut notes = state.select_mode.get_notes(&state.music).unwrap();
                    notes.sort();
                    let min = notes[0].start / PPQ_U;
                    let max = notes.last().unwrap().end / PPQ_U;
                    (
                        text.get_with_values(
                            "PIANO_ROLL_PANEL_SELECTED_MANY",
                            &[&min.to_string(), &max.to_string()],
                        ),
                        true,
                    )
                }
                None => (text.get("PIANO_ROLL_PANEL_SELECTED_NONE"), false),
            },
        };
        let playback_label = Label {
            text: playback_string,
            position: [playback_x, self.time_y],
        };
        renderer.text(&playback_label, &playback_color);
        let selection_label = Label {
            text: selection_string,
            position: [selection_x, self.time_y],
        };
        // Current playback time.
        if conn.state.time.music {
            if let Some(music_time) = conn.state.time.time {
                let music_time_string =
                    ppq_to_string(state.time.samples_to_ppq(music_time, conn.framerate));
                let music_time_x =
                    selection_x + selection_label.text.chars().count() as u32 + TIME_PADDING;
                let music_time_label = Label {
                    text: music_time_string,
                    position: [music_time_x, self.time_y],
                };
                renderer.text(&music_time_label, &Renderer::get_key_color(focus));
            }
        }
        renderer.text(
            &selection_label,
            &if focus {
                if selected {
                    ColorKey::NoteSelected
                } else {
                    ColorKey::SelectedNotesBackground
                }
            } else {
                ColorKey::NoFocus
            },
        );

        // Time delta label.
        let dt_string = text.get_with_values(
            "PIANO_ROLL_PANEL_VIEW_DT",
            &[&ppq_to_string(dt[0].get_u()), &ppq_to_string(dt[1].get_u())],
        );
        let dt_x =
            panel.rect.position[0] + panel.rect.size[0] - dt_string.chars().count() as u32 - 1;
        let dt_label = Label {
            text: dt_string,
            position: [dt_x, self.time_y],
        };
        renderer.text(&dt_label, &Renderer::get_key_color(focus));

        if state.view.single_track {
        }
        // Multi-track.
        else {
            self.multi_track.update(dt, renderer, state, conn);
        }

        // Draw time lines.
        self.draw_time_lines(
            cursor_line_x0,
            state.time.cursor,
            &cursor_color,
            state.view.single_track,
            renderer,
            &dt,
        );
        self.draw_time_lines(
            playback_line_x0,
            state.time.playback,
            &playback_color,
            state.view.single_track,
            renderer,
            &dt,
        );
        // Show where we are in the music.
        if conn.state.time.music {
            if let Some(music_time) = conn.state.time.time {
                let music_time = state.time.samples_to_ppq(music_time, conn.framerate);
                if music_time >= dt[0].get_u() && music_time <= dt[1].get_u() {
                    let x = get_note_x(
                        music_time,
                        self.piano_roll_rows_rect[0],
                        self.piano_roll_rows_rect[2],
                        &dt,
                    );
                    let music_color = if focus {
                        ColorKey::FocusDefault
                    } else {
                        ColorKey::NoFocus
                    };
                    renderer.vertical_line_pixel(
                        x,
                        self.piano_roll_rows_rect[1],
                        if state.view.single_track {
                            self.time_line_bottoms[0]
                        } else {
                            self.time_line_bottoms[1]
                        },
                        &music_color,
                    );
                }
            }
        }
    }
}
