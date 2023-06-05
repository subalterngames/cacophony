use common::macroquad::texture::Texture2D;

/// Two textures. One for when there is focus, and one for when there isn't.
pub(crate) struct FocusableTexture {
    /// The texture for when something is in focus.
    pub focus: Texture2D,
    /// The texture for when something is not in focus.
    pub no_focus: Texture2D,
}
