use crate::panel::*;
use common::PanelType;

/// Are we done yet?
#[derive(Default)]
pub(crate) struct ExportPanel {
    /// The previous panels.
    panels: Vec<PanelType>,
    /// The previous focus.
    focus: usize,
    /// If true, the panel was enabled on this frame.
    enabled: bool,
}

impl ExportPanel {
    /// Enable this panel.
    pub fn enable(&mut self, state: &mut State) {
        self.panels = state.panels.clone();
        self.focus = state.focus.get();
        state.panels = vec![PanelType::Export];
        state.focus.set(0);
        self.enabled = true;
    }
}

impl Panel for ExportPanel {
    fn update(
        &mut self,
        state: &mut State,
        conn: &mut Conn,
        _: &Input,
        _: &mut TTS,
        _: &Text,
        _: &mut PathsState,
    ) -> Option<Snapshot> {
        // We're done.
        if !self.enabled && conn.export_state.is_none() {
            state.panels = self.panels.clone();
            state.focus.set(self.focus);
        } else if self.enabled {
            self.enabled = false;
        }
        None
    }
}
