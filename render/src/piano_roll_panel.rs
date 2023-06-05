use crate::panel::*;
mod image;
mod note_names;
use note_names::NoteNames;
mod top_bar;
use common::{MAX_NOTE, MIN_NOTE};
use common::State;
use top_bar::TopBar;

/// Draw the piano roll panel.
pub struct PianoRollPanel {
    /// The panel.
    panel: Panel,
    /// Data for the top bar sub-panel.
    top_bar: TopBar,
    /// The note names textures.
    note_names: NoteNames,
    /// The position of the note names.
    note_names_position: [u32; 2],
    /// The height of the piano roll sub-panel.
    piano_roll_height: u32
}

impl PianoRollPanel {
    pub fn new(config: &Ini, state: &State, text: &Text, renderer: &Renderer) -> Self {
        let piano_roll_panel_position = get_piano_roll_panel_position(config);
        let piano_roll_panel_size = get_piano_roll_panel_size(config);
        let panel = Panel::new(
            PanelType::PianoRoll,
            piano_roll_panel_position,
            piano_roll_panel_size,
            text,
        );
        let top_bar = TopBar::new(config, text);
        let note_names = NoteNames::new(config, renderer);
        let note_names_position = [
            panel.position[0] + 1,
            panel.position[1] + PIANO_ROLL_PANEL_TOP_BAR_HEIGHT,
        ];
        let piano_roll_height = (state.view.dn[1] - state.view.dn[0]) as u32;
        Self {
            panel,
            top_bar,
            note_names,
            note_names_position,
            piano_roll_height,
        }
    }
}

impl Drawable for PianoRollPanel {
    fn update(
        &self,
        renderer: &Renderer,
        state: &State,
        conn: &Conn,
        input: &Input,
        text: &Text,
        _: &OpenFile,
    ) {
        let focus = self.panel.has_focus(state);

        // Panel background.
        self.panel.update(focus, renderer);

        // Top bar.
        self.top_bar.update(state, renderer, text, focus);

        // Note names.
        let texture = self.note_names.texture.get(focus);
        let rect = [
            0,
            ((MAX_NOTE - MIN_NOTE) - state.view.dn[0]) as u32,
            PIANO_ROLL_PANEL_NOTE_NAMES_WIDTH,
            self.piano_roll_height,
        ];
        renderer.texture(texture, self.note_names_position, Some(rect));
    }
}
