use crate::panel::*;
use input::InputEvent;

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
    pub fn new(config: &Ini, input: &Input, text: &Text) -> Self {
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
        let help = Field::horizontal("MAIN_MENU_HELP", &mut x, y, text);
        let status = Field::horizontal_tooltip("MAIN_MENU_STATUS",  InputEvent::StatusTTS, &mut x, y, input, text);
        let input_field = Field::horizontal_tooltip("MAIN_MENU_INPUT", InputEvent::InputTTS, &mut x, y, input, text);
        let app = Field::horizontal_tooltip("MAIN_MENU_APP", InputEvent::AppTTS, &mut x, y, input, text);
        let file = Field::horizontal_tooltip("MAIN_MENU_FILE", InputEvent::FileTTS, &mut x, y, input, text);
        let stop = Field::horizontal_tooltip("MAIN_MENU_STOP", InputEvent::StopTTS, &mut x, y, input, text);
        let fields = [help, status, input_field, app, file, stop];

        Self { panel, fields }
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
