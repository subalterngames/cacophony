use crate::music_panel::MusicPanel;
use crate::panel::*;

/// Every panel.
pub struct Panels {
    /// The music panel.
    music_panel: MusicPanel,
}

impl Panels {
    pub fn new(config: &Ini, text: &Text) -> Self {
        let music_panel = MusicPanel::new(config, text);
        Self { music_panel }
    }

    /// Draw the panels.
    ///
    /// - `renderer` The renderer.
    /// - `state` The state of the app.
    /// - `conn` The synthesizer-player connection.
    /// - `input` Input events, key presses, etc.
    /// - `text` The text.
    pub fn update(
        &self,
        renderer: &Renderer,
        state: &State,
        conn: &Conn,
        input: &Input,
        text: &Text,
    ) {
        for panel_type in &state.panels {
            // Get the panel.
            let panel: &dyn Drawable = match panel_type {
                PanelType::Music => &self.music_panel,
                other => panic!("TODO {:?}", other),
            };
            // Draw the panel.
            panel.update(renderer, state, conn, input, text);
        }
    }
}
