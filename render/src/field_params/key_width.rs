use super::util::{get_half_width, KV_PADDING};
use super::{Label, Width};
use text::truncate;

/// A key label and a value width.
pub(crate) struct KeyWidth {
    /// The position and text of the key.
    pub key: Label,
    /// The position and width of the value.
    pub value: Width,
    /// The total width.
    pub width: u32,
}

impl KeyWidth {
    pub fn new(key: &str, position: [u32; 2], value_width: u32) -> Self {
        let width = key.chars().count() as u32 + KV_PADDING + value_width;
        // The key is on the left.
        let key = Label {
            position,
            text: key.to_string(),
        };

        // The value is on the right.
        let value_position = [position[0] + width - value_width, position[1]];
        let value = Width::new(value_position, value_width as usize);

        Self { key, value, width }
    }

    /// The `key` will be at `position` and the `value` will be at a position that tries to fill `width`.
    /// `key` will be truncated and `value` will match `value_width`.
    pub fn new_from_width(key: &str, position: [u32; 2], width: u32, value_width: u32) -> Self {
        let half_width = get_half_width(width);

        // The key is on the left.
        let key = Label {
            position,
            text: truncate(key, half_width, false),
        };

        // The value is on the right.
        let value_position = [position[0] + width - value_width, position[1]];
        let value = Width::new(value_position, value_width as usize);

        Self { key, value, width }
    }

    /// Truncates a value string to `self.width` and converts it into a `Label`.
    pub fn get_value(&self, value: &str) -> Label {
        Label {
            position: self.value.position,
            text: truncate(value, self.value.width, true),
        }
    }
}
