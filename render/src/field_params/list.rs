use text::truncate;
use super::{Label, Width};

const LEFT_ARROW: &str = "<";
const RIGHT_ARROW: &str = ">";

/// A list has a label and two arrows.
pub struct List {
    /// The label at the center of the list. There is no stored text.
    label: Width,
    /// The left arrow.
    pub left_arrow: Label,
    /// The right arrow.
    pub right_arrow: Label,
}

impl List {
    /// Fit the text, with the arrows, within the `width`.
    pub fn new(position: [u32; 2], width: u32) -> Self {
        let label_width = width - 2;
        let text_position = [position[0] + 1, position[1]];
        let label = Width { position: text_position, width: label_width};
        let left_arrow = Label { position, text: LEFT_ARROW.to_string() };
        let right_arrow = Label { position: [position[0] + width - 1, position[1]], text: RIGHT_ARROW.to_string() };
        Self { label, left_arrow, right_arrow }
    }

    /// Truncates a value string to `self.width` and converts it into a `Label`.
    pub fn get_value(&self, value: &str) -> Label {
        Label { position: self.label.position, text: truncate(&value, self.label.width, false)}
    }
}