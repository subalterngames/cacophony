use super::{Label, Rectangle};

/// A label and its rectangle.
#[derive(Clone)]
pub(crate) struct LabelRectangle {
    /// The label.
    pub label: Label,
    /// The rectangle.
    pub rect: Rectangle,
}

impl LabelRectangle {
    pub fn new(position: [u32; 2], text: String) -> Self {
        let rect = Rectangle::new(position, [text.chars().count() as u32, 1]);
        let label = Label::new(position, text);
        Self { label, rect }
    }
}
