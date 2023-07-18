use crate::music_panel_field::MusicPanelField;
use crate::{
    EditMode, Index, IndexedEditModes, IndexedValues, InputState, Music, PanelType, PianoRollMode,
    SelectMode, Time, View,
};
use ini::Ini;
use serde::{Deserialize, Serialize};

/// `State` contains all app data that can go on the undo/redo stacks.
/// Because the entire `State` goes on the stacks, it needs to be as small as possible.
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
    pub music_panel_field: IndexedValues<MusicPanelField, 3>,
    /// The piano roll panel's current mode.
    pub piano_roll_mode: PianoRollMode,
    /// The index of the current piano roll edit mode.
    pub edit_mode: IndexedEditModes,
    /// The current selection.
    pub select_mode: SelectMode,
    /// If true, there are unsaved changes.
    #[serde(skip_serializing, skip_deserializing)]
    pub unsaved_changes: bool,
}

impl State {
    pub fn new(config: &Ini) -> State {
        let music = Music::default();
        let view = View::new(config);
        let time = Time::default();
        let input = InputState::default();
        let panels = vec![PanelType::Music, PanelType::Tracks, PanelType::PianoRoll];
        let focus = Index::new(0, panels.len());
        let music_panel_field = IndexedValues::new(
            0,
            [
                MusicPanelField::Name,
                MusicPanelField::BPM,
                MusicPanelField::Gain,
            ],
        );
        let piano_roll_mode = PianoRollMode::Time;
        let edit_mode = EditMode::indexed();
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
            unsaved_changes: false,
        }
    }
}
