use crate::panel::*;
use crate::Popup;
use audio::export::ExportState;
use macroquad::prelude::*;

/// Are we done yet?
pub(crate) struct ExportPanel {
    /// The panel.
    panel: Panel,
    /// The popup handler.
    pub popup: Popup,
    decaying_label: Label,
    writing_label: Label,
}

impl ExportPanel {
    pub fn new(config: &Ini, text: &Text) -> Self {
        let window_grid_size = get_window_grid_size(config);
        let h: u32 = 3;
        let y = window_grid_size[1] / 2 - 1;
        let w = window_grid_size[0] / 4;
        let x = window_grid_size[0] / 2 - w / 2;
        let panel = Panel::new(PanelType::ExportState, [x, y], [w, h], text);
        let popup = Popup::new(PanelType::ExportState);
        let decaying = text.get("EXPORT_PANEL_APPENDING_DECAY");
        let decaying_label = Label::new(
            [
                panel.rect.position[0] + panel.rect.size[0] / 2
                    - decaying.chars().count() as u32 / 2,
                panel.rect.position[1] + 1,
            ],
            decaying,
        );
        let writing = text.get("EXPORT_PANEL_WRITING");
        let writing_label = Label::new(
            [
                panel.rect.position[0] + panel.rect.size[0] / 2
                    - writing.chars().count() as u32 / 2,
                panel.rect.position[1] + 1,
            ],
            writing,
        );
        Self {
            panel,
            popup,
            decaying_label,
            writing_label,
        }
    }
}

impl Drawable for ExportPanel {
    fn update(&self, renderer: &Renderer, _: &State, conn: &Conn, _: &Text, _: &PathsState) {
        self.popup.update(renderer);
        self.panel.update(true, renderer);

        let export_state = conn.export_state.lock();
        match *export_state {
            ExportState::WritingWav {
                total_samples,
                exported_samples,
            } => {
                let samples = format!("{}/{}", exported_samples, total_samples);
                // Draw the string.
                let w = samples.chars().count() as u32;
                let x = self.panel.rect.position[0] + self.panel.rect.size[0] / 2 - w / 2;
                let y = self.panel.rect.position[1] + 1;
                let label = Label {
                    position: [x, y],
                    text: samples,
                };
                renderer.text(&label, &ColorKey::FocusDefault);
            }
            ExportState::AppendingDecay => {
                renderer.text(&self.decaying_label, &ColorKey::FocusDefault);
            }
            ExportState::WritingToDisk => {
                renderer.text(&self.writing_label, &ColorKey::FocusDefault);
            }
            _ => (),
        }
    }
}
