use crate::panel::*;

/// Are we done yet?
pub(crate) struct ExportPanel {
    panel: Panel,
}

impl ExportPanel {
    pub fn new(config: &Ini, text: &Text) -> Self {
        let window_grid_size = get_window_grid_size(config);
        let h: u32 = 3;
        let y = window_grid_size[1] / 2 - 1;
        let w = window_grid_size[0] / 2;
        let x = window_grid_size[0] / 2 - w / 2;
        let panel = Panel::new(PanelType::Export, [x, y], [w, h], text);
        Self { panel }
    }
}

impl Drawable for ExportPanel {
    fn update(
        &self,
        renderer: &Renderer,
        _: &State,
        conn: &Conn,
        _: &Input,
        _: &Text,
        _: &OpenFile,
    ) {
        self.panel.update(true, renderer);

        // Get the string.
        let export_state = conn.export_state.unwrap();
        let mut s = export_state.exported.to_string();
        s.push('/');
        s.push_str(&export_state.samples.to_string());

        // Draw the string.
        let w = s.chars().count() as u32;
        let x = self.panel.rect.position[0] + self.panel.rect.position[2] / 2 + w / 2;
        let y = self.panel.rect.position[1];
        let label = Label {
            position: [x, y],
            text: s,
        };
        renderer.text(&label, &ColorKey::FocusDefault);
    }
}
