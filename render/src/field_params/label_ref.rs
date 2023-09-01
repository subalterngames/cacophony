/// A position and a string.
#[derive(Clone)]
pub(crate) struct LabelRef<'a> {
    /// The position in grid units.
    pub position: [u32; 2],
    /// The text.
    pub text: &'a str,
}

impl<'a> LabelRef<'a> {
    pub fn new(position: [u32; 2], text: &'a str) -> Self {
        Self { position, text }
    }
}
