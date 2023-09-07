use crate::Renderer;
use common::{PanelType, State};

/// A popup tries to capture the backround texture when its panel is first enabled.
pub(crate) struct Popup {
    /// My panel type.
    panel_type: PanelType,
    /// If true, capture the screen.
    captured_screen: bool,
}

impl Popup {
    pub(crate) fn new(panel_type: PanelType) -> Self {
        Self {
            panel_type,
            captured_screen: false,
        }
    }

    /// Update and draw.
    pub(crate) fn update(&self, renderer: &Renderer) {
        renderer.background();
    }

    /// Update the popup. Maybe request a screen capture.
    ///
    /// - If the corresponding panel was enabled on this frame, set the background texture.
    /// - If the corresponding panel was disabled on thie frame, un-set the background texture.
    pub(crate) fn late_update(&mut self, state: &State, renderer: &mut Renderer) {
        // This panel is active.
        if state.panels.contains(&self.panel_type) {
            if !self.captured_screen {
                self.captured_screen = true;
                // I don't have a background and I need one.
                renderer.screen_capture();
            }
        } else {
            self.captured_screen = false;
        }
    }
}
