use super::{Label, RectanglePixel};
use crate::Renderer;

/// A label and its rectangle.
#[derive(Clone)]
pub(crate) struct LabelRectangle {
    /// The label.
    pub label: Label,
    /// The rectangle.
    pub rect: RectanglePixel,
}

impl LabelRectangle {
    pub fn new(position: [u32; 2], text: String, renderer: &Renderer) -> Self {
        let rect = RectanglePixel::new_from_u(position, [text.chars().count() as u32, 1], renderer);
        let label = Label::new(position, text, renderer);
        Self { label, rect }
    }
}
