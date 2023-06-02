use text::Text;

/// The text to display in the app for boolean values.
pub struct BooleanText {
    pub yes: String,
    pub no: String,
}

impl BooleanText {
    pub fn new(text: &Text) -> Self {
        let yes = text.get("TRUE");
        let no = text.get("FALSE");
        Self { yes, no }
    }

    pub fn get_max_length(&self) -> usize {
        [self.yes, self.no].iter().map(|s| s.chars().count()).max().unwrap()
    }
}