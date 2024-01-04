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

#[cfg(test)]
mod tests {
    use crate::field_params::{
        list::{LEFT_ARROW, RIGHT_ARROW},
        List,
    };
    use crate::tests::get_test_renderer;

    #[test]
    fn field_params_list() {
        let renderer = get_test_renderer();
        let li = List::new([3, 5], 17, &renderer);
        assert_eq!(&li.left_arrow.text, LEFT_ARROW);
        assert_eq!(&li.right_arrow.text, RIGHT_ARROW);
        assert_eq!(li.label.position, [4, 5]);
        assert_eq!(li.label.width, 17);
        let la = li.get_value("This is a very long label! Too long!", &renderer);
        assert_eq!(la.text, "This is a very lo")
    }
}
