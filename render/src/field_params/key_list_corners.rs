use super::{KeyList, Rectangle};

/// A key `Label` on the left, a `List` on the right, and corners all around.
pub(crate) struct KeyListCorners {
    /// The key-list pair.
    pub key_list: KeyList,
    /// The corners rect.
    pub corners_rect: Rectangle,
}

impl KeyListCorners {
    /// The key will be on the left and won't be truncated.
    /// The value will be on the right and of width `value_width`
    pub fn new(key: &str, position: [u32; 2], width: u32, value_width: u32) -> Self {
        let key_list = KeyList::new(key, [position[0] + 1, position[1]], width - 2, value_width);
        let corners_rect = Rectangle::new(position, [width, 1]);
        Self {
            key_list,
            corners_rect,
        }
    }
}
