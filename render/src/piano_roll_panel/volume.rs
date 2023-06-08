use super::notes::*;
use crate::panel::*;

/// The piano roll volume sub-panel.
pub(crate) struct Volume {
    /// The position and size of the panel in grid units.
    rect: Rectangle,
    /// The title label for the panel.
    title: Label,
    /// The position and size of the title in grid units.
    title_rect: Rectangle,
    /// The panel pixel dimensions.
    rect_f: [f32; 4],
    /// The min and max extents of the lines.
    line_extents: [f32; 3],
}

impl Volume {
    pub fn new(config: &Ini, text: &Text, renderer: &Renderer) -> Self {
        let piano_roll_panel_position = get_piano_roll_panel_position(config);
        let piano_roll_panel_size = get_piano_roll_panel_size(config);
        let position = [
            piano_roll_panel_position[0],
            piano_roll_panel_position[1] + piano_roll_panel_size[1],
        ];
        let size = [piano_roll_panel_size[0], PIANO_ROLL_PANEL_VOLUME_HEIGHT];
        let rect = Rectangle::new(position, size);
        let title_position = [position[0] + 2, position[1]];
        let title_text = text.get("PIANO_ROLL_PANEL_VOLUME_TITLE");
        let title_width = title_text.chars().count() as u32;
        let title = Label {
            text: title_text,
            position: title_position,
        };
        let title_rect = Rectangle::new(title_position, [title_width, 1]);

        let position_f = renderer.grid_to_pixel([
            position[0] + 1 + PIANO_ROLL_PANEL_NOTE_NAMES_WIDTH,
            position[1] + 1,
        ]);
        let size_f =
            renderer.grid_to_pixel([size[0] - 2 - PIANO_ROLL_PANEL_NOTE_NAMES_WIDTH, size[1] - 2]);
        let rect_f = [position_f[0], position_f[1], size_f[0], size_f[1]];

        let line_y1 = position_f[1];
        let line_y0 = position_f[1] + size_f[1];
        let line_extents = [line_y1, line_y0, line_y1 - line_y0];

        Self {
            rect,
            title,
            title_rect,
            rect_f,
            line_extents,
        }
    }

    pub fn update(&self, renderer: &Renderer, state: &State) {
        // Get focus.
        let focus = state.panels[state.focus.get()] == PanelType::PianoRoll;
        // Draw the panel background.
        let (bg_color, line_color) = if focus {
            (ColorKey::FocusDefault, ColorKey::Note)
        } else {
            (ColorKey::NoFocus, ColorKey::NoFocus)
        };
        renderer.rectangle(&self.rect, &ColorKey::Background);
        renderer.border(&self.rect, &bg_color);
        renderer.rectangle(&self.title_rect, &ColorKey::Background);
        renderer.text(&self.title, &bg_color);
        // Are there volumes to draw?
        if let Some(track) = state.music.get_selected_track() {
            for note in track.notes.iter() {
                // Ignore notes that aren't in the viewport.
                if !is_note_in_view(note, note.start + note.duration, state) {
                    continue;
                }
                // Get the x coordinate of the volume line.
                let x = get_note_x(note.start, &self.rect_f, state);
                // Get the height of the line.
                let h = self.line_extents[2] * (note.velocity as f32 / 127.0);
                let bottom = self.line_extents[0];
                let top = bottom - h;
                renderer.vertical_line_pixel(x, top, bottom, &line_color)
            }
        }
    }
}
