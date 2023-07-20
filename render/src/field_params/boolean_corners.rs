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
    /// - `key` The key label text. It's at at `[position.x + 1, position.y]`.
    /// - `position` The position of the key label in grid units.
    /// - `width` The value label is at `position.x + 1 + (width - 2) - max_boolean_width`.  The corners size is `[width, 1]`.
    /// - `text` The text lookup.
    pub fn new(key: String, position: [u32; 2], width: u32, text: &Text) -> Self {
        let boolean = Boolean::new_from_width(key, [position[0] + 1, position[1]], width - 2, text);
        let corners_rect = Rectangle::new(position, [width, 1]);
        Self {
            boolean,
            corners_rect,
        }
    }
}
