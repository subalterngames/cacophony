use text::truncate;
use crate::BooleanText;
use super::Label;
use super::util::get_half_width;

/// A key-boolean pair.
pub(crate) struct Boolean {
    /// The key label.
    pub key: Label,
    /// The position of the boolean value.
    value_position: [u32; 2],
}

impl Boolean {
    /// The `key` will be at `position` and the boolean value will be at a position that tries to fill `width`.
    /// The `key` text will be truncated. The boolean position will be set to aligned-right.
    pub fn new(key: &str, position: [u32; 2], width: u32, boolean_text: &BooleanText) -> Self {
        let half_width = get_half_width(width);   

        // The key is on the left.
        let key = Label { position, text: &truncate(key, half_width, true )};

        // The value is on the right.
        let value_position = [position[0] + width - boolean_text.get_max_length() as u32, position[1]];
        
        Self {key, value_position}
    }

    /// Converts a boolean `value` into a `Label`.
    pub fn get_boolean_label(&self, value: bool, boolean_text: &BooleanText) -> Label {
        Label { position: self.value_position, text: if value { boolean_text.yes } else { boolean_text.no } }
    }
}