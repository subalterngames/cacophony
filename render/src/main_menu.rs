use crate::panel::*;
use common::VERSION;
use input::InputEvent;
use text::Tooltips;

/// The color of the panel and the text.
const COLOR: ColorKey = ColorKey::Key;

/// The main menu panel. This panel is always in ghostly not-quite-focus.
pub(crate) struct MainMenu {
    /// The panel background.
    panel: Panel,
    /// The title if there are unsaved changes.
    title_changes: LabelRectangle,
    /// The field labels and the version label.
    labels: [Label; 7],
}

impl MainMenu {
    pub fn new(config: &Ini, input: &Input, text: &Text) -> Self {
        // Get the width of the panel.
        let width = get_main_menu_width(config);

        let position = get_main_menu_position(config);

        // Get the panel.
        let panel = Panel::new(
            PanelType::MainMenu,
            position,
            [width, MAIN_MENU_HEIGHT],
            text,
        );
        let title_changes = LabelRectangle::new(
            panel.title.label.position,
            format!("*{}", panel.title.label.text),
        );

        let mut tooltips = Tooltips::new(text);

        // Get the fields.
        let mut x = panel.rect.position[0] + 1;
        let y = panel.rect.position[1] + 1;
        let help = Self::label_from_key("MAIN_MENU_HELP", &mut x, y, text);
        x += 4;
        let status = Self::tooltip(
            "MAIN_MENU_STATUS",
            InputEvent::StatusTTS,
            &mut x,
            y,
            input,
            text,
            &mut tooltips
        );
        let input_field = Self::tooltip(
            "MAIN_MENU_INPUT",
            InputEvent::InputTTS,
            &mut x,
            y,
            input,
            text,
            &mut tooltips
        );
        let app = Self::tooltip("MAIN_MENU_APP", InputEvent::AppTTS, &mut x, y, input, text, &mut tooltips);
        let file = Self::tooltip(
            "MAIN_MENU_FILE",
            InputEvent::FileTTS,
            &mut x,
            y,
            input,
            text,
            &mut tooltips
        );
        let stop = Self::tooltip(
            "MAIN_MENU_STOP",
            InputEvent::StopTTS,
            &mut x,
            y,
            input,
            text,
            &mut tooltips
        );
        let version = Label {
            position: [
                panel.rect.position[0] + panel.rect.size[0] - VERSION.chars().count() as u32 - 1,
                y,
            ],
            text: VERSION.to_string(),
        };
        let fields = [help, status, input_field, app, file, stop, version];

        Self {
            panel,
            labels: fields,
            title_changes,
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
        tooltips: &mut Tooltips
    ) -> Label {
        let text = tooltips.get_tooltip(key, &[event], input, text).seen;
        let width = key.chars().count() as u32;
        let position = [*x, y];
        *x += width;
        Label { text, position }
    }
}

impl Drawable for MainMenu {
    fn update(
        &self,
        renderer: &Renderer,
        state: &State,
        _: &Conn,
        _: &Input,
        _: &Text,
        _: &PathsState,
        _: &Exporter,
    ) {
        self.panel.update_ex(&COLOR, renderer);
        if state.unsaved_changes {
            renderer.rectangle(&self.title_changes.rect, &ColorKey::Background);
            renderer.text(&self.title_changes.label, &COLOR);
        }
        for label in self.labels.iter() {
            renderer.text(label, &COLOR)
        }
    }
}
