use crate::Renderer;

/// A position and a string.
#[derive(Clone)]
pub(crate) struct Label {
    /// The position in grid units.
    pub position: [f32; 2],
    /// The text.
    pub text: String,
}

impl Label {
    pub fn new(position: [u32; 2], text: String, renderer: &Renderer) -> Self {
        Self {
            position: renderer.get_label_position(position, &text),
            text,
        }
    }
}
