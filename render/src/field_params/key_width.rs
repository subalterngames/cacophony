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
        let value = Self::get_value_width(position, width, value_width);

        Self { key, value, width }
    }

    /// The `key` will be at `position` and the `value` will be at a position that tries to fill `width`.
    /// `key` will be truncated and `value` will match `value_width`.
    pub fn new_from_width(key: &str, position: [u32; 2], width: u32, value_width: u32) -> Self {
        let half_width = Self::get_half_width(width);

        // The key is on the left.
        let key = Label::new(position, truncate(key, half_width, false));

        // The value is on the right.
        let value = Self::get_value_width(position, width, value_width);

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
        if width % 2 == 0 && half_width > 0 {
            half_width -= 1;
        }
        half_width as usize
    }

    /// Returns the value `Width`.
    fn get_value_width(position: [u32; 2], width: u32, value_width: u32) -> Width {
        let x = position[0] + width;
        let value_position = [x.checked_sub(value_width).unwrap_or(x), position[1]];
        Width::new(value_position, value_width as usize)
    }
}

#[cfg(test)]
mod tests {
    use crate::field_params::KeyWidth;

    #[test]
    fn key_width() {
        // New.
        let key_str = "My key-width pair";
        let position = [3, 5];
        let value_width = 3;
        let key_width = KeyWidth::new(key_str.to_string(), position, value_width);
        assert_eq!(&key_width.key.text, key_str);
        assert_eq!(key_width.key.position, position);
        assert_eq!(key_width.width, 22);
        assert_eq!(key_width.value.position, [22, 5]);
        assert_eq!(key_width.value.width_u32, value_width);
        let value = key_width.get_value("value");
        assert_eq!(&value.text, "lue");
        assert_eq!(value.position, [22, 5]);

        // New from width.
        let key_width = KeyWidth::new_from_width(key_str, position, 10, value_width);
        assert_eq!(key_width.key.text, "My k");
        assert_eq!(key_width.key.position, position);
        assert_eq!(key_width.width, 10);
        assert_eq!(key_width.value.position, [10, 5]);
        assert_eq!(key_width.value.width_u32, value_width);
    }
}
