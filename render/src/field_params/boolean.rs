use super::util::KV_PADDING;
use super::Label;
use text::Text;

/// A key-boolean pair.
pub(crate) struct Boolean {
    /// The key label.
    pub key: Label,
    /// The position of the boolean value.
    value_position: [u32; 2],
    /// The width of the boolean kev-value pair.
    pub width: u32,
}

impl Boolean {
    pub fn new(key: &str, position: [u32; 2], text: &Text) -> Self {
        let key_width = key.chars().count() as u32 + KV_PADDING;
        let width = key_width + text.get_max_boolean_length() as u32;

        // The key is on the left.
        let key = Label {
            position,
            text: key.to_string(),
        };

        // The value is on the right.
        let value_position = [position[0] + key_width, position[1]];

        Self {
            key,
            value_position,
            width,
        }
    }

    /// Converts a boolean `value` into a `Label`.
    pub fn get_boolean_label(&self, value: bool, text: &Text) -> Label {
        Label {
            position: self.value_position,
            text: text.get_boolean(value),
        }
    }
}
