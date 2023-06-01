pub(crate) use crate::Renderer;
pub(crate) use audio::Conn;
pub(crate) use common::State;
pub(crate) use input::Input;
pub(crate) use text::Text;

/// A drawable region.
pub(crate) trait Drawable {
    /// Draw the panel.
    ///
    /// - `renderer` The renderer.
    /// - `state` The state of the app.
    /// - `conn` The synthesizer-player connection.
    /// - `input` Input events, key presses, etc.
    /// - `text` The text.
    fn update(
        &self,
        renderer: &Renderer,
        state: &State,
        conn: &Conn,
        input: &Input,
        text: &Text,
    );
}
