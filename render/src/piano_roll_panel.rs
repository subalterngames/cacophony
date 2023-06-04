use crate::panel::*;
mod top_bar;
use top_bar::TopBar;

/// Draw the piano roll panel.
pub struct PianoRollPanel {
    /// The panel.
    panel: Panel,
    /// Data for the top bar sub-panel.
    top_bar: TopBar,
}

impl PianoRollPanel {
    pub fn new(config: &Ini, text: &Text) -> Self {
        let piano_roll_panel_position = get_piano_roll_panel_position(config);
        let piano_roll_panel_size = get_piano_roll_panel_size(config);
        let panel = Panel::new(
            PanelType::PianoRoll,
            piano_roll_panel_position,
            piano_roll_panel_size,
            text,
        );
        let top_bar = TopBar::new(config, text);
        Self { panel, top_bar }
    }
}

impl Drawable for PianoRollPanel {
    fn update(
        &self,
        renderer: &Renderer,
        state: &State,
        conn: &Conn,
        input: &Input,
        text: &Text,
        open_file: &OpenFile,
    ) {
        let focus = self.panel.has_focus(state);
        self.panel.update(focus, renderer);
        self.top_bar.update(state, renderer, text, focus);
    }
}
