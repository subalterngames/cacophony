use crate::edit_mode::EDIT_MODES;
use crate::music_panel_field::{MusicPanelField, MUSIC_PANEL_FIELDS};
use crate::{Index, InputState, Music, PanelType, PianoRollMode, SelectMode, Time, View};
use ini::Ini;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    /// The music.
    pub music: Music,
    /// The viewport.
    pub view: View,
    /// The time state.
    pub time: Time,
    /// The input state.
    pub input: InputState,
    /// A list of all panels that need to be drawn.
    pub panels: Vec<PanelType>,
    /// The index of the focused panel.
    pub focus: Index,
    /// The currently-selected music panel field.
    pub music_panel_field: Index,
    /// The piano roll panel's current mode.
    pub piano_roll_mode: PianoRollMode,
    /// The index of the current piano roll edit mode.
    pub edit_mode: Index,
    /// The current selection.
    pub select_mode: SelectMode,
}

impl State {
    pub fn new(config: &Ini) -> State {
        let music = Music::default();
        let view = View::new(config);
        let time = Time::default();
        let input = InputState::default();
        let panels = vec![PanelType::Music, PanelType::Tracks, PanelType::PianoRoll];
        let focus = Index::new(0, panels.len());
        let music_panel_field = Index::new(0, MUSIC_PANEL_FIELDS.len());
        let piano_roll_mode = PianoRollMode::Time;
        let edit_mode = Index::new(0, EDIT_MODES.len());
        let select_mode = SelectMode::Single(None);
        Self {
            music,
            view,
            time,
            input,
            panels,
            focus,
            music_panel_field,
            piano_roll_mode,
            edit_mode,
            select_mode,
        }
    }

    pub fn get_music_panel_field(&self) -> &MusicPanelField {
        &MUSIC_PANEL_FIELDS[self.music_panel_field.get()]
    }
}
