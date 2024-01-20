use crate::Renderer;

/// A position and a string.
#[derive(Clone)]
pub(crate) struct LabelRef<'a> {
    /// The position in grid units.
    pub position: [f32; 2],
    /// The text.
    pub text: &'a str,
}

impl<'a> LabelRef<'a> {
    pub fn new(position: [u32; 2], text: &'a str, renderer: &Renderer) -> Self {
        Self {
            position: renderer.get_label_position(position, text),
            text,
        }
    }
}
