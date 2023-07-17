use macroquad::texture::Texture2D;

/// Two textures. One for when there is focus, and one for when there isn't.
pub(crate) struct FocusableTexture {
    /// The texture for when something is in focus.
    focus: Texture2D,
    /// The texture for when something is not in focus.
    no_focus: Texture2D,
}

impl FocusableTexture {
    pub fn new(focus: Texture2D, no_focus: Texture2D) -> Self {
        Self { focus, no_focus }
    }

    pub fn get(&self, focus: bool) -> &Texture2D {
        if focus {
            &self.focus
        } else {
            &self.no_focus
        }
    }
}
