use crate::panel::*;
use common::hashbrown::HashMap;
use common::music_panel_field::MusicPanelField;
use text::truncate;

/// The music panel.
pub(crate) struct MusicPanel {
    /// The panel background.
    panel: Panel,
    /// The position of each field in grid units.
    field_positions: HashMap<MusicPanelField, [u32; 2]>,
    /// The width of each field in grid units.
    field_width: u32,
    /// The maximum length of the name text.
    max_name_length: usize,
    /// The width of a key-value pair.
    kv_width: usize,
    /// The title of the BPM field.
    bpm_title: String,
    /// The title of the gain field.
    gain_title: String,
}

impl MusicPanel {
    pub fn new(config: &Ini, text: &Text) -> Self {
        let width = get_tracks_panel_width(config);
        let panel = Panel::new(
            PanelType::Music,
            MUSIC_PANEL_POSITION,
            [width, MUSIC_PANEL_HEIGHT],
            text,
        );
        // Define x, y coordinates for the fields.
        let mut field_positions = HashMap::new();
        let x = panel.position[0] + 1;
        let y = panel.position[1] + 1;
        field_positions.insert(MusicPanelField::Name, [x, y]);
        field_positions.insert(MusicPanelField::BPM, [x, y + 1]);
        field_positions.insert(MusicPanelField::Gain, [x, y + 2]);

        // Define the size of the fields.
        let width = panel.size[0] - 2;
        let field_width = width - 2;
        let max_name_length = field_width as usize - 4;
        let kv_width: usize = (width / 2 - 1) as usize;

        let bpm_title = truncate(&text.get("TITLE_BPM"), kv_width, false);
        let gain_title = truncate(&text.get("TITLE_GAIN"), kv_width, false);

        // Return.
        Self {
            panel,
            field_positions,
            field_width,
            max_name_length,
            kv_width,
            bpm_title,
            gain_title,
        }
    }
}

impl Drawable for MusicPanel {
    fn update(&self, renderer: &Renderer, state: &State, conn: &Conn, _: &Input, _: &Text) {
        // Get the focus,
        let focus = self.panel.has_focus(state);
        // Draw the rect.
        self.panel.draw(focus, renderer);
        // Get the enum value of the focused widget.
        let focused_field = state.get_music_panel_field();

        for field in self.field_positions.iter() {
            let field_focus = focused_field == field.0;
            match field.0 {
                MusicPanelField::Name => {
                    renderer.input(
                        &truncate(&state.music.name, self.max_name_length, true),
                        *field.1,
                        self.field_width,
                        [focus, field_focus],
                    );
                }
                MusicPanelField::BPM => renderer.key_input(
                    &self.bpm_title,
                    &truncate(&state.music.bpm.to_string(), self.kv_width, true),
                    *field.1,
                    self.field_width,
                    3,
                    [focus, field_focus],
                ),
                MusicPanelField::Gain => {
                    if field_focus {
                        renderer.corners(*field.1, [self.field_width, 1], focus);
                    }
                    let w = self.field_width - 2;
                    renderer.key_list(
                        &self.gain_title,
                        &truncate(
                            conn.state.gain.to_string().as_str(),
                            self.kv_width - 2,
                            true,
                        ),
                        *field.1,
                        w,
                        3,
                        [focus, field_focus],
                    );
                }
            }
        }
    }
}
