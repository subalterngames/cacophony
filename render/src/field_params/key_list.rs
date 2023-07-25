use super::{Label, List};

/// A key `Label` on the left and a `List` on the right.
pub(crate) struct KeyList {
    /// The key label.
    pub key: Label,
    /// The value list.
    pub value: List,
}

impl KeyList {
    /// - The key will be at `position` and won't be truncated.
    /// - The value will at position [`position[0] + width - value_width - 2]` and truncated to `value_width`.
    pub fn new(key: String, position: [u32; 2], width: u32, value_width: u32) -> Self {
        let key = Label::new(position, key);
        let mut x = position[0] + width;
        x = x.checked_sub(value_width).unwrap_or(x);
        let value_position = [x.checked_sub(2).unwrap_or(x), position[1]];
        let value = List::new(value_position, value_width);
        Self { key, value }
    }
}
