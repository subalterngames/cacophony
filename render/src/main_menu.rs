use crate::panel::*;
use input::InputEvent;

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
    /// The positions of the separator lines.
    separator_positions: [[u32; 2]; 2],
}

impl MainMenu {
    pub fn new(
        config: &Ini,
        input: &Input,
        text: &mut Text,
        remote_version: Option<String>,
    ) -> Self {
        // Get the width of the panel.
        let width = get_main_menu_width(config);

        let position = get_main_menu_position(config);

        // Get the panel.
        let mut panel = Panel::new(
            PanelType::MainMenu,
            position,
            [width, MAIN_MENU_HEIGHT],
            text,
        );
        // Add an update notice to the title.
        if let Some(remote_version) = remote_version {
            let update = text.get_with_values("MAIN_MENU_UPDATE", &[&remote_version]);
            panel.title.label.text.push_str("   ");
            panel.title.label.text.push_str(&update);
            panel.title.rect.size[0] += update.chars().count() as u32 + 3;
        }
        let title_changes = LabelRectangle::new(
            panel.title.label.position,
            format!("*{}", panel.title.label.text),
        );

        // Get the fields.
        let mut x = panel.rect.position[0] + 2;
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
        x += 1;
        let separator_help = [x, y];
        x += 2;
        let x0 = x;
        let links = Self::tooltip(
            "MAIN_MENU_ONLINE",
            InputEvent::EnableLinksPanel,
            &mut x,
            y,
            input,
            text,
        );
        x = x0 + links.text.chars().count() as u32 + 1;
        let separator_links = [x, y];
        let fields = [help, status, input_field, app, file, stop, links];
        let separator_positions = [separator_help, separator_links];

        Self {
            panel,
            labels: fields,
            title_changes,
            separator_positions,
        }
    }

    fn label(key: String, x: &mut u32, y: u32) -> Label {
        let width = key.chars().count() as u32;
        let position = [*x, y];
        *x += width;
        Label::new(position, key)
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
        text: &mut Text,
    ) -> Label {
        let tooltip = text.get_tooltip(key, &[event], input).seen;
        let width = key.chars().count() as u32;
        let position = [*x, y];
        *x += width;
        Label::new(position, tooltip)
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
        _: &SharedExporter,
    ) {
        self.panel.update_ex(&COLOR, renderer);
        if state.unsaved_changes {
            renderer.rectangle(&self.title_changes.rect, &ColorKey::Background);
            renderer.text(&self.title_changes.label, &COLOR);
        }
        for label in self.labels.iter() {
            renderer.text(label, &COLOR)
        }
        for position in self.separator_positions {
            renderer.vertical_line_separator(position, &COLOR)
        }
    }
}
