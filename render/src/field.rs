use text::Text;

/// A field has a position and a label for the value.
pub(crate) struct Field {
    /// The position in grid units.
    pub position: [u32; 2],
    /// The label text.
    pub label: Option<String>,
}

impl Field {
    pub fn new_with_label(
        position: [u32; 2],
        key: &str,
        text: &Text,
    ) -> Self {
        let label = Some(text.get(key));
        Self { position, label }
    }

    pub fn new_no_label(position: [u32; 2]) -> Self {
        Self {
            position,
            label: None,
        }
    }
}
