use crate::panel::*;
use common::{EditMode, IndexedEditModes, PianoRollMode};
use hashbrown::HashMap;
use text::ppq_to_string;

/// The padding between input and mode labels.
const PADDING: u32 = 4;
type ModesMap = HashMap<PianoRollMode, (Label, Rectangle)>;

/// Render the top bar.
pub(super) struct TopBar {
    /// The top-left position.
    position: [u32; 2],
    /// The width of the bar.
    width: u32,
    /// The armed toggle.
    armed: Boolean,
    /// The input beat value.
    beat: KeyWidth,
    /// The use-volume toggle.
    use_volume: Boolean,
    /// The input volume value.
    volume: KeyWidth,
    /// The position of the vertical separator line to the right of the inputs.
    inputs_separator_position: [u32; 2],
    /// The position of the vertical separator line to the right of the modes.
    modes_separator_position: [u32; 2],
    /// The piano roll mode labels.
    modes: ModesMap,
    /// The position of the sub-mode label.
    edit_mode_position: [u32; 2],
}

impl TopBar {
    pub fn new(config: &Ini, text: &Text) -> Self {
        let piano_roll_panel_size = get_piano_roll_panel_size(config);
        let size = [piano_roll_panel_size[0], PIANO_ROLL_PANEL_TOP_BAR_HEIGHT];
        let piano_roll_panel_position = get_piano_roll_panel_position(config);
        let mut x = piano_roll_panel_position[0];
        let x0 = x;
        let y = piano_roll_panel_position[1] + 1;
        let position = [x, piano_roll_panel_position[1]];

        x += 1;

        // Get the fields.
        let armed = Boolean::new(text.get("PIANO_ROLL_PANEL_TOP_BAR_ARMED"), [x, y], text);
        x += armed.width + PADDING;
        let beat = KeyWidth::new(text.get("PIANO_ROLL_PANEL_TOP_BAR_BEAT"), [x, y], 4);
        // Only increment by 1 because beat has a long value space.
        x += beat.width + 1;
        let use_volume = Boolean::new(
            text.get("PIANO_ROLL_PANEL_TOP_BAR_USE_VOLUME"),
            [x, y],
            text,
        );
        x += use_volume.width + PADDING;
        let volume = KeyWidth::new(text.get("PIANO_ROLL_PANEL_TOP_BAR_VOLUME"), [x, y], 3);
        x += volume.width + PADDING;

        // Get the separator position.
        let inputs_separator_position = [x, y];

        x += PADDING + 3;

        // Get the modes.
        let total_modes_width = (((piano_roll_panel_size[0] - 2) - (x - x0)) as f64 * 0.75) as u32;
        let dx = total_modes_width / 4;
        let mut modes = HashMap::new();
        TopBar::insert_mode(
            "PIANO_ROLL_PANEL_TOP_BAR_TIME",
            PianoRollMode::Time,
            [x, y],
            &mut modes,
            text,
        );
        x += dx;
        TopBar::insert_mode(
            "PIANO_ROLL_PANEL_TOP_BAR_VIEW",
            PianoRollMode::View,
            [x, y],
            &mut modes,
            text,
        );
        x += dx;
        TopBar::insert_mode(
            "PIANO_ROLL_PANEL_TOP_BAR_SELECT",
            PianoRollMode::Select,
            [x, y],
            &mut modes,
            text,
        );
        x += dx;
        TopBar::insert_mode(
            "PIANO_ROLL_PANEL_TOP_BAR_EDIT",
            PianoRollMode::Edit,
            [x, y],
            &mut modes,
            text,
        );
        x += dx;

        // Get the separator position.
        let modes_separator_position = [x, y];

        x += 2;

        let edit_mode_position = [x, y];

        let width = size[0] - 2;
        Self {
            position,
            width,
            armed,
            beat,
            use_volume,
            volume,
            modes,
            inputs_separator_position,
            modes_separator_position,
            edit_mode_position,
        }
    }

    /// Update the top bar from the app state.
    pub fn update(&self, state: &State, renderer: &Renderer, text: &Text, focus: bool) {
        // Draw the fields.
        renderer.boolean(state.input.armed, &self.armed, focus);
        let value_color = Renderer::get_value_color([focus, true]);
        let key_color = Renderer::get_key_color(focus);
        let colors = [&key_color, &value_color];
        renderer.key_value(&ppq_to_string(state.input.beat.get_u()), &self.beat, colors);
        renderer.boolean(state.input.use_volume, &self.use_volume, focus);
        renderer.key_value(&state.input.volume.get().to_string(), &self.volume, colors);

        // Separator.
        let line_color = if focus {
            &ColorKey::FocusDefault
        } else {
            &ColorKey::NoFocus
        };
        renderer.vertical_line_separator(self.inputs_separator_position, line_color);

        // Draw the modes.
        for mode in self.modes.iter() {
            // Reverse the colors.
            if focus && *mode.0 == state.piano_roll_mode {
                renderer.rectangle(&mode.1 .1, &ColorKey::FocusDefault);
                renderer.text(&mode.1 .0, &ColorKey::Background);
            } else {
                renderer.text(
                    &mode.1 .0,
                    &(if focus {
                        ColorKey::FocusDefault
                    } else {
                        ColorKey::NoFocus
                    }),
                );
            }
        }

        // Separator.
        renderer.vertical_line_separator(self.modes_separator_position, line_color);

        // Edit mode.
        let edit_mode = match state.piano_roll_mode {
            PianoRollMode::Edit => Self::get_edit_mode_text(&state.edit_mode, text),
            PianoRollMode::Select => text.get_ref(if state.selection.single {
                "PIANO_ROLL_PANEL_EDIT_MODE_SINGLE"
            } else {
                "PIANO_ROLL_PANEL_EDIT_MODE_MANY"
            }),
            PianoRollMode::Time => Self::get_edit_mode_text(&state.time.mode, text),
            PianoRollMode::View => Self::get_edit_mode_text(&state.view.mode, text),
        };
        let edit_mode = LabelRef {
            text: edit_mode,
            position: self.edit_mode_position,
        };
        let edit_mode_color = if focus {
            ColorKey::Key
        } else {
            ColorKey::NoFocus
        };
        renderer.text_ref(&edit_mode, &edit_mode_color);

        // Horizontal line.
        renderer.horizontal_line(
            self.position[0],
            self.position[0] + self.width + 2,
            [0.45, -0.45],
            self.position[1] + 2,
            0.6,
            line_color,
        );
    }

    /// Returns the string corresponding to the edit mode.
    fn get_edit_mode_text<'t>(edit_mode: &IndexedEditModes, text: &'t Text) -> &'t str {
        let key = match edit_mode.get_ref() {
            EditMode::Normal => "PIANO_ROLL_PANEL_EDIT_MODE_NORMAL",
            EditMode::Quick => "PIANO_ROLL_PANEL_EDIT_MODE_QUICK",
            EditMode::Precise => "PIANO_ROLL_PANEL_EDIT_MODE_PRECISE",
        };
        text.get_ref(key)
    }

    /// Insert an edit mode label into a HashMap.
    fn insert_mode(
        key: &str,
        mode: PianoRollMode,
        position: [u32; 2],
        modes: &mut ModesMap,
        text: &Text,
    ) {
        let label = Label {
            text: text.get(key),
            position,
        };
        let rect = Rectangle::new(position, [label.text.chars().count() as u32, 1]);
        modes.insert(mode, (label, rect));
    }
}
