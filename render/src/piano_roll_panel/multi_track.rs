use super::viewable_notes::{ViewableNote, ViewableNotes};
use crate::panel::*;
use crate::{get_track_heights, Page};
use common::config::parse;
use common::{U64orF32, MAX_NOTE, MIN_NOTE};

/// Track colors for when the panel has focus.
const TRACK_COLORS_FOCUS: [ColorKey; 6] = [
    ColorKey::Track0Focus,
    ColorKey::Track1Focus,
    ColorKey::Track2Focus,
    ColorKey::Track3Focus,
    ColorKey::Track4Focus,
    ColorKey::Track5Focus,
];
/// Track colors for when the panel doesn't have focus.
const TRACK_COLORS_NO_FOCUS: [ColorKey; 6] = [
    ColorKey::Track0NoFocus,
    ColorKey::Track1NoFocus,
    ColorKey::Track2NoFocus,
    ColorKey::Track3NoFocus,
    ColorKey::Track4NoFocus,
    ColorKey::Track5NoFocus,
];
/// The min/max note delta as a float.
const DN_F: f32 = (MAX_NOTE - MIN_NOTE) as f32;
/// The viewable note delta.
const DN: [u8; 2] = [MAX_NOTE, MIN_NOTE];

/// View multiple tracks at the same time.
pub(crate) struct MultiTrack {
    /// The rectangle of the entire multi-track sub-panel.
    rect: Rectangle,
    /// The (x, y, w, h) of the sub-panel in pixels.
    rect_f: [f32; 4],
    /// The height of each note in pixels.
    note_height: f32,
    /// The string used for drawing an arrow.
    arrow: Label,
}

impl MultiTrack {
    pub fn new(config: &Ini, renderer: &Renderer) -> Self {
        let section = config.section(Some("RENDER")).unwrap();
        let note_height = parse(section, "multi_track_note_height");
        let piano_roll_panel_position = get_piano_roll_panel_position(config);
        let piano_roll_panel_size = get_piano_roll_panel_size(config);
        let position = [
            piano_roll_panel_position[0] + 1 + PIANO_ROLL_PANEL_NOTE_NAMES_WIDTH,
            piano_roll_panel_position[1] + PIANO_ROLL_PANEL_TOP_BAR_HEIGHT + 1,
        ];
        let size = [
            piano_roll_panel_size[0] - 2 - PIANO_ROLL_PANEL_NOTE_NAMES_WIDTH,
            piano_roll_panel_size[1],
        ];
        let rect = Rectangle::new(position, size);
        let position_f = renderer.grid_to_pixel(position);
        let size_f = renderer.grid_to_pixel(size);
        let rect_f = [position_f[0], position_f[1], size_f[0], size_f[1]];
        let mut arrow_text = String::from("<");
        for _ in 0..PIANO_ROLL_PANEL_NOTE_NAMES_WIDTH as usize + 1 {
            arrow_text.push('=');
        }
        let arrow = Label {
            position: [piano_roll_panel_position[0] - 1, position[1]],
            text: arrow_text,
        };
        Self {
            rect,
            rect_f,
            note_height,
            arrow,
        }
    }

    pub(crate) fn update(
        &self,
        dt: [U64orF32; 2],
        renderer: &Renderer,
        state: &State,
        conn: &Conn,
    ) {
        let focus = state.panels[state.focus.get()] == PanelType::PianoRoll;
        // Get the page.
        let track_heights = get_track_heights(state, conn);
        let page = Page::new(&state.music.selected, &track_heights, self.rect.size[1]).visible;
        let x = self.rect.position[0];
        let mut y = self.rect.position[1];
        let w = self.rect.size[0];
        let mut color_index = 0;
        // Iterate through the heights and indices.
        for (height, i) in track_heights.iter().zip(page) {
            // Draw a rectangle.
            let rect = Rectangle::new([x, y], [w, *height]);
            let color = if focus {
                TRACK_COLORS_FOCUS[color_index]
            } else {
                TRACK_COLORS_NO_FOCUS[color_index]
            };
            renderer.rectangle(&rect, &color);
            // Draw an arrow if this is the selected track.
            if let Some(selected) = state.music.selected {
                if selected == i {
                    let mut arrow = self.arrow.clone();
                    arrow.position[1] = y + *height / 2;
                    renderer.text(&arrow, &color);
                }
            }
            // Increment the color index.
            color_index += 1;
            if color_index >= TRACK_COLORS_FOCUS.len() {
                color_index = 0;
            }
            // Get the track.
            let track = &state.music.midi_tracks[i];
            // Get the viewable notes.
            let notes = ViewableNotes::new_from_track(
                self.rect_f[0],
                self.rect_f[2],
                track,
                state,
                conn,
                focus,
                dt,
                DN,
            );
            // Draw the selection background.
            let selected = notes
                .notes
                .iter()
                .filter(|n| n.selected)
                .collect::<Vec<&ViewableNote>>();
            let h = renderer.grid_to_pixel([0, 1])[1] * *height as f32;
            let position = renderer.grid_to_pixel([x, y]);
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
                    let x1 = ViewableNotes::get_note_x(
                        select_1.note.end,
                        notes.pulses_per_pixel,
                        self.rect_f[0],
                        &dt,
                    ) - dt[0].get_f();
                    renderer.rectangle_pixel(
                        [select_0.x, position[1]],
                        [x1 - select_0.x, h],
                        &color,
                    )
                }
            }
            // Draw some notes.
            for note in notes.notes.iter() {
                let note_y = position[1] + (1.0 - ((note.note.note - MIN_NOTE) as f32) / DN_F) * h;
                let note_w = notes.get_note_w(note);
                renderer.rectangle_pixel(
                    [note.x, note_y],
                    [note_w, self.note_height],
                    &ColorKey::Background,
                )
            }
            y += *height;
        }
    }
}
