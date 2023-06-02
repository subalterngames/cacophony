use crate::panel::*;
use text::{get_file_name, get_folder_name, truncate};


/// The open-file dialogue box.
pub struct OpenFilePanel {
    /// The panel.
    panel: Panel,
}

impl OpenFilePanel {
    pub fn new(config: &Ini, text: &Text) -> Self {
        let window_grid_size = get_window_grid_size(config);
        let size = [window_grid_size[0] / 2, window_grid_size[1] / 2];
        let position = [window_grid_size[0] / 2 - size[0] / 2, window_grid_size[1] / 2 - size[1] / 2];
        let panel = Panel::new(PanelType::OpenFile, position, size, text);

        Self { panel }
    }
}

impl Drawable for OpenFilePanel {
    fn update(&self, renderer: &Renderer, state: &State, conn: &Conn, input: &Input, text: &Text) {
        // Draw the panel background.
        self.panel.draw(true, renderer);
        // Draw the working directory.
        let mut x = self.panel.position[0] + 1;
        let mut y = self.panel.position[1] + 1;
    }
}