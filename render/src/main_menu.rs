use crate::panel::*;
use input::InputEvent;
use tooltip::get_tooltip;

/// The color of the panel and the text.
const COLOR: ColorKey = ColorKey::Key;

/// The main menu panel. This panel is always in ghostly not-quite-focus.
pub(crate) struct MainMenu {
    /// The panel background.
    panel: Panel,
    /// The fields.
    labels: [Label; 6],
}

impl MainMenu {
    pub fn new(config: &Ini, input: &Input, text: &Text) -> Self {
        // Get the width of the panel.
        let tracks_panel_width = get_tracks_panel_width(config);
        let window_grid_size = get_window_grid_size(config);
        let width = window_grid_size[0] - tracks_panel_width;

        let position = [
            MUSIC_PANEL_POSITION[0] + tracks_panel_width,
            MUSIC_PANEL_POSITION[1],
        ];

        // Get the panel.
        let panel = Panel::new(
            PanelType::MainMenu,
            position,
            [width, MAIN_MENU_HEIGHT],
            text,
        );

        // Get the fields.
        let mut x = panel.position[0] + 1;
        let y = panel.position[1] + 1;
        let help = Self::label_from_key("MAIN_MENU_HELP", &mut x, y, text);
        x += 4;
        let status = Self::tooltip(
            "MAIN_MENU_STATUS",
            InputEvent::StatusTTS,
            &mut x,
            y,
            input,
            text,
        );
        let input_field = Self::tooltip(
            "MAIN_MENU_INPUT",
            InputEvent::InputTTS,
            &mut x,
            y,
            input,
            text,
        );
        let app = Self::tooltip("MAIN_MENU_APP", InputEvent::AppTTS, &mut x, y, input, text);
        let file = Self::tooltip(
            "MAIN_MENU_FILE",
            InputEvent::FileTTS,
            &mut x,
            y,
            input,
            text,
        );
        let stop = Self::tooltip(
            "MAIN_MENU_STOP",
            InputEvent::StopTTS,
            &mut x,
            y,
            input,
            text,
        );
        let fields = [help, status, input_field, app, file, stop];

        Self {
            panel,
            labels: fields,
        }
    }

    fn label(key: String, x: &mut u32, y: u32) -> Label {
        let width = key.chars().count() as u32;
        let position = [*x, y];
        *x += width;
        Label {
            text: key,
            position,
        }
    }

    fn label_from_key(key: &str, x: &mut u32, y: u32, text: &Text) -> Label {
        Self::label(text.get(key), x, y)
    }

    fn tooltip(
        key: &str,
        event: InputEvent,
        x: &mut u32,
        y: u32,
        input: &Input,
        text: &Text,
    ) -> Label {
        let text = get_tooltip(key, &[event], input, text);
        let width = key.chars().count() as u32;
        let position = [*x, y];
        *x += width;
        Label { text, position }
    }
}

impl Drawable for MainMenu {
    fn update(&self, renderer: &Renderer, _: &State, _: &Conn, _: &Input, _: &Text, _: &OpenFile) {
        self.panel.draw_ex(&COLOR, renderer);
        for label in self.labels.iter() {
            renderer.text(label, &COLOR)
        }
    }
}
