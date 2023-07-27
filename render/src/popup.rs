use crate::Renderer;
use common::{PanelType, State};
use image::imageops::{resize, FilterType};
use image::RgbaImage;
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
            renderer.texture_ex(&background.0, [0, 0], background.1.clone());
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
                let (mut background, params) = renderer.screen_capture();
                // This can happen on Linux. Resize the texture.
                if background.width() > renderer.window_pixel_size[0]
                    || background.height() > renderer.window_pixel_size[1]
                {
                    // Convert the texture to image data.
                    let image_data = background.get_texture_data().bytes;
                    // Convert the image data to an image buffer.
                    let mut image = RgbaImage::from_vec(
                        background.width() as u32,
                        background.height() as u32,
                        image_data,
                    )
                    .unwrap();
                    // Resize the image.
                    image = resize(
                        &image,
                        renderer.window_pixel_size[0] as u32,
                        renderer.window_pixel_size[1] as u32,
                        FilterType::Lanczos3,
                    );
                    // Convert to a texture again.
                    background = Texture2D::from_rgba8(
                        image.width() as u16,
                        image.height() as u16,
                        image.as_raw(),
                    );
                }
                self.background = Some((background, params));
            }
        }
        // I don't need a background anymore.
        else if self.background.is_some() {
            self.background = None;
        }
    }
}
