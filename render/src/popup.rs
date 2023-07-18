use crate::Renderer;
use common::{PanelType, State};
use macroquad::prelude::*;

/// A popup tries to capture the backround texture when its panel is first enabled.
pub(crate) struct Popup {
    /// My panel type.
    panel_type: PanelType,
    /// The background texture (screen capture).
    pub background: Option<(Texture2D, DrawTextureParams)>,
}

impl Popup {
    pub(crate) fn new(panel_type: PanelType) -> Self {
        Self {
            panel_type,
            background: None,
        }
    }

    /// Update and draw.
    pub(crate) fn update(&self, renderer: &Renderer) {
        if let Some(background) = &self.background {
            renderer.texture_ex(background.0, [0, 0], background.1.clone());
        }
    }

    /// Update the popup. Maybe request a screen capture.
    ///
    /// - If the corresponding panel was enabled on this frame, set the background texture.
    /// - If the corresponding panel was disabled on thie frame, un-set the background texture.
    pub(crate) fn late_update(&mut self, state: &State, renderer: &Renderer) {
        // This panel is active.
        if state.panels.contains(&self.panel_type) {
            // I don't have a background and I need one.
            if self.background.is_none() {
                self.background = Some(renderer.screen_capture());
            }
        }
        // I don't need a background anymore.
        else if self.background.is_some() {
            self.background = None;
        }
    }
}
