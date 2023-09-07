use crate::{panel::*, popup::Popup};
use input::InputEvent;

const LABEL_PADDING: u32 = 8;
const LABEL_COLOR: ColorKey = ColorKey::Value;

/// Do you really want to quit?
pub(crate) struct QuitPanel {
    /// The panel.
    panel: Panel,
    /// Yes and no.
    labels: [Label; 2],
    /// The popup.
    pub popup: Popup,
}

impl QuitPanel {
    pub fn new(config: &Ini, text: &mut Text, input: &Input) -> Self {
        // Get the width of the panel.
        let yes = text
            .get_tooltip("QUIT_PANEL_YES", &[InputEvent::QuitPanelYes], input)
            .seen;
        let no = text
            .get_tooltip("QUIT_PANEL_NO", &[InputEvent::QuitPanelNo], input)
            .seen;
        let yes_w = yes.chars().count() as u32 + LABEL_PADDING;
        let w = yes_w + no.chars().count() as u32 + 4;
        let h = 3;
        // Get the position of the panel.
        let window_grid_size = get_window_grid_size(config);
        let x = window_grid_size[0] / 2 - w / 2;
        let y = window_grid_size[1] / 2 - h / 2;
        // Define the panel.
        let panel = Panel::new(PanelType::Quit, [x, y], [w, h], text);
        // Define the labels.
        let yes = Label::new([x + 2, y + 1], yes);
        let no = Label::new([yes.position[0] + yes_w, yes.position[1]], no);
        let labels = [yes, no];
        let popup = Popup::new(PanelType::Quit);
        Self {
            panel,
            labels,
            popup,
        }
    }
}

impl Drawable for QuitPanel {
    fn update(
        &self,
        renderer: &Renderer,
        _: &State,
        _: &Conn,
        _: &Text,
        _: &PathsState,
        _: &SharedExporter,
    ) {
        self.popup.update(renderer);
        self.panel.update(true, renderer);
        renderer.text(&self.labels[0], &LABEL_COLOR);
        renderer.text(&self.labels[1], &LABEL_COLOR);
    }
}
