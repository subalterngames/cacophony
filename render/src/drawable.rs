pub(crate) use crate::Renderer;
pub(crate) use audio::Conn;
pub(crate) use common::{PathsState, State};
pub(crate) use input::Input;
pub(crate) use text::Text;

/// A drawable region.
pub(crate) trait Drawable {
    /// Draw the panel.
    ///
    /// - `renderer` The renderer.
    /// - `state` The state of the app.
    /// - `conn` The synthesizer-player connection.
    /// - `text` The text.
    /// - `paths_state` The file paths state.
    fn update(
        &self,
        renderer: &Renderer,
        state: &State,
        conn: &Conn,
        text: &Text,
        paths_state: &PathsState,
    );
}
