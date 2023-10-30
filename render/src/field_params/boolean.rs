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
    /// The key label is at `position`. The value label is at `position[0] + key.w + KV_PADDING`.
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
    /// - `width` The value label is at `position[0] + width - max_boolean_width`.
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

    /// Converts a boolean `value` into a `Label`.
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

#[cfg(test)]
mod tests {
    use crate::field_params::Boolean;
    use common::Paths;
    use ini::Ini;
    use std::path::PathBuf;
    use text::Text;

    #[test]
    fn boolean() {
        Paths::init(&PathBuf::from("../data"));
        let config = Ini::load_from_file("../data/config.ini").unwrap();
        let paths = Paths::get();
        let text = Text::new(&config, &paths);
        let b0_key = "Boolean value".to_string();
        let position = [3, 5];

        // New.
        let b0 = Boolean::new(b0_key.clone(), position, &text);
        assert_eq!(&b0.key.text, &b0_key);
        assert_eq!(b0.key.position, position);
        assert_eq!(b0.width, 16);
        assert!(b0.values.contains_key(&true));
        assert!(b0.values.contains_key(&false));
        for (bo, bt) in b0.values.keys().zip(["Y", "N"]) {
            let bv = &b0.values[bo];
            assert_eq!(&bv.text, bt);
            assert_eq!(bv.position, [18, 5]);
        }
        let bv = b0.get_boolean_label(&true);
        assert_eq!(&bv.text, "Y");
        let bv = b0.get_boolean_label(&false);
        assert_eq!(&bv.text, "N");

        // New from width.
        let width = 21;
        let b1 = Boolean::new_from_width(b0_key.clone(), position, width, &text);
        assert_eq!(&b1.key.text, &b0_key);
        assert_eq!(b1.key.position, position);
        assert_eq!(b1.width, width);
        for bo in b0.values.keys() {
            assert_eq!(b1.values[bo].position, [23, 5]);
        }
    }
}
