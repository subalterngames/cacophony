use super::{Label, Width};
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
    pub fn new(position: [u32; 2], width: u32) -> Self {
        let label = Width::new([position[0] + 1, position[1]], width as usize);
        let left_arrow = Label {
            position,
            text: LEFT_ARROW.to_string(),
        };
        let right_arrow = Label {
            position: [position[0] + width + 1, position[1]],
            text: RIGHT_ARROW.to_string(),
        };
        Self {
            label,
            left_arrow,
            right_arrow,
        }
    }

    /// Truncates a value string to `self.width` and converts it into a `Label`.
    pub fn get_value(&self, value: &str) -> Label {
        Label {
            position: self.label.position,
            text: truncate(value, self.label.width, false),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::field_params::{List, list::{LEFT_ARROW, RIGHT_ARROW}};

    #[test]
    fn field_params_list() {
        let li = List::new([3, 5], 17);
        assert_eq!(li.left_arrow.position, [3, 5]);
        assert_eq!(&li.left_arrow.text, LEFT_ARROW);
        assert_eq!(li.right_arrow.position, [21, 5]);
        assert_eq!(&li.right_arrow.text, RIGHT_ARROW);
        assert_eq!(li.label.position, [4, 5]);
        assert_eq!(li.label.width, 17);
        let la = li.get_value("This is a very long label! Too long!");
        assert_eq!(la.position, [4, 5]);
        assert_eq!(&la.text, "This is a very lo")
    }
}