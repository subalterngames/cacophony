use common::{Index, PanelType, State};

/// A popup needs to store the panels that were active before it was enabled, and re-enable them when the popup is disabled.
#[derive(Default)]
pub(crate) struct Popup {
    /// The index of the panel that had focus prior to this popup being enabled.
    focus: usize,
    /// The active panels prior to this popup being enabled.
    panels: Vec<PanelType>,
}

impl Popup {
    /// Enable the panel. Store the state of the active panels. Set the state's active panels.
    pub fn enable(&mut self, state: &mut State, panels: Vec<PanelType>) {
        self.focus = state.focus.get();
        self.panels = state.panels.clone();
        state.panels = panels;
        state.focus = Index::new(0, state.panels.len());
    }

    /// Disable the panel. Set the state's active panels.
    pub fn disable(&self, state: &mut State) {
        state.panels = self.panels.clone();
        state.focus = Index::new(self.focus, self.panels.len());
    }
}
