pub(crate) use crate::drawable::*;
pub(crate) use crate::field_params::*;
pub(crate) use crate::ColorKey;
pub(crate) use crate::Ini;
pub(crate) use common::sizes::*;
pub(crate) use common::PanelType;

pub(crate) struct Panel {
    /// The type of panel.
    panel_type: PanelType,
    /// The title label for the panel.
    title: Label,
    /// The position of the panel in grid units.
    pub position: [u32; 2],
    /// The size of the panel in grid units.
    pub size: [u32; 2],
    /// The size of the title in grid units.
    title_size: [u32; 2],
}

impl Panel {
    pub fn new(panel_type: PanelType, position: [u32; 2], size: [u32; 2], text: &Text) -> Self {
        // Get the title from the panel type.
        let title_key = match panel_type {
            PanelType::MainMenu => "TITLE_MAIN_MENU",
            PanelType::Music => "TITLE_MUSIC",
            PanelType::OpenFile => "TITLE_OPEN_FILE",
            PanelType::PianoRoll => "TITLE_PIANO_ROLL",
            PanelType::Tracks => "TITLE_TRACKS",
        };
        let title = text.get(title_key);
        let title_position = [position[0] + 2, position[1]];
        let title_size = [title.chars().count() as u32, 1];
        Self {
            panel_type,
            title: Label {
                position: title_position,
                text: title,
            },
            position,
            size,
            title_size,
        }
    }

    /// Draw an empty panel. The color will be defined by the value of `focus`.
    pub fn update(&self, focus: bool, renderer: &Renderer) {
        let color: ColorKey = if focus {
            ColorKey::FocusDefault
        } else {
            ColorKey::NoFocus
        };
        self.draw_ex(&color, renderer);
    }

    /// Draw an empty panel. The border and title text will be an explicitly defined color.
    pub fn draw_ex(&self, color: &ColorKey, renderer: &Renderer) {
        renderer.rectangle(self.position, self.size, &ColorKey::Background);
        renderer.border(self.position, self.size, color);
        renderer.rectangle(self.title.position, self.title_size, &ColorKey::Background);
        renderer.text(&self.title, color);
    }

    /// Returns true if this panel has focus.
    pub fn has_focus(&self, state: &State) -> bool {
        self.panel_type == state.panels[state.focus.get()]
    }
}
