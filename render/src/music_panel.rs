use crate::panel::*;
use common::music_panel_field::*;

/// The music panel.
pub(crate) struct MusicPanel {
    /// The panel background.
    panel: Panel,
    /// The name field.
    name: Width,
    /// The total span of the name field, including where the corners are renderered.
    name_rect: Rectangle,
    /// The rectangle of the backround of the namefield.
    name_input_rect: Rectangle,
    /// The BPM field.
    bpm: KeyInput,
    /// The gain field.
    gain: KeyList,
    /// The rectangle of the background of the name field.
    gain_rect: Rectangle,
}

impl MusicPanel {
    pub fn new(config: &Ini, text: &Text) -> Self {
        // Get the width of the panel.
        let mut width = get_tracks_panel_width(config);
        // Get the panel.
        let panel = Panel::new(
            PanelType::Music,
            MUSIC_PANEL_POSITION,
            [width, MUSIC_PANEL_HEIGHT],
            text,
        );

        // Move the (x, y) coordinates inward by 1.
        let x = panel.rect.position[0] + 1;
        let mut y = panel.rect.position[1] + 1;
        // Shorten the width for the fields.
        width -= 2;
        let w_usize = width as usize;

        // Set the fields.
        let name = Width::new([x + 1, y], w_usize - 2);
        let name_rect = Rectangle::new([x, y], [width, 1]);
        let name_input_rect = Rectangle::new(name.position, [name.width_u32, 1]);
        y += 1;
        let bpm = KeyInput::new_from_value_width(text.get_ref("TITLE_BPM"), [x, y], width, 4);
        // Move the position of the value to align it with the gain field.
        y += 1;
        let gain = KeyList::new(text.get("TITLE_GAIN"), [x + 1, y], width - 2, 3);
        let gain_rect = Rectangle::new([x, y], [width, 1]);

        // Return.
        Self {
            panel,
            name,
            name_rect,
            name_input_rect,
            bpm,
            gain,
            gain_rect,
        }
    }
}

impl Drawable for MusicPanel {
    fn update(&self, renderer: &Renderer, state: &State, conn: &Conn, _: &Text, _: &PathsState) {
        // Get the focus,
        let focus = self.panel.has_focus(state);
        // Draw the rect.
        self.panel.update(focus, renderer);
        // Get the enum value of the focused widget.
        let focused_field = state.music_panel_field.get();

        let key_color = Renderer::get_key_color(focus);

        // Name.
        let name_focus = focused_field == MusicPanelField::Name;
        if name_focus {
            // Draw corners.
            renderer.corners(&self.name_rect, focus);
            // Draw a rectangle for input.
            if state.input.alphanumeric_input {
                renderer.rectangle(&self.name_input_rect, &ColorKey::TextFieldBG);
            }
        }
        // Draw the name.
        renderer.text_ref(
            &self.name.to_label(&conn.exporter.metadata.title),
            &Renderer::get_value_color([focus, name_focus]),
        );

        // BPM.
        let bpm_focus = focused_field == MusicPanelField::BPM;
        if bpm_focus {
            // Draw corners.
            renderer.corners(&self.bpm.corners_rect, focus);
            // Draw a rectangle for input.
            if state.input.alphanumeric_input {
                renderer.rectangle(&self.bpm.input_rect, &ColorKey::TextFieldBG);
            }
        }
        // Draw the BPM.
        renderer.key_value(
            &state.time.bpm.to_string(),
            &self.bpm.key_width,
            [&key_color, &Renderer::get_value_color([focus, bpm_focus])],
        );

        // Gain.
        let gain_focus = focused_field == MusicPanelField::Gain;
        if gain_focus {
            renderer.corners(&self.gain_rect, focus);
        }
        renderer.key_list(
            &conn.state.gain.to_string(),
            &self.gain,
            [focus, gain_focus],
        )
    }
}
