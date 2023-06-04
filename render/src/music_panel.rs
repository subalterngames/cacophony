use crate::panel::*;
use common::music_panel_field::MusicPanelField;
use text::truncate;

/// The music panel.
pub(crate) struct MusicPanel {
    /// The panel background.
    panel: Panel,
    /// The BPM field.
    bpm: KeyValue,
    /// The gain field.
    gain: KeyValue,
    /// The maximum length of the name text.
    max_name_length: usize,
    /// The width of a key-value pair.
    kv_width: usize,
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

        // TODO
        y += 1;

        let bpm = KeyValue::from_width_and_value_width(&text.get("TITLE_BPM"), [x, y], width, 3);
        y += 1;
        let gain = KeyValue::from_width_and_value_width(&text.get("TITLE_GAIN"), [x, y], width, 3);


        // Define the size of the fields.
        let width = panel.size[0] - 2;
        let field_width = width - 2;
        let max_name_length = field_width as usize - 4;

        // Return.
        Self {
            panel,
            bpm,
            gain,
            max_name_length,
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
        let focused_field = state.get_music_panel_field();

        for field in self.fields.iter() {
            let field_focus = focused_field == field.0;
            match field.0 {
                MusicPanelField::Name => {
                    renderer.input(
                        &truncate(&state.music.name, self.max_name_length, true),
                        field.1.position,
                        self.field_width,
                        [focus, field_focus],
                    );
                }
                MusicPanelField::BPM => renderer.key_input(
                    field.1.label.as_ref().unwrap(),
                    &truncate(&state.music.bpm.to_string(), self.kv_width, true),
                    field.1.position,
                    self.field_width,
                    3,
                    [focus, field_focus],
                ),
                MusicPanelField::Gain => {
                    if field_focus {
                        renderer.corners(field.1.position, [self.field_width, 1], focus);
                    }
                    let w = self.field_width - 2;
                    renderer.key_list(
                        field.1.label.as_ref().unwrap(),
                        &truncate(
                            conn.state.gain.to_string().as_str(),
                            self.kv_width - 2,
                            true,
                        ),
                        field.1.position,
                        w,
                        3,
                        [focus, field_focus],
                    );
                }
            }
        }
    }
}
