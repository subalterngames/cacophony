/// A rectangle has a position and a size.
pub(crate) struct Rectangle {
    /// The position in grid units.
    pub position: [u32; 2],
    /// The size in grid units.
    pub size: [u32; 2],
}

impl Rectangle {
    pub fn new(position: [u32; 2], size: [u32; 2]) -> Self {
        Self { position, size }
    }
}
