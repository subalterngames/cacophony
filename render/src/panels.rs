use crate::export_panel::ExportPanel;
use crate::export_settings_panel::ExportSettingsPanel;
use crate::main_menu::MainMenu;
use crate::music_panel::MusicPanel;
use crate::open_file_panel::OpenFilePanel;
use crate::panel::*;
use crate::piano_roll_panel::PianoRollPanel;
use crate::quit_panel::QuitPanel;
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
    /// The export settings panel.
    export_settings_panel: ExportSettingsPanel,
    /// The quit panel.
    quit_panel: QuitPanel,
}

impl Panels {
    pub fn new(
        config: &Ini,
        state: &State,
        input: &Input,
        text: &mut Text,
        renderer: &Renderer,
        remote_version: Option<String>,
    ) -> Self {
        let music_panel = MusicPanel::new(config, text);
        let main_menu = MainMenu::new(config, input, text, remote_version);
        let tracks_panel = TracksPanel::new(config, text);
        let open_file_panel = OpenFilePanel::new(config, text);
        let piano_roll_panel = PianoRollPanel::new(config, state, text, renderer);
        let export_panel = ExportPanel::new(config, text);
        let export_settings_panel = ExportSettingsPanel::new(config, text);
        let quit_panel = QuitPanel::new(config, text, input);
        Self {
            music_panel,
            main_menu,
            tracks_panel,
            open_file_panel,
            piano_roll_panel,
            export_panel,
            export_settings_panel,
            quit_panel,
        }
    }

    /// Draw the panels.
    ///
    /// - `renderer` The renderer.
    /// - `state` The state of the app.
    /// - `conn` The synthesizer-player connection.
    /// - `input` Input events, key presses, etc.
    /// - `text` The text.
    /// - `paths_state` The state of the file paths.
    /// - `exporter` The exporter.
    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &self,
        renderer: &Renderer,
        state: &State,
        conn: &Conn,
        input: &Input,
        text: &Text,
        paths_state: &PathsState,
        exporter: &SharedExporter,
    ) {
        // Draw the main panel.
        self.main_menu
            .update(renderer, state, conn, input, text, paths_state, exporter);
        for panel_type in &state.panels {
            // Get the panel.
            let panel: &dyn Drawable = match panel_type {
                PanelType::Music => &self.music_panel,
                PanelType::MainMenu => &self.main_menu,
                PanelType::Tracks => &self.tracks_panel,
                PanelType::OpenFile => &self.open_file_panel,
                PanelType::PianoRoll => &self.piano_roll_panel,
                PanelType::ExportState => &self.export_panel,
                PanelType::ExportSettings => &self.export_settings_panel,
                PanelType::Quit => &self.quit_panel,
            };
            // Draw the panel.
            panel.update(renderer, state, conn, input, text, paths_state, exporter);
        }
    }

    /// Do something after input is received from elsewhere.
    pub fn late_update(&mut self, state: &State, renderer: &Renderer) {
        self.open_file_panel.popup.late_update(state, renderer);
        self.export_panel.popup.late_update(state, renderer);
        self.quit_panel.popup.late_update(state, renderer);
    }
}
