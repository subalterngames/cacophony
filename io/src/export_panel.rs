use crate::panel::*;
use common::PanelType;

/// Are we done yet?
#[derive(Default)]
pub(crate) struct ExportPanel {
    /// The previous panels.
    panels: Vec<PanelType>,
    /// The previous focus.
    focus: usize,
}

impl ExportPanel {
    /// Enable this panel.
    pub fn enable(&mut self, state: &mut State, panels: &[PanelType], focus: usize) {
        self.panels = panels.to_vec();
        self.focus = focus;
        state.panels = vec![PanelType::ExportState];
        state.focus.set(0);
    }
}

impl Panel for ExportPanel {
    fn update(
        &mut self,
        state: &mut State,
        conn: &mut Conn,
        _: &Input,
        _: &mut TTS,
        _: &mut Text,
        _: &mut PathsState,
        _: &mut SharedExporter,
    ) -> Option<Snapshot> {
        // We're done.
        if conn.export_state.is_none() {
            state.panels = self.panels.clone();
            state.focus.set(self.focus);
        }
        None
    }

    fn on_disable_abc123(&mut self, _: &mut State, _: &mut SharedExporter) {}

    fn update_abc123(
        &mut self,
        _: &mut State,
        _: &Input,
        _: &mut SharedExporter,
    ) -> (Option<Snapshot>, bool) {
        (None, false)
    }

    fn allow_alphanumeric_input(&self, _: &State, _: &SharedExporter) -> bool {
        false
    }
}
