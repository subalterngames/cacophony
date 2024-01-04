use crate::Renderer;

/// A rectangle has a position and a size.
#[derive(Clone)]
pub(crate) struct RectanglePixel {
    /// The position in pixel units.
    pub position: [f32; 2],
    /// The size in pixel units.
    pub size: [f32; 2],
}

impl RectanglePixel {
    pub fn new_from_u(position: [u32; 2], size: [u32; 2], renderer: &Renderer) -> Self {
        Self::new(
            renderer.grid_to_pixel(position),
            renderer.grid_to_pixel(size),
        )
    }

    pub fn new(position: [f32; 2], size: [f32; 2]) -> Self {
        Self { position, size }
    }
}
