use super::{Label, LabelRef, Width};
use crate::Renderer;
use text::truncate;

const LEFT_ARROW: &str = "<";
const RIGHT_ARROW: &str = ">";

/// A list has a label and two arrows.
pub(crate) struct List {
    /// The label at the center of the list. There is no stored text.
    label: Width,
    /// The left arrow.
    pub left_arrow: Label,
    /// The right arrow.
    pub right_arrow: Label,
}

impl List {
    /// Fit the text, with the arrows, within the `width`.
    pub fn new(position: [u32; 2], width: u32, renderer: &Renderer) -> Self {
        let label = Width::new([position[0] + 1, position[1]], width as usize);
        let left_arrow = Label::new(position, LEFT_ARROW.to_string(), renderer);
        let right_arrow = Label::new(
            [position[0] + width + 1, position[1]],
            RIGHT_ARROW.to_string(),
            renderer,
        );
        Self {
            label,
            left_arrow,
            right_arrow,
        }
    }

    /// Truncates a value string to `self.width` and converts it into a `LabelRef`.
    pub fn get_value<'t>(&self, value: &'t str, renderer: &Renderer) -> LabelRef<'t> {
        LabelRef::new(
            self.label.position,
            truncate(value, self.label.width, false),
            renderer,
        )
    }
}
