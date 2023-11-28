use super::{Rectangle, RectanglePixel};
use crate::Renderer;

/// A background rectangle and a border rectangle, both in pixel units.
#[derive(Clone)]
pub(crate) struct PanelBackground {
    /// The background.
    pub background: RectanglePixel,
    /// The border.
    pub border: RectanglePixel,
    /// The rectangle in grid units.
    pub grid_rect: Rectangle,
}

impl PanelBackground {
    pub fn new(position: [u32; 2], size: [u32; 2], renderer: &Renderer) -> Self {
        let background = RectanglePixel::new(
            renderer.grid_to_pixel(position),
            renderer.grid_to_pixel(size),
        );
        let border = renderer.get_border_rect(position, size);
        let grid_rect = Rectangle::new(position, size);
        Self {
            background,
            border,
            grid_rect,
        }
    }

    /// Adjust the size by a delta in grid units.
    pub fn resize_by(&mut self, delta: [u32; 2], renderer: &Renderer) {
        self.grid_rect.size[0] += delta[0];
        self.grid_rect.size[1] += delta[1];
        let delta = renderer.grid_to_pixel(delta);
        self.background.size[0] += delta[0];
        self.background.size[1] += delta[1];
        self.border.size[0] += delta[0];
        self.border.size[1] += delta[1];
    }
}
