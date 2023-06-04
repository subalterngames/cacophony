use crate::panel::*;
use common::music_panel_field::MusicPanelField;

/// The music panel.
pub(crate) struct MusicPanel {
    /// The panel background.
    panel: Panel,
    /// The name field.
    name: TextWidth,
    /// The BPM field.
    bpm: KeyWidth,
    /// The gain field.
    gain: KeyList,
}

impl MusicPanel {
    pub fn new(config: &Ini, text: &Text) -> Self {
        let mut width = get_tracks_panel_width(config);
        let panel = Panel::new(
            PanelType::Music,
            MUSIC_PANEL_POSITION,
            [width, MUSIC_PANEL_HEIGHT],
            text,
        );
        width -= 2;
        let x = panel.position[0] + 1;
        let mut y = panel.position[1] + 1;

        let name = TextWidth::new([x, y], width);
        y += 1;
        let bpm = KeyWidth::new(&text.get("TITLE_BPM"), [x, y], width, 3);
        y += 1;
        let gain = KeyList::from_width_and_value_width(&text.get("TITLE_GAIN"), [x, y], width, 3);

        // Return.
        Self {
            panel,
            name,
            bpm,
            gain,
        }
    }
}

impl Drawable for MusicPanel {
    fn update(
        &self,
        renderer: &Renderer,
        state: &State,
        conn: &Conn,
        _: &Input,
        _: &Text,
        _: &OpenFile,
    ) {
        // Get the focus,
        let focus = self.panel.has_focus(state);
        // Draw the rect.
        self.panel.draw(focus, renderer);
        // Get the enum value of the focused widget.
        let focused_field = *state.get_music_panel_field();

        // Name.
        renderer.input(
            &state.music.name,
            &self.name,
            [focus, focused_field == MusicPanelField::Name],
        );
        // BPM.
        renderer.key_input(
            &state.music.bpm.to_string(),
            &self.bpm,
            [focus, focused_field == MusicPanelField::BPM],
        );
        // Gain.
        renderer.key_list(
            &conn.state.gain.to_string(),
            &self.gain,
            [focus, focused_field == MusicPanelField::Gain],
        );
    }
}
