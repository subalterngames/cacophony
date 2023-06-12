use crate::export_panel::ExportPanel;
use crate::main_menu::MainMenu;
use crate::music_panel::MusicPanel;
use crate::open_file_panel::OpenFilePanel;
use crate::panel::*;
use crate::piano_roll_panel::PianoRollPanel;
use crate::tracks_panel::TracksPanel;
use common::State;

/// Every panel.
pub struct Panels {
    /// The music panel.
    music_panel: MusicPanel,
    /// The main menu.
    main_menu: MainMenu,
    /// The tracks panel.
    tracks_panel: TracksPanel,
    /// The open-file panel.
    open_file_panel: OpenFilePanel,
    /// The piano roll panel.
    piano_roll_panel: PianoRollPanel,
    /// The export panel.
    export_panel: ExportPanel,
}

impl Panels {
    pub fn new(
        config: &Ini,
        state: &State,
        input: &Input,
        text: &Text,
        renderer: &Renderer,
    ) -> Self {
        let music_panel = MusicPanel::new(config, text);
        let main_menu = MainMenu::new(config, input, text);
        let tracks_panel = TracksPanel::new(config, text);
        let open_file_panel = OpenFilePanel::new(config, text);
        let piano_roll_panel = PianoRollPanel::new(config, state, text, renderer);
        let export_panel = ExportPanel::new(config, text);
        Self {
            music_panel,
            main_menu,
            tracks_panel,
            open_file_panel,
            piano_roll_panel,
            export_panel,
        }
    }

    /// Draw the panels.
    ///
    /// - `renderer` The renderer.
    /// - `state` The state of the app.
    /// - `conn` The synthesizer-player connection.
    /// - `input` Input events, key presses, etc.
    /// - `text` The text.
    /// - `open_file` The open-file context.
    pub fn update(
        &self,
        renderer: &Renderer,
        state: &State,
        conn: &Conn,
        input: &Input,
        text: &Text,
        open_file: &OpenFile,
    ) {
        // Draw the main panel.
        self.main_menu
            .update(renderer, state, conn, input, text, open_file);
        for panel_type in &state.panels {
            // Get the panel.
            let panel: &dyn Drawable = match panel_type {
                PanelType::Music => &self.music_panel,
                PanelType::MainMenu => &self.main_menu,
                PanelType::Tracks => &self.tracks_panel,
                PanelType::OpenFile => &self.open_file_panel,
                PanelType::PianoRoll => &self.piano_roll_panel,
                PanelType::Export => &self.export_panel,
            };
            // Draw the panel.
            panel.update(renderer, state, conn, input, text, open_file);
        }
    }

    /// Do something after input is received from elsewhere.
    pub fn late_update(&mut self, open_file: &mut OpenFile, renderer: &Renderer) {
        // The open-file panel was enabled on this frame. Capture the screen.
        if open_file.enabled {
            self.open_file_panel.set_background(renderer);
            open_file.enabled = false;
        }
    }
}
