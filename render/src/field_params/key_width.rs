use super::util::KV_PADDING;
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
    /// The `key` will be at `position`. The value will be at `position.x + key_width + KV_PADDING + value_width`.
    pub fn new(key: String, position: [u32; 2], value_width: u32) -> Self {
        let width = key.chars().count() as u32 + KV_PADDING + value_width;
        // The key is on the left.
        let key = Label::new(position, key);

        // The value is on the right.
        let value_position = [position[0] + width - value_width, position[1]];
        let value = Width::new(value_position, value_width as usize);

        Self { key, value, width }
    }

    /// The `key` will be at `position` and the `value` will be at a position that tries to fill `width`.
    /// `key` will be truncated and `value` will match `value_width`.
    pub fn new_from_width(key: &str, position: [u32; 2], width: u32, value_width: u32) -> Self {
        let half_width = Self::get_half_width(width);

        // The key is on the left.
        let key = Label::new(position, truncate(key, half_width, false));

        // The value is on the right.
        let value_position = [position[0] + width - value_width, position[1]];
        let value = Width::new(value_position, value_width as usize);

        Self { key, value, width }
    }

    /// Truncates a value string to `self.width` and converts it into a `Label`.
    pub fn get_value(&self, value: &str) -> Label {
        Label::new(self.value.position, truncate(value, self.value.width, true))
    }

    /// Returns half of the width, or slightly less than half.
    /// The half-width is `width / 2` for odd numbers `and `width / 2 - 1)` for even numbers.
    fn get_half_width(width: u32) -> usize {
        let mut half_width = width / 2;
        if width % 2 == 0 {
            half_width -= 1;
        }
        half_width as usize
    }
}
