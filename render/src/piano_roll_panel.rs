use crate::panel::*;
mod image;
mod note_names;
mod piano_roll_rows;
use note_names::get_note_names;
use piano_roll_rows::get_piano_roll_rows;
mod top_bar;
use super::FocusableTexture;
use common::State;
use common::{MAX_NOTE, MIN_NOTE};
use top_bar::TopBar;

/// Draw the piano roll panel.
pub struct PianoRollPanel {
    /// The panel.
    panel: Panel,
    /// Data for the top bar sub-panel.
    top_bar: TopBar,
    /// The note names textures.
    note_names: FocusableTexture,
    /// The position of the note names.
    note_names_position: [u32; 2],
    /// The height of the piano roll sub-panel.
    piano_roll_height: u32,
    /// The piano roll rows textures.
    piano_roll_rows: FocusableTexture,
    /// The position of the piano roll rows.
    piano_roll_rows_position: [u32; 2],
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
        let note_names = get_note_names(config, renderer);
        let note_names_position = [
            panel.position[0] + 1,
            panel.position[1] + PIANO_ROLL_PANEL_TOP_BAR_HEIGHT,
        ];
        let piano_roll_height = (state.view.dn[1] - state.view.dn[0]) as u32;
        let piano_roll_rows = get_piano_roll_rows(config, renderer);
        let piano_roll_rows_position = [
            note_names_position[0] + PIANO_ROLL_PANEL_NOTE_NAMES_WIDTH,
            note_names_position[1],
        ];
        Self {
            panel,
            top_bar,
            note_names,
            note_names_position,
            piano_roll_height,
            piano_roll_rows,
            piano_roll_rows_position,
        }
    }
}

impl Drawable for PianoRollPanel {
    fn update(
        &self,
        renderer: &Renderer,
        state: &State,
        conn: &Conn,
        _: &Input,
        text: &Text,
        _: &OpenFile,
    ) {
        let focus = self.panel.has_focus(state);

        // Panel background.
        self.panel.update(focus, renderer);

        // Top bar.
        self.top_bar.update(state, renderer, text, focus);

        // Note names.
        let texture = self.note_names.get(focus);
        let rect = [
            0,
            ((MAX_NOTE - MIN_NOTE) - state.view.dn[0]) as u32,
            PIANO_ROLL_PANEL_NOTE_NAMES_WIDTH,
            self.piano_roll_height,
        ];
        renderer.texture(texture, self.note_names_position, Some(rect));

        // Piano roll rows.
        let texture = self.piano_roll_rows.get(focus);
        renderer.texture(texture, self.piano_roll_rows_position, None);
    }
}
