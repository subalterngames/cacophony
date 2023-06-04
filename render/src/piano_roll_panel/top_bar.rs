use crate::panel::*;
use crate::BooleanText;
use common::hashbrown::HashMap;
use common::{EditMode, Index, PianoRollMode, SelectMode, EDIT_MODES};
use text::fraction;

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
    modes: HashMap<PianoRollMode, Label>,
    /// The position of the sub-mode label.
    edit_mode_position: [u32; 2],
}

impl TopBar {
    pub fn new(boolean_text: &BooleanText, config: &Ini, text: &Text) -> Self {
        let piano_roll_panel_size = get_piano_roll_panel_size(config);
        let size = [piano_roll_panel_size[0], PIANO_ROLL_PANEL_TOP_BAR_HEIGHT];
        let piano_roll_panel_position = get_piano_roll_panel_position(config);
        let mut x = piano_roll_panel_position[0];
        let x0 = x;
        let y = piano_roll_panel_position[1] + 1;
        let position = [x, piano_roll_panel_position[1]];

        // The width of all of the input fields.
        let total_inputs_width = (piano_roll_panel_size[0] as f64 * 0.6) as u32 - 2;
        // The width of each input field.
        let input_width = total_inputs_width / 4 - 3;

        x += 1;

        // Get the fields.
        let dx = input_width + 1;
        let armed = Boolean::new(
            &text.get("PIANO_ROLL_PANEL_TOP_BAR_ARMED"),
            [x, y],
            input_width,
            boolean_text,
        );
        x += dx;
        let beat = KeyWidth::new(
            &text.get("PIANO_ROLL_PANEL_TOP_BAR_BEAT"),
            [x, y],
            input_width,
            4,
        );
        x += dx;
        let use_volume = Boolean::new(
            &text.get("PIANO_ROLL_PANEL_TOP_BAR_USE_VOLUME"),
            [x, y],
            input_width,
            boolean_text,
        );
        x += dx;
        let volume = KeyWidth::new(
            &text.get("PIANO_ROLL_PANEL_TOP_BAR_VOLUME"),
            [x, y],
            input_width,
            3,
        );
        x += dx;

        // Get the separator position.
        let inputs_separator_position = [x, y];

        x += 2;

        // Get the modes.
        let total_modes_width = (((piano_roll_panel_size[0] - 2) - (x - x0)) as f64 * 0.75) as u32;
        let dx = total_modes_width / 4;
        let mut modes = HashMap::new();
        modes.insert(
            PianoRollMode::Time,
            Label {
                text: text.get("PIANO_ROLL_PANEL_TOP_BAR_TIME"),
                position: [x, y],
            },
        );
        x += dx;
        modes.insert(
            PianoRollMode::View,
            Label {
                text: text.get("PIANO_ROLL_PANEL_TOP_BAR_VIEW"),
                position: [x, y],
            },
        );
        x += dx;
        modes.insert(
            PianoRollMode::Select,
            Label {
                text: text.get("PIANO_ROLL_PANEL_TOP_BAR_SELECT"),
                position: [x, y],
            },
        );
        x += dx;
        modes.insert(
            PianoRollMode::Edit,
            Label {
                text: text.get("PIANO_ROLL_PANEL_TOP_BAR_EDIT"),
                position: [x, y],
            },
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

    pub fn update(&self, renderer: &Renderer, state: &State, text: &Text, focus: bool) {
        // Draw the fields.
        renderer.boolean(state.input.armed, &self.armed, focus);
        let value_color = Renderer::get_value_color([focus, true]);
        let key_color = Renderer::get_key_color(focus);
        let colors = [&key_color, &value_color];
        renderer.key_value(&fraction(&state.input.beat), &self.beat, colors);
        renderer.boolean(state.input.use_volume, &self.use_volume, focus);
        renderer.key_value(&state.input.volume.get().to_string(), &self.volume, colors);

        // Separator.
        let line_color = if focus {
            &ColorKey::FocusDefault
        } else {
            &ColorKey::NoFocus
        };
        Self::vertical_line(self.inputs_separator_position, line_color, renderer);

        // Draw the modes.
        for mode in self.modes.iter() {
            // Reverse the colors.
            if focus && *mode.0 == state.piano_roll_mode {
                renderer.rectangle(
                    mode.1.position,
                    [mode.1.text.chars().count() as u32, 1],
                    &ColorKey::FocusDefault,
                );
                renderer.text(mode.1, &ColorKey::Background);
            } else {
                renderer.text(
                    mode.1,
                    &(if focus {
                        ColorKey::FocusDefault
                    } else {
                        ColorKey::NoFocus
                    }),
                );
            }
        }

        // Separator.
        Self::vertical_line(self.modes_separator_position, line_color, renderer);

        // Edit mode.
        let edit_mode = match state.piano_roll_mode {
            PianoRollMode::Edit => Self::get_edit_mode_text(&state.edit_mode, text),
            PianoRollMode::Select => match state.select_mode {
                SelectMode::Single(_) => text.get("PIANO_ROLL_PANEL_EDIT_MODE_SINGLE"),
                SelectMode::Many(_) => text.get("PIANO_ROLL_PANEL_EDIT_MODE_MANY"),
            },
            PianoRollMode::Time => Self::get_edit_mode_text(&state.view.mode, text),
            PianoRollMode::View => Self::get_edit_mode_text(&state.view.mode, text),
        };
        let edit_mode = Label {
            text: edit_mode,
            position: self.edit_mode_position,
        };
        let edit_mode_color = if focus {
            ColorKey::Key
        } else {
            ColorKey::NoFocus
        };
        renderer.text(&edit_mode, &edit_mode_color);

        // Horizontal line.
        renderer.horizontal_line(
            self.position[0],
            self.position[0] + self.width,
            [-0.5, 0.5],
            self.position[1] + 1,
            0.4,
            line_color,
        );
    }

    fn get_edit_mode_text(edit_mode: &Index, text: &Text) -> String {
        let edit_mode = EDIT_MODES[edit_mode.get()];
        let key = match edit_mode {
            EditMode::Normal => "PIANO_ROLL_PANEL_EDIT_MODE_NORMAL",
            EditMode::Quick => "PIANO_ROLL_PANEL_EDIT_MODE_QUICK",
            EditMode::Precise => "PIANO_ROLL_PANEL_EDIT_MODE_PRECISE",
        };
        text.get(key)
    }

    fn vertical_line(position: [u32; 2], color: &ColorKey, renderer: &Renderer) {
        renderer.vertical_line(
            position[0],
            0.5,
            position[1],
            position[1],
            [-0.5, 1.0],
            color,
        );
    }
}
