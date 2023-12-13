/// A position and a string.
#[derive(Clone, Debug)]
pub(crate) struct Label {
    /// The position in grid units.
    pub position: [u32; 2],
    /// The text.
    pub text: String,
}

impl Label {
    pub fn new(position: [u32; 2], text: String) -> Self {
        Self { position, text }
    }
}
