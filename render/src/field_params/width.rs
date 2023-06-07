use super::Label;
use text::truncate;

/// Not a label... but the IDEA of a label.
/// This holds space for text.
pub(crate) struct Width {
    /// The position in grid coordinates.
    pub position: [u32; 2],
    /// The width of the space being held.
    pub width: usize,
    /// The width as a u32.
    pub width_u32: u32,
}

impl Width {
    pub fn new(position: [u32; 2], width: usize) -> Self {
        Self {
            position,
            width,
            width_u32: width as u32,
        }
    }

    /// Converts this `Width` into a `Label` with truncated text.
    pub fn to_label(&self, value: &str) -> Label {
        Label {
            position: self.position,
            text: truncate(value, self.width, true),
        }
    }
}
