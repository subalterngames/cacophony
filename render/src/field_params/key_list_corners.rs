use super::{KeyList, Rectangle};

/// A key `Label` on the left, a `List` on the right, and corners all around.
pub(crate) struct KeyListCorners {
    /// The key-list pair.
    pub key_list: KeyList,
    /// The corners rect.
    pub corners_rect: Rectangle,
}

impl KeyListCorners {
    /// - The `key` is at `position[0] + 1`.
    /// - The `value` is at a position that tries to fill `width - 2` truncated to `value_width`.
    /// - The `corners_rect` is at `position` and is size `[width, 1]`.
    pub fn new(key: String, position: [u32; 2], width: u32, value_width: u32) -> Self {
        let key_list = KeyList::new(
            key,
            [position[0] + 1, position[1]],
            width.checked_sub(2).unwrap_or(width),
            value_width,
        );
        let corners_rect = Rectangle::new(position, [width, 1]);
        Self {
            key_list,
            corners_rect,
        }
    }
}
