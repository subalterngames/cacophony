/// A rectangle has a position and a size.
#[derive(Clone)]
pub(crate) struct RectanglePixel {
    /// The position in pixel units.
    pub position: [f32; 2],
    /// The size in pixel units.
    pub size: [f32; 2],
}

impl RectanglePixel {
    pub fn new(position: [f32; 2], size: [f32; 2]) -> Self {
        Self { position, size }
    }
}
