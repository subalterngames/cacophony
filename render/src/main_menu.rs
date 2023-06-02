use crate::panel::*;

/// Padding between text.
const PADDING: u32 = 3;
/// The color of the panel and the text.
const COLOR: ColorKey = ColorKey::Key;

/// The main menu panel. This panel is always in ghostly not-quite-focus.
pub(crate) struct MainMenu {
    /// The panel background.
    panel: Panel,
    /// The fields.
    fields: [Field; 6],
}

impl MainMenu {
    pub fn new(config: &Ini, text: &Text) -> Self {
        // Get the width of the panel.
        let tracks_panel_width = get_tracks_panel_width(config);
        let window_grid_size = get_window_grid_size(config);
        let width = window_grid_size[0] - tracks_panel_width;

        // Get the panel.
        let panel = Panel::new(
            PanelType::MainMenu,
            MUSIC_PANEL_POSITION,
            [width, MAIN_MENU_HEIGHT],
            text,
        );

        // Get the fields.
        let mut x = panel.position[0] + 1;
        let y = panel.position[1] + 1;
        let help = MainMenu::get_field("MAIN_MENU_HELP", &mut x, y, text);
        let status = MainMenu::get_field("MAIN_MENU_STATUS", &mut x, y, text);
        let input = MainMenu::get_field("MAIN_MENU_INPUT", &mut x, y, text);
        let app = MainMenu::get_field("MAIN_MENU_APP", &mut x, y, text);
        let file = MainMenu::get_field("MAIN_MENU_FILE", &mut x, y, text);
        let stop = MainMenu::get_field("MAIN_MENU_STOP", &mut x, y, text);
        let fields = [help, status, input, app, file, stop];

        Self { panel, fields }
    }

    /// Returns a new field. Moves the x value by the length of the field's title plus padding.
    fn get_field(key: &str, x: &mut u32, y: u32, text: &Text) -> Field {
        let field = Field::new_with_label([*x, y], key, text);
        *x += field.label.as_ref().unwrap().chars().count() as u32 + PADDING;
        field
    }
}

impl Drawable for MainMenu {
    fn update(&self, renderer: &Renderer, _: &State, _: &Conn, _: &Input, _: &Text, _: &OpenFile) {
        self.panel.draw_ex(&COLOR, renderer);
        for field in self.fields.iter() {
            renderer.text(field.label.as_ref().unwrap(), field.position, &COLOR)
        }
    }
}
