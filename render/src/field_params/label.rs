/// A position and a string. This is used for drawing text.
pub(crate) struct Label {
    /// The position in grid units.
    pub position: [u32; 2],
    /// The text.
    pub text: String,
}