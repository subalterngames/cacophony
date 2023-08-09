use crate::panel::*;
use colorgrad::Color;
use colorgrad::CustomGradient;
use image::{ImageBuffer, Rgba};
use input::InputEvent;
use macroquad::texture::Texture2D;

/// The color of the panel and the text.
const COLOR: ColorKey = ColorKey::Key;
/// Lerp the sample bars by this delta.
const LERP_DT: f32 = 0.2;
/// Check the power bar sample this many samples.
const POWER_BAR_DELTA: u64 = 5;

/// Apply a lerp to a sample value.
#[derive(Default)]
struct Lerp {
    a: f32,
    b: f32,
    up: bool,
}

impl Lerp {
    pub(super) fn set(&mut self, b: f32) {
        self.b = b;
        self.up = self.b > self.a;
    }

    pub(super) fn lerp(&mut self) -> f32 {
        if self.up && self.a < self.b {
            self.a += LERP_DT;
            if self.a > self.b {
                self.a = self.b;
            }
        } else if !self.up && self.a > self.b {
            self.a -= LERP_DT;
            if self.a < self.b {
                self.a = self.b;
            }
        }
        self.a
    }
}

/// The main menu panel. This panel is always in ghostly not-quite-focus.
pub(crate) struct MainMenu {
    /// The panel background.
    panel: Panel,
    /// The title if there are unsaved changes.
    title_changes: LabelRectangle,
    /// The field labels and the version label.
    labels: [Label; 7],
    /// The positions of the separator lines.
    separator_positions: [[u32; 2]; 2],
    /// The power bar texture.
    power_bar_texture: Texture2D,
    /// The rectangles of the power bars per sample.
    power_bar_rects: [[[f32; 2]; 2]; 2],
    /// Sample lerp targets per bar.
    power_bar_lerps: [Lerp; 2],
    /// The current sample time. This is updated continuously and is used to smooth the power bars.
    time: u64,
}

impl MainMenu {
    pub fn new(
        config: &Ini,
        input: &Input,
        text: &mut Text,
        renderer: &Renderer,
        remote_version: Option<String>,
    ) -> Self {
        // Get the width of the panel.
        let width = get_main_menu_width(config);

        let position = get_main_menu_position(config);

        // Get the panel.
        let mut panel = Panel::new(
            PanelType::MainMenu,
            position,
            [width, MAIN_MENU_HEIGHT],
            text,
        );
        // Add an update notice to the title.
        if let Some(remote_version) = remote_version {
            let update = text.get_with_values("MAIN_MENU_UPDATE", &[&remote_version]);
            panel.title.label.text.push_str("   ");
            panel.title.label.text.push_str(&update);
            panel.title.rect.size[0] += update.chars().count() as u32 + 3;
        }
        let title_changes = LabelRectangle::new(
            panel.title.label.position,
            format!("*{}", panel.title.label.text),
        );

        // Get the fields.
        let mut x = panel.rect.position[0] + 2;
        let y = panel.rect.position[1] + 1;
        let help = Self::label_from_key("MAIN_MENU_HELP", &mut x, y, text);
        x += 4;
        let status = Self::tooltip(
            "MAIN_MENU_STATUS",
            InputEvent::StatusTTS,
            &mut x,
            y,
            input,
            text,
        );
        let input_field = Self::tooltip(
            "MAIN_MENU_INPUT",
            InputEvent::InputTTS,
            &mut x,
            y,
            input,
            text,
        );
        let app = Self::tooltip("MAIN_MENU_APP", InputEvent::AppTTS, &mut x, y, input, text);
        let file = Self::tooltip(
            "MAIN_MENU_FILE",
            InputEvent::FileTTS,
            &mut x,
            y,
            input,
            text,
        );
        let stop = Self::tooltip(
            "MAIN_MENU_STOP",
            InputEvent::StopTTS,
            &mut x,
            y,
            input,
            text,
        );
        x += 1;
        let separator_help = [x, y];
        x += 2;
        let x0 = x;
        let links = Self::tooltip(
            "MAIN_MENU_ONLINE",
            InputEvent::EnableLinksPanel,
            &mut x,
            y,
            input,
            text,
        );
        x = x0 + links.text.chars().count() as u32 + 1;
        let separator_links = [x, y];
        let fields = [help, status, input_field, app, file, stop, links];
        let separator_positions = [separator_help, separator_links];
        x += 1;
        let window_grid_size = get_window_grid_size(config);
        let w = window_grid_size[0] - x - 2;
        let (power_bar_texture, power_bar_position_left) =
            Self::get_power_textures([x, y], [w, 1], renderer);
        let cell_height = renderer.grid_to_pixel([1, 1])[1];
        let mut power_bar_position_right = [
            [
                power_bar_position_left[0][0],
                power_bar_position_left[0][1] + cell_height,
            ],
            power_bar_position_left[1],
        ];
        power_bar_position_right[0][1] -= power_bar_position_right[1][1];
        let power_bar_rects = [power_bar_position_left, power_bar_position_right];
        let power_bar_lerps = [Lerp::default(), Lerp::default()];
        Self {
            panel,
            labels: fields,
            title_changes,
            separator_positions,
            power_bar_texture,
            power_bar_rects,
            power_bar_lerps,
            time: 0,
        }
    }

    fn label(key: String, x: &mut u32, y: u32) -> Label {
        let width = key.chars().count() as u32;
        let position = [*x, y];
        *x += width;
        Label::new(position, key)
    }

    fn label_from_key(key: &str, x: &mut u32, y: u32, text: &Text) -> Label {
        Self::label(text.get(key), x, y)
    }

    fn tooltip(
        key: &str,
        event: InputEvent,
        x: &mut u32,
        y: u32,
        input: &Input,
        text: &mut Text,
    ) -> Label {
        let tooltip = text.get_tooltip(key, &[event], input).seen;
        let width = key.chars().count() as u32;
        let position = [*x, y];
        *x += width;
        Label::new(position, tooltip)
    }

    /// Returns a tuple: The power bar texture and a rectangle around it.
    fn get_power_textures(
        position: [u32; 2],
        size: [u32; 2],
        renderer: &Renderer,
    ) -> (Texture2D, [[f32; 2]; 2]) {
        let position = renderer.grid_to_pixel(position);
        let mut size = renderer.grid_to_pixel(size);
        size[1] /= 2.0;
        // Define the gradient.
        let color_0 = Self::get_color(&ColorKey::FocusDefault, renderer);
        let color_1 = Self::get_color(&ColorKey::Value, renderer);
        let gradiant = CustomGradient::new()
            .colors(&[color_0, color_1])
            .build()
            .unwrap();
        // Define the image.
        let mut image = ImageBuffer::new(size[0] as u32, size[1] as u32);
        let width = size[0] as f64;
        for (x, _, pixel) in image.enumerate_pixels_mut() {
            let rgba = gradiant.at(x as f64 / width).to_rgba8();
            *pixel = Rgba(rgba);
        }
        let texture = Texture2D::from_rgba8(size[0] as u16, size[1] as u16, &image);
        (texture, [position, size])
    }

    /// Converts a ColorKey into a gradiant color.
    fn get_color(color_key: &ColorKey, renderer: &Renderer) -> Color {
        let color = renderer.get_color(color_key);
        Color::new(color.r as f64, color.g as f64, color.b as f64, 1.0)
    }

    /// Draw a power bar and its mask.
    fn draw_sample_power(&mut self, index: usize, renderer: &Renderer) {
        // Draw the bar.
        let rect = &self.power_bar_rects[index];
        renderer.texture_pixel(&self.power_bar_texture, rect[0], None);
        let value = self.power_bar_lerps[index].lerp();
        // Get the width of the mask.
        let w = rect[1][0] * (1.0 - value);
        // Get the x coordinate of the mask.
        let x = rect[0][0] + (rect[1][0] - w);
        let position = [x, rect[0][1]];
        let size = [w, rect[1][1]];
        // Draw the mask.
        renderer.rectangle_pixel(position, size, &ColorKey::Background);
    }

    /// Set the lerp target of power bar.
    fn set_lerp_target(&mut self, index: usize, sample: f32) {
        self.power_bar_lerps[index].set(sample.abs());
    }

    /// Get a sample, set lerp targets, and draw bars.
    pub fn late_update(&mut self, renderer: &Renderer, conn: &Conn) {
        // Set the power bar lerp targets from the sample.
        if let Some(sample) = conn.sample {
            if self.time % POWER_BAR_DELTA == 0 {
                self.set_lerp_target(0, sample.0);
                self.set_lerp_target(1, sample.1);
            }
            self.time += 1;
        }
        // Draw each bar.
        self.draw_sample_power(0, renderer);
        self.draw_sample_power(1, renderer);
    }
}

impl Drawable for MainMenu {
    fn update(
        &self,
        renderer: &Renderer,
        state: &State,
        _: &Conn,
        _: &Input,
        _: &Text,
        _: &PathsState,
        _: &SharedExporter,
    ) {
        self.panel.update_ex(&COLOR, renderer);
        if state.unsaved_changes {
            renderer.rectangle(&self.title_changes.rect, &ColorKey::Background);
            renderer.text(&self.title_changes.label, &COLOR);
        }
        for label in self.labels.iter() {
            renderer.text(label, &COLOR)
        }
        for position in self.separator_positions {
            renderer.vertical_line_separator(position, &COLOR)
        }
    }
}
