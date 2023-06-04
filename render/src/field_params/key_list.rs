use super::{Label, List};

/// A key `Label` on the left and a `List` on the right.
pub(crate) struct KeyList {
    /// The key label.
    pub key: Label,
    /// The value list.
    pub value: List,
}

impl KeyList {
    /// The key will be on the left and won't be truncated.
    /// The value will be on the right and of width `value_width`
    pub fn new(key: &str, position: [u32; 2], width: u32, value_width: u32) -> Self {
        let key = Label {
            position,
            text: key.to_string(),
        };
        let value_position = [position[0] + width - value_width, position[1]];
        let value = List::new(value_position, value_width);
        Self { key, value }
    }
}
