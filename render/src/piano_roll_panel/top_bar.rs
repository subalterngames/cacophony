use crate::panel::*;
use crate::BooleanText;

const PADDING: u32 = 3;

/// Render the top bar.
pub(super) struct TopBar {
    /// The top-left position.
    position: [u32; 2],
    /// The width of the bar.
    width: u32,
    /// The armed toggle.
    armed: Field,
    /// The input beat value.
    beat: Field,
    /// The use-volume toggle.
    use_volume: Field,
    /// The input volume value.
    volume: Field,
}

impl TopBar {
    pub fn new(boolean_text: &BooleanText, config: &Ini, text: &Text) -> Self {
        let piano_roll_panel_size = get_piano_roll_panel_size(config);
        let size = [piano_roll_panel_size[0], PIANO_ROLL_PANEL_TOP_BAR_HEIGHT];
        let piano_roll_panel_position = get_piano_roll_panel_position(config);
        let mut x = piano_roll_panel_position[0];
        let y = piano_roll_panel_position[1] + 1;
        let position = [x, piano_roll_panel_position[1]];
        x += 1;

        let armed = Field::horizontal_boolean("PIANO_ROLL_PANEL_TOP_BAR_ARMED", boolean_text, &mut x, y, text);
        let beat = Field::horizontal_value("PIANO_ROLL_PANEL_TOP_BAR_BEAT", 3, &mut x, y, text);
        let use_volume = Field::horizontal_boolean("PIANO_ROLL_PANEL_TOP_BAR_USE_VOLUME", boolean_text, &mut x, y, text);
        let volume = Field::horizontal_value("PIANO_ROLL_PANEL_TOP_BAR_VOLUME", 3, &mut x, y, text);
        let width = size[0] - 2;
        Self {
            position,
            width,
            armed, 
            beat,
            use_volume,
            volume,
        }
    }

    pub fn draw(&self, renderer: &Renderer, state: &State) {

    }

    pub fn draw(&self, piano_roll: &PianoRollPanel, focus: bool, state: &State) {
        let mut position = self.position;
        let value_color = &Renderer::get_value_color(focus, true);
        // Armed.
        let key_color = &Renderer::get_key_color(focus);
        let (armed_value, armed_value_width) = if state.input.armed {
            (&self.yes, self.yes_width)
        } else {
            (&self.no, self.no_width)
        };
        let armed_width = (self.armed_key_width + armed_value_width) + 3;
        state.renderer.key_value_horizontal(
            &self.armed_key,
            armed_value,
            position,
            armed_width,
            key_color,
            &Renderer::get_boolean_color(state.input.armed, focus),
        );
        position[0] += armed_width + PADDING;

        // Beat.
        state.renderer.key_value_horizontal(
            &self.beat,
            Text::fraction(&state.input.beat).as_str(),
            position,
            self.beat_width,
            key_color,
            value_color,
        );
        position[0] += self.beat_width + PADDING;

        // Volume.
        state.renderer.key_value_horizontal(
            &self.volume,
            &state.input.volume.to_string(),
            position,
            self.volume_width,
            key_color,
            value_color,
        );
        position[0] += self.volume_width + PADDING;

        // Use volume.
        let use_volume_value = if state.input.use_volume {
            &self.yes
        } else {
            &self.no
        };
        state.renderer.key_value_horizontal(
            &self.use_volume,
            use_volume_value,
            position,
            self.use_volume_width,
            key_color,
            &Renderer::get_boolean_color(state.input.use_volume, focus),
        );
        position[0] += self.use_volume_width + PADDING;

        // Vertical line.
        let top_bar_line_color = if focus {
            &ColorKey::FocusDefault
        } else {
            &ColorKey::NoFocus
        };

        TopBar::vertical_line(position, top_bar_line_color, &state.renderer);

        position[0] += PADDING;

        // Mode.
        for mode in PIANO_ROLL_MODES.iter() {
            let mode_string = mode.to_string(&state.text);
            let mode_width = mode_string.chars().count() as u32;
            // Reverse the colors.
            if focus && *mode == PIANO_ROLL_MODES[piano_roll.mode] {
                state
                    .renderer
                    .rectangle(position, [mode_width, 1], &ColorKey::FocusDefault);
                state
                    .renderer
                    .text(mode_string.as_str(), position, &ColorKey::Background);
            } else {
                state.renderer.text(
                    mode_string.as_str(),
                    position,
                    &(if focus {
                        ColorKey::FocusDefault
                    } else {
                        ColorKey::NoFocus
                    }),
                );
            }
            position[0] += mode_width + PADDING * 3;
        }

        // Vertical line.
        TopBar::vertical_line(position, top_bar_line_color, &state.renderer);
        position[0] += PADDING;

        // Sub-mode.
        let (key, value) = match PIANO_ROLL_MODES[piano_roll.mode] {
            PianoRollMode::Edit => (
                state.text.get("PIANO_ROLL_EDIT_MODE_PREFIX"),
                TIME_MODES[piano_roll.edit.mode].to_string(&state.text),
            ),
            PianoRollMode::Select => (
                state.text.get("PIANO_ROLL_SELECT_MODE_PREFIX"),
                piano_roll.select.mode.to_string(&state.text),
            ),
            PianoRollMode::Time => (
                state.text.get("PIANO_ROLL_TIME_MODE_PREFIX"),
                TIME_MODES[piano_roll.time.mode].to_string(&state.text),
            ),
            PianoRollMode::View => (
                state.text.get("PIANO_ROLL_VIEW_MODE_PREFIX"),
                TIME_MODES[piano_roll.view.mode].to_string(&state.text),
            ),
        };
        let width = (key.chars().count() + value.chars().count() + 1) as u32;
        let position = [self.position[0] + self.width - width, self.position[1]];
        let key_color = &Renderer::get_key_color(focus);
        let value_color = &Renderer::get_key_color(focus);
        state.renderer.key_value_horizontal(
            key.as_str(),
            value.as_str(),
            position,
            width,
            key_color,
            value_color,
        );

        // Horizontal line.
        state.renderer.horizontal_line(
            self.position[0],
            self.position[0] + self.width,
            [-0.5, 0.5],
            self.position[1] + 1,
            0.4,
            0.5,
            top_bar_line_color,
        );
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

impl Drawable for TopBar {
    fn update(
            &self,
            renderer: &Renderer,
            state: &State,
            conn: &Conn,
            input: &Input,
            text: &Text,
            open_file: &OpenFile,
        ) {
        let focus = state.panels[state.focus.get()] = PanelType::PianoRoll;

        // Input fields.
        renderer.boolean(&self.armed.label.as_ref().unwrap(), state.input.armed, self.armed.position, None, focus);
        renderer.key_value_horizontal(key, value, position, colors)
        //
    }
}