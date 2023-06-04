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
    /// The value on the right will be of a truncated width to fit within the `width`.
    pub fn from_width(key: &str, position: [u32; 2], width: u32) -> Self {
        let key_width = key.chars().count() as u32;
        let value_width: u32 = match width.checked_sub(key_width) {
            Some(v) => match v.check_sub(1) {
                Some(v) => v,
                None => 3
            }
            None => 3
        };
        let value_position = [position[0] + width - value_width, position[1]];
        let key = Label { position, text: key.to_string() };
        let value = List::new(value_position, value_width);
        Self { key, value }
    }

    /// The key will be on the left and won't be truncated. 
    /// The value will be on the right and of width `value_width`
    pub fn from_width_and_value_width(key: &str, position: [u32; 2], width: u32, value_width: u32) -> Self {
        let key = Label { position, text: key.to_string() };
        let value_position = [position[0] + width - value_width, position[1]];
        let value = List::new(value_position, value_width);
        Self { key, value }
    }

    /// Truncates a value string to `self.width` and converts it into a `Label`.
    pub fn get_value(&self, value: &str) -> Label {
        self.value.get_value(value)
    }
}