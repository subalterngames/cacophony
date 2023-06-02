use tooltip::get_tooltip;
use input::*;
use text::Text;
use crate::BooleanText;

/// Padding between text.
const PADDING: u32 = 3;

/// A field has a position and a label for the value.
pub(crate) struct Field {
    /// The position in grid units.
    pub position: [u32; 2],
    /// The width of the field.
    pub width: u32,
    /// The label text.
    pub label: String,
}

impl Field {
    /// Returns a new field with tooltip text. Moves the x value by the length of the field's title plus padding.
    pub fn horizontal_tooltip(key: &str, event: InputEvent, x: &mut u32, y: u32, input: &Input, text: &Text) -> Self {
        let label = Some(get_tooltip(key, &[event], input, text));
        let width = label.chars().count() as u32;
        let field = Self { position: [*x, y], label, width };
        *x += field.label.as_ref().unwrap().chars().count() as u32 + PADDING;
        field
    }

    /// Returns a new field that can fit a boolean. Moves the x value by the length of the field's title plus padding.
    pub fn horizontal_boolean(key: &str, boolean_text: &BooleanText, x: &mut u32, y: u32, text: &Text) -> Self {
        let field = Field::new_with_label([*x, y], key, text);
        *x += (field.label.as_ref().unwrap().chars().count() + 1 + boolean_text.get_max_length()) as u32 + PADDING;
        field
    }

    /// Returns a new field that can fit a value of known length. Moves the x value by the length of the field's title plus padding.
    pub fn horizontal_value(key: &str, length: u32, x: &mut u32, y: u32, text: &Text) -> Self {
        let label = text.get(key);
        let width = label.chars().count() as u32 + 1 + length;
        let position = [*x, y];
        let field: Field = Self { position, label, width };
        *x += field.label.as_ref().unwrap().chars().count() as u32 + 1 + length + PADDING;
        field
    }

    pub fn new(position: [u32; 2], key: &str, text: &Text) -> Self {
        let label = text.get(key);
        let width = label.chars().count() as u32;
        Self { position, label, width }
    }
}
