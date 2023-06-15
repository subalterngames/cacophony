use crate::panel::*;
use crate::{get_page, get_track_heights};

const TRACK_COLORS: [ColorKey; 7] = [
    ColorKey::Track0,
    ColorKey::Track1,
    ColorKey::Track2,
    ColorKey::Track3,
    ColorKey::Track4,
    ColorKey::Track5,
    ColorKey::Track6,
];

/// View multiple tracks at the same time.
pub(crate) struct MultiTrack {
    /// The rectangle of the entire multi-track sub-panel.
    rect: Rectangle,
}

impl MultiTrack {
    pub fn new(config: &Ini) -> Self {
        let piano_roll_panel_position = get_piano_roll_panel_position(config);
        let piano_roll_panel_size = get_piano_roll_panel_size(config);
        let position = [
            piano_roll_panel_position[0] + 1,
            piano_roll_panel_position[1] + PIANO_ROLL_PANEL_TOP_BAR_HEIGHT,
        ];
        let size = [
            piano_roll_panel_size[0] - 2,
            piano_roll_panel_size[1] - PIANO_ROLL_PANEL_TOP_BAR_HEIGHT - 2,
        ];
        let rect = Rectangle::new(position, size);
        Self { rect }
    }
}

impl Drawable for MultiTrack {
    fn update(
        &self,
        renderer: &Renderer,
        state: &State,
        conn: &Conn,
        _: &Input,
        _: &Text,
        _: &PathsState,
    ) {
        // Get the page.
        let track_heights = get_track_heights(state, conn);
        let page = get_page(&state.music.selected, &track_heights, self.rect.size[1]);
        let x = self.rect.position[0];
        let mut y = self.rect.position[1];
        let w = self.rect.size[0];
        let mut color_index = 0;
        // Iterate through the heights and indices.
        for (height, i) in track_heights.iter().zip(page) {
            let track = &state.music.midi_tracks[i];
            // Draw a rectangle.
            let rect = Rectangle::new([x, y], [w, *height]);
            let color = &TRACK_COLORS[color_index];
            renderer.rectangle(&rect, &color);
            color_index += 1;
            if color_index >= TRACK_COLORS.len() {
                color_index = 0;
            }
            y += *height;
        }
    }
}
