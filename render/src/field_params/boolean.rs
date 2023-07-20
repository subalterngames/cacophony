use super::util::KV_PADDING;
use super::Label;
use hashbrown::HashMap;
use text::Text;

/// A key-boolean pair.
pub(crate) struct Boolean {
    /// The key label.
    pub key: Label,
    /// The width of the boolean kev-value pair.
    pub width: u32,
    /// The value labels.
    values: HashMap<bool, Label>,
}

impl Boolean {
    /// The key label is at `position`. The value label is at `position.x + key.w + KV_PADDING`.
    ///
    /// - `key` The key label text.
    /// - `position` The position of the key label in grid units.
    /// - `text` The text lookup.
    pub fn new(key: String, position: [u32; 2], text: &Text) -> Self {
        let key_width = key.chars().count() as u32 + KV_PADDING;
        let width = key_width + text.get_max_boolean_length();

        // The key is on the left.
        let key = Label::new(position, key);

        // The value is on the right.
        let value_position = [position[0] + key_width, position[1]];

        let values: HashMap<bool, Label> = Self::get_boolean_labels(value_position, text);

        Self { key, values, width }
    }

    /// The key label is at `position`.
    ///
    /// - `key` The key label text.
    /// - `position` The position of the key label in grid units.
    /// - `width` The value label is at `position.x + width - max_boolean_width`.
    /// - `text` The text lookup.
    pub fn new_from_width(key: String, position: [u32; 2], width: u32, text: &Text) -> Self {
        // The key is on the left.
        let key = Label::new(position, key);

        // The value is on the right.
        let value_position = [
            position[0] + width - text.get_max_boolean_length(),
            position[1],
        ];
        let values: HashMap<bool, Label> = Self::get_boolean_labels(value_position, text);

        Self { key, values, width }
    }

    /// Returns the label corresponding to `value`.
    pub fn get_boolean_label(&self, value: &bool) -> &Label {
        &self.values[value]
    }

    /// Converts a boolean `value` into a `LabelRef`.
    fn get_boolean_labels(value_position: [u32; 2], text: &Text) -> HashMap<bool, Label> {
        let mut values = HashMap::new();
        values.insert(
            true,
            Label::new(value_position, text.get_boolean(&true).to_string()),
        );
        values.insert(
            false,
            Label::new(value_position, text.get_boolean(&false).to_string()),
        );
        values
    }
}
