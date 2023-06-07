use super::{Label, Width};
use text::truncate;

/// This is basically the same as a `Width` but it knows it can accept truncated text.
/// The text length and the field length might not be the same.
pub(crate) struct TextWidth {
    /// The position and width in grid coordinates.
    pub width: Width,
    /// The value space, which is assumed to be within `width`.
    pub value: Width,
}

impl TextWidth {
    pub fn new(position: [u32; 2], width: u32) -> Self {
        let w = width as usize;
        let width = Width { position, width: w };
        let value = Width {
            position: [position[0], position[1]],
            width: w,
        };
        Self { width, value }
    }

    /// Truncates a value string to `self.width` and converts it into a `Label`.
    pub fn get_value(&self, value: &str) -> Label {
        Label {
            position: self.value.position,
            text: truncate(value, self.value.width, true),
        }
    }
}
