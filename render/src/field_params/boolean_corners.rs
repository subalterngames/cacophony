use super::{Boolean, Rectangle};
use text::Text;

/// A boolean and corners all around.
pub(crate) struct BooleanCorners {
    /// The boolean.
    pub boolean: Boolean,
    /// The corners rect.
    pub corners_rect: Rectangle,
}

impl BooleanCorners {
    pub fn new(key: &str, position: [u32; 2], width: u32, text: &Text) -> Self {
        let boolean = Boolean::new_from_width(key, [position[0] + 1, position[1]], width - 2, text);
        let corners_rect = Rectangle::new(position, [width, 1]);
        Self {
            boolean,
            corners_rect,
        }
    }
}
