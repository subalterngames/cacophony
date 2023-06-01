use crate::panel::*;
use common::hashbrown::HashMap;
use common::music_panel_field::MusicPanelField;
use text::truncate;

/// Padding between text.
const PADDING: u32 = 3;

/// The main menu panel.
pub(crate) struct MainMenu {
    /// The panel background.
    panel: Panel,
    /// The fields.
    fields: [Field; 7],
}

impl MainMenu {
    pub fn new(config: &Ini, text: &Text) -> Self {
        // Get the width of the panel.
        let tracks_panel_width = get_tracks_panel_width(config);
        let window_grid_size = get_window_grid_size(config);
        let width = window_grid_size[0] - tracks_panel_width;

        // Get the panel.
        let panel = Panel::new(PanelType::MainMenu, MUSIC_PANEL_POSITION, [width, MAIN_MENU_HEIGHT], text);

        // Get the fields.
        let mut x = panel.position[0] + 1;
        let y = panel.position[1] + 1;
        // Help.
        let help = Field::new_with_label([x, y], "MAIN_MENU_HELP", text);
        x += help.label.unwrap().chars().count() as u32 + PADDING;

    }
}