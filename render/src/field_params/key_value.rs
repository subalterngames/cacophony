use super::util::get_half_width;
use text::truncate;
use super::{Label, Width};

/// A key-value pair of labels.
pub(crate) struct KeyValue {
    /// The position and text of the key.
    pub key: Label,
    /// The position and width of the value.
    pub value: Width,
    /// The total width.
    pub width: u32,
}

impl KeyValue {
    /// The `key` will be at `position` and the `value` will be at a position that tries to fill `width`.
    /// Both the `key` and `value` will be truncated to fit in the `width`.
    pub fn from_width(key: &str, value: &str, position: [u32; 2], width: u32) -> Self {
        let half_width = get_half_width(width);

        // The key is on the left.
        let key = Label { position, text: &truncate(key, half_width, true )};

        // The value is on the right.
        let value_text = &truncate(value, half_width, false);
        let value_position = [position[0] + width - value_text.chars().count() as u32, position[1]];
        let value = Width { position: value_position, width: half_width};
        
        Self {key, value, width}
    }

    /// Same as above, but leave room for corner borders.
    pub fn from_border_width(key: &str, value: &str, position: [u32; 2], width: u32) -> Self {
        let mut kv = Self::from_width(key, value, [position[0] + 1, position[1]], width - 2);
        kv.width = width;
        kv
    }

    /// The `key` will be at `position` and the `value` will be at a position that tries to fill `width`.
    /// `key` will be truncated and `value` will match `value_width`.
    pub fn from_width_and_value_width(key: &str, position: [u32; 2], width: u32, value_width: u32) -> Self {
        let half_width = get_half_width(width);

        // The key is on the left.
        let key = Label { position, text: &truncate(key, half_width, true )};

        // The value is on the right.
        let value_position = [position[0] + width - value_width, position[1]];
        let value = Width { position: value_position, width: value_width};
        
        Self {key, value, width}
    }

    /// Truncates a value string to `self.width` and converts it into a `Label`.
    pub fn get_value(&self, value: &str) -> Label {
        Label { position: self.value.position, text: truncate(&value, self.value, true)}
    }
}