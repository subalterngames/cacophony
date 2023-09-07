use crate::field_params::*;
use crate::{ColorKey, Focus};
use common::config::parse_bool;
use common::font::{get_font, get_subtitle_font};
use common::sizes::*;
use hashbrown::HashMap;
use ini::Ini;
use macroquad::prelude::*;

const TEXTURE_COLOR: Color = macroquad::color::colors::WHITE;

/// Draw shapes and text. This also stores colors, fonts, etc.
pub struct Renderer {
    /// Color key - Macroquad color map.
    colors: HashMap<ColorKey, Color>,
    /// The font for everything except subtitltes.
    font: Font,
    /// The font used for subtitles.
    subtitle_font: Font,
    /// The size of a single cell.
    pub(crate) cell_size: [f32; 2],
    /// The font size.
    font_size: u16,
    /// The top-left position of the subtitle text.
    subtitle_position: [u32; 2],
    /// The maximum width of a line of subtitles.
    max_subtitle_width: u32,
    /// The width of all lines.
    pub(crate) line_width: f32,
    /// Half-width line.
    half_line_width: f32,
    /// The offsets used when drawing a border.
    border_offsets: [f32; 4],
    /// The length of each line when drawing corners.
    corner_line_length: f32,
    /// This is used to flip captured textures.
    flip_y: bool,
    /// This is used to resize captured textures.
    pub(crate) window_pixel_size: [f32; 2],
    /// An optional background texture. This is used for popups.
    background: Option<Texture2D>,
    /// Parameters for drawing the texture.
    background_params: Option<DrawTextureParams>,
}

impl Renderer {
    pub fn new(config: &Ini) -> Self {
        // Get the color aliases.
        let aliases_section = config.section(Some("COLOR_ALIASES")).unwrap();
        let mut aliases = HashMap::new();
        for kv in aliases_section.iter() {
            aliases.insert(kv.0.to_string(), Renderer::parse_color(kv.1));
        }
        // Get the colors.
        let colors_section = config.section(Some("COLORS")).unwrap();
        let mut colors = HashMap::new();
        for kv in colors_section.iter() {
            match kv.0.parse::<ColorKey>() {
                Ok(key) => {
                    let color = match aliases.get(kv.1) {
                        Some(color) => *color,
                        None => Renderer::parse_color(kv.1),
                    };
                    colors.insert(key, color);
                }
                Err(error) => panic!("Invalid color key: {:?} {}", kv, error),
            }
        }

        // Fonts.
        let font = get_font(config);
        let subtitle_font = get_subtitle_font(config);
        let font_size = get_font_size(config);

        // Sizes.
        let cell_size = get_cell_size(config);
        let main_menu_position = get_main_menu_position(config);
        let subtitle_position = [(main_menu_position[0] + 1), (main_menu_position[1] + 1)];
        let border_offsets: [f32; 4] = [
            cell_size[0] / 2.0,
            cell_size[1] / 3.0,
            -cell_size[0],
            -cell_size[1] * (2.0 / 3.0),
        ];
        let corner_line_length = cell_size[0] / 2.0;
        let max_subtitle_width = get_main_menu_width(config) - 2;

        // Render settings.
        let render_section = config.section(Some("RENDER")).unwrap();
        let line_width = get_line_width(config);
        let half_line_width = line_width / 2.0;
        let flip_y = parse_bool(render_section, "flip_y");

        let window_pixel_size = get_window_pixel_size(config);

        Self {
            colors,
            font,
            subtitle_font,
            font_size,
            cell_size,
            line_width,
            half_line_width,
            corner_line_length,
            border_offsets,
            flip_y,
            subtitle_position,
            max_subtitle_width,
            window_pixel_size,
            background: None,
            background_params: None,
        }
    }

    /// Draw a rectangle.
    ///
    /// - `rectangle` The position and size of the bordered area.
    /// - `color` A `ColorKey` for the rectangle.
    pub(crate) fn rectangle(&self, rect: &Rectangle, color: &ColorKey) {
        let xy = self.grid_to_pixel(rect.position);
        let wh = self.grid_to_pixel(rect.size);
        let color = self.colors[color];
        draw_rectangle(xy[0], xy[1], wh[0], wh[1], color);
    }

    /// Draw a rectangle using pixel coordinates instead of grid coordinates.
    /// This is used to draw notes.
    ///
    /// - `position` The top-left position in pixel coordinates.
    /// - `size` The width-height in pixel coordinates.
    /// - `color` A `ColorKey` for the rectangle.
    pub(crate) fn rectangle_pixel(&self, position: [f32; 2], size: [f32; 2], color: &ColorKey) {
        draw_rectangle(
            position[0],
            position[1],
            size[0],
            size[1],
            self.colors[color],
        )
    }

    /// Draw a border that is slightly offset from the edges of the cells.
    ///
    /// - `rectangle` The position and size of the bordered area.
    /// - `color` A `ColorKey` for the rectangle.
    pub(crate) fn border(&self, rect: &Rectangle, color: &ColorKey) {
        let mut xy = self.grid_to_pixel(rect.position);
        xy[0] += self.border_offsets[0];
        xy[1] += self.border_offsets[1];
        let mut wh = self.grid_to_pixel(rect.size);
        wh[0] += self.border_offsets[2];
        wh[1] += self.border_offsets[3];
        let color = self.colors[color];
        draw_rectangle_lines(xy[0], xy[1], wh[0], wh[1], self.line_width, color);
    }

    /// Draw text.
    ///
    /// - `label` Parameters for drawing text.
    /// - `color` A `ColorKey` for the rectangle.
    pub(crate) fn text(&self, label: &Label, text_color: &ColorKey) {
        self.text_ex(
            label.position,
            &label.text,
            text_color,
            &self.font,
            self.font_size,
        );
    }

    /// Draw text.
    ///
    /// - `label` Parameters for drawing text.
    /// - `color` A `ColorKey` for the rectangle.
    pub(crate) fn text_ref(&self, label: &LabelRef, text_color: &ColorKey) {
        self.text_ex(
            label.position,
            label.text,
            text_color,
            &self.font,
            self.font_size,
        );
    }

    /// Draw corner borders around a rectangle.
    ///
    /// - `rectangle` The position and size of the bordered area.
    /// - `focus` If true, the panel has focus. This determines the color of the corners.
    pub(crate) fn corners(&self, rect: &Rectangle, focus: bool) {
        // Get the color.
        let color = self.colors[&if focus {
            ColorKey::FocusDefault
        } else {
            ColorKey::NoFocus
        }];
        // Top-left.
        let mut p = self.grid_to_pixel(rect.position);
        draw_line(
            p[0] - self.half_line_width,
            p[1],
            p[0] + self.corner_line_length,
            p[1],
            self.line_width,
            color,
        );
        draw_line(
            p[0],
            p[1],
            p[0],
            p[1] + self.corner_line_length,
            self.line_width,
            color,
        );
        // Top-right.
        p = self.grid_to_pixel([rect.position[0] + rect.size[0], rect.position[1]]);
        draw_line(
            p[0] - self.corner_line_length,
            p[1],
            p[0] + self.half_line_width,
            p[1],
            self.line_width,
            color,
        );
        draw_line(
            p[0],
            p[1],
            p[0],
            p[1] + self.corner_line_length,
            self.line_width,
            color,
        );
        // Bottom-right.
        p = self.grid_to_pixel([
            rect.position[0] + rect.size[0],
            rect.position[1] + rect.size[1],
        ]);
        draw_line(
            p[0] - self.corner_line_length,
            p[1],
            p[0] + self.half_line_width,
            p[1],
            self.line_width,
            color,
        );
        draw_line(
            p[0],
            p[1] - self.corner_line_length,
            p[0],
            p[1],
            self.line_width,
            color,
        );
        // Bottom-left.
        p = self.grid_to_pixel([rect.position[0], rect.position[1] + rect.size[1]]);
        draw_line(
            p[0] - self.half_line_width,
            p[1],
            p[0] + self.corner_line_length,
            p[1],
            self.line_width,
            color,
        );
        draw_line(
            p[0],
            p[1] - self.corner_line_length,
            p[0],
            p[1],
            self.line_width,
            color,
        );
    }

    /// Draw an arbitrary texture at a pixel position
    ///
    /// - `texture` The texture.
    /// - `position` The top-left position in pixel coordinates.
    /// - `rect` The area of the texture to draw.
    pub(crate) fn texture_pixel(
        &self,
        texture: &Texture2D,
        position: &[f32; 2],
        rect: Option<Rect>,
    ) {
        match rect {
            Some(rect) => {
                let params = DrawTextureParams {
                    source: Some(rect),
                    ..Default::default()
                };
                draw_texture_ex(texture, position[0], position[1], TEXTURE_COLOR, params);
            }
            None => draw_texture(texture, position[0], position[1], TEXTURE_COLOR),
        }
    }

    pub(crate) fn background(&self) {
        if let Some(texture) = &self.background {
            if let Some(params) = &self.background_params {
                draw_texture_ex(texture, 0.0, 0.0, TEXTURE_COLOR, params.clone());
            }
        }
    }

    /// Draw a line from top to bottom in pixel coordinates.
    ///
    /// - `x` The x pixel coordinate.
    /// - `top` The top y pixel coordinate.
    /// - `bottom` The bottom y pixel coordinate.
    /// - `color` A `ColorKey` for the line.
    pub(crate) fn vertical_line_pixel(&self, x: f32, top: f32, bottom: f32, color: &ColorKey) {
        draw_line(x, top, x, bottom, self.line_width, self.colors[color]);
    }

    /// Draw a a vertical line with fixed offsets that can be used as a separator between rows.
    ///
    /// - `position` The top position in grid coordinates.
    /// - `color` A `ColorKey` for the line.
    pub fn vertical_line_separator(&self, position: [u32; 2], color: &ColorKey) {
        self.vertical_line(
            position[0],
            0.5,
            position[1],
            position[1] + 1,
            [-0.6, 0.4],
            color,
        );
    }

    /// Draw a line from left to right.
    ///
    /// - `left` The left grid coordinate.
    /// - `right` The right grid coordinate.
    /// - `x_offsets` Two floats between 0.0 and 1.0 to offset `left` and `right` in pixel coordinates. 0.5 will put the y coordinate at the mid-point of the grid cell.
    /// - `y` The y grid coordinate.
    /// - `y_offset` A float between 0.0 and 1.0 to offset `y` in pixel coordinates. 0.5 will put the y coordinate at the mid-point of the grid cell.
    /// - `color` A `ColorKey` for the rectangle.
    pub(crate) fn horizontal_line(
        &self,
        left: u32,
        right: u32,
        x_offsets: [f32; 2],
        y: u32,
        y_offset: f32,
        color: &ColorKey,
    ) {
        let left = left as f32 * self.cell_size[0] + x_offsets[0] * self.cell_size[0];
        let right = right as f32 * self.cell_size[0] + x_offsets[1] * self.cell_size[0];
        let y = y as f32 * self.cell_size[1] + y_offset * self.cell_size[1];
        draw_line(left, y, right, y, self.half_line_width, self.colors[color]);
    }

    /// Draw a line from left to right using pixel coordinates.
    ///
    /// - `left` The left pixel coordinate.
    /// - `right` The right pixel coordinate.
    /// - `y` The y pixel coordinate.
    /// - `color` A `ColorKey` for the line.
    pub(crate) fn horizontal_line_pixel(&self, left: f32, right: f32, y: f32, color: &ColorKey) {
        draw_line(left, y, right, y, self.half_line_width, self.colors[color]);
    }

    /// Draw subtitles.
    ///
    /// - `text` The text.
    pub(crate) fn subtitle(&self, text: &str) {
        let width = text.chars().count() as u32;
        // One row.
        if width <= self.max_subtitle_width {
            self.rectangle(
                &Rectangle::new(self.subtitle_position, [width, 1]),
                &ColorKey::SubtitleBackground,
            );
            self.text_sub(&Label {
                position: self.subtitle_position,
                text: text.to_string(),
            });
        }
        // Multi-row.
        else {
            let mut rows = vec![];
            let mut words = text.split(' ').collect::<Vec<&str>>();
            let mut row = String::new();
            while !words.is_empty() {
                let mut row1 = row.clone();
                row1.push(' ');
                row1.push_str(words[0]);
                let width = row1.chars().count() as u32;
                // The row doesn't fit.
                if width > self.max_subtitle_width {
                    // Add the row.
                    rows.push(row.trim().to_string());
                    row = words[0].to_string();
                    words.remove(0);
                }
                // Append the row.
                else {
                    row.push(' ');
                    row.push_str(words[0]);
                    words.remove(0);
                }
            }
            // Last row.
            if row.chars().count() > 0 {
                rows.push(row.trim().to_string());
            }
            let mut y = self.subtitle_position[1];
            for row in rows {
                self.rectangle(
                    &Rectangle::new(
                        [self.subtitle_position[0], y],
                        [row.chars().count() as u32, 1],
                    ),
                    &ColorKey::SubtitleBackground,
                );
                self.text_sub(&Label {
                    position: [self.subtitle_position[0], y],
                    text: row,
                });
                y += 1;
            }
        }
    }

    /// Capture the screen and update the cached background texture.
    pub(crate) fn screen_capture(&mut self) {
        let screen_data = get_screen_data();
        match self.background.as_mut() {
            Some(background) => {
                background.update(&screen_data);
            }
            None => {
                self.background = Some(Texture2D::from_image(&screen_data));
                let dest_size = Some(Vec2::new(
                    self.window_pixel_size[0],
                    self.window_pixel_size[1],
                ));
                self.background_params = Some(DrawTextureParams {
                    flip_y: self.flip_y,
                    dest_size,
                    ..Default::default()
                });
            }
        }
    }

    /// Draw a value with left and right arrows with a key.
    ///
    /// - `text` The value text.
    /// - `key_list` The key-list parameters pair.
    /// - `focus` A two-element array. Element 0: Panel focus. Element 1: widget focus.
    pub(crate) fn key_list(&self, text: &str, key_list: &KeyList, focus: Focus) {
        // Draw the key.
        self.text(&key_list.key, &Renderer::get_key_color(focus[0]));
        // Draw the value.
        self.list(text, &key_list.value, focus);
    }

    /// Draw a value with left and right arrows with a key, and possible corners.
    ///
    /// - `text` The value text.
    /// - `key_list` The key-list parameters pair.
    /// - `focus` A two-element array. Element 0: Panel focus. Element 1: widget focus.
    pub(crate) fn key_list_corners(&self, text: &str, key_list: &KeyListCorners, focus: Focus) {
        // Draw corners.
        if focus[1] {
            self.corners(&key_list.corners_rect, focus[0]);
        }
        // Draw the key.
        self.text(&key_list.key_list.key, &Renderer::get_key_color(focus[0]));
        // Draw the value.
        self.list(text, &key_list.key_list.value, focus);
    }

    /// Draw a value with left and right arrows.
    ///
    /// - `text` The text in the label.
    /// - `list` The `List` draw parameters.
    /// - `focus` A two-element array. Element 0: Panel focus. Element 1: widget focus.
    pub(crate) fn list(&self, text: &str, list: &List, focus: Focus) {
        // Draw the arrows.
        if focus[1] {
            let arrow_color = if focus[0] {
                ColorKey::Arrow
            } else {
                ColorKey::NoFocus
            };
            self.text(&list.left_arrow, &arrow_color);
            self.text(&list.right_arrow, &arrow_color);
        }
        // Get the label.
        let value = list.get_value(text);
        // Draw the value.
        self.text(&value, &Self::get_value_color(focus));
    }

    /// Draw a key-value pair.
    ///
    /// - `text` The value text.
    /// - `kv` Draw parameters for the key-value pair.
    /// - `colors` The key and value colors.
    pub(crate) fn key_value(&self, text: &str, kv: &KeyWidth, colors: [&ColorKey; 2]) {
        self.text(&kv.key, colors[0]);
        self.text(&kv.get_value(text), colors[1]);
    }

    /// Draw a key-input pair.
    ///
    /// - `text` The text in the label.
    /// - `ki` The `KeyInput` draw parameters.
    /// - `alphanumeric_input` If true, alphanumeric input is enabled.
    /// - `focus` A two-element array. Element 0: Panel focus. Element 1: Widget focus.
    pub(crate) fn key_input(
        &self,
        value: &str,
        ki: &KeyInput,
        alphanumeric_input: bool,
        focus: Focus,
    ) {
        if focus[1] {
            // Draw corners.
            self.corners(&ki.corners_rect, focus[0]);
            // Draw a rectangle for input.
            if alphanumeric_input {
                self.rectangle(&ki.input_rect, &ColorKey::TextFieldBG);
            }
        }
        let key_color = &Self::get_key_color(focus[0]);
        if value.is_empty() {
            self.text(&ki.key_width.key, key_color);
        } else {
            // Draw the key-value pair.
            self.key_value(
                value,
                &ki.key_width,
                [
                    &Self::get_key_color(focus[0]),
                    &Self::get_value_color(focus),
                ],
            );
        }
    }

    /// Draw a horizontally-aligned key-value boolean pair.
    ///
    /// - `value` The value of the boolean.
    /// - `boolean` Parameters for drawing a key-value string-bool pair.
    /// - `focus` If true, the panel has focus.
    pub(crate) fn boolean(&self, value: bool, boolean: &Boolean, focus: bool) {
        self.text(&boolean.key, &Renderer::get_key_color(focus));
        self.text(
            boolean.get_boolean_label(&value),
            &Renderer::get_boolean_color(focus, value),
        );
    }

    /// Draw a horizontally-aligned key-value boolean pair with corners.
    ///
    /// - `value` The value of the boolean.
    /// - `boolean` Parameters for drawing a key-value string-bool pair.
    /// - `focus` If true, the panel has focus.
    pub(crate) fn boolean_corners(&self, value: bool, boolean: &BooleanCorners, focus: Focus) {
        if focus[1] {
            // Draw corners.
            self.corners(&boolean.corners_rect, focus[0]);
        }
        self.boolean(value, &boolean.boolean, focus[0]);
    }

    /// Returns the color of the value text.
    ///
    /// - `focus` A two-element array. Element 0: Panel focus. Element 1: widget focus.
    pub(crate) fn get_value_color(focus: Focus) -> ColorKey {
        match (focus[0], focus[1]) {
            (true, true) => ColorKey::Value,
            (true, false) => ColorKey::Key,
            (false, true) => ColorKey::NoFocus,
            (false, false) => ColorKey::NoFocus,
        }
    }

    /// Returns the color of the key text.
    ///
    /// - `focus` If true, the panel has focus.
    pub(crate) fn get_key_color(focus: bool) -> ColorKey {
        if focus {
            ColorKey::Key
        } else {
            ColorKey::NoFocus
        }
    }

    /// Returns the color of a boolean value.
    pub(crate) fn get_boolean_color(focus: bool, value: bool) -> ColorKey {
        if !focus {
            ColorKey::NoFocus
        } else if value {
            ColorKey::True
        } else {
            ColorKey::False
        }
    }

    /// Returns a color.
    pub(crate) fn get_color(&self, color_key: &ColorKey) -> Color {
        self.colors[color_key]
    }

    /// Converts a grid point to a pixel point.
    ///
    /// - `point` The point in grid coordinates.
    pub(crate) fn grid_to_pixel(&self, point: [u32; 2]) -> [f32; 2] {
        [
            point[0] as f32 * self.cell_size[0],
            point[1] as f32 * self.cell_size[1],
        ]
    }

    /// Parse a serialized 3-element array as an RGBA color.
    fn parse_color(value: &str) -> Color {
        let c: Result<[u8; 3], serde_json::Error> = serde_json::from_str(value);
        match c {
            Ok(c) => color_u8!(c[0], c[1], c[2], 255),
            Err(error) => panic!("Invalid color alias: {} {}", value, error),
        }
    }

    /// Draw subtitle text.
    ///
    /// - `label` Parameters for drawing text.
    fn text_sub(&self, label: &Label) {
        self.text_ex(
            label.position,
            &label.text,
            &ColorKey::Subtitle,
            &self.subtitle_font,
            self.font_size,
        );
    }

    /// Draw a line from top to bottom.
    ///
    /// - `x` The x grid coordinate.
    /// - `x_offset` A float between 0.0 and 1.0 to offset `x` in pixel coordinates. 0.5 will put the x coordinate at the mid-point of the grid cell.
    /// - `top` The top y grid coordinate.
    /// - `bottom` The bottom y grid coordinate.
    /// - `y_offsets` Two floats between 0.0 and 1.0 to offset `top` and `bottom` in pixel coordinates. 0.5 will put the y coordinate at the mid-point of the grid cell.
    /// - `color` A `ColorKey` for the line.
    fn vertical_line(
        &self,
        x: u32,
        x_offset: f32,
        top: u32,
        bottom: u32,
        y_offsets: [f32; 2],
        color: &ColorKey,
    ) {
        let x = x as f32 * self.cell_size[0] + x_offset * self.cell_size[0];
        let top = top as f32 * self.cell_size[1] + y_offsets[0] * self.cell_size[1];
        let bottom = bottom as f32 * self.cell_size[1] + y_offsets[1] * self.cell_size[1];
        draw_line(x, top, x, bottom, self.line_width, self.colors[color]);
    }

    /// Draw text.
    ///
    /// - `position` The position of the text.
    /// - `text` The text.
    /// - `color` A `ColorKey` for the rectangle.
    /// - `font` The font.
    /// - `font_size` The font size.
    fn text_ex(
        &self,
        position: [u32; 2],
        text: &str,
        text_color: &ColorKey,
        font: &Font,
        font_size: u16,
    ) {
        let font = Some(font);
        let mut xy = self.grid_to_pixel(position);
        let dim = measure_text(text, font, font_size, 1.0);
        xy[1] += self.cell_size[1] - dim.offset_y / 3.0;
        let color = self.colors[text_color];
        let text_params = TextParams {
            font,
            font_size,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            rotation: 0.0,
            color,
        };
        draw_text_ex(text, xy[0], xy[1], text_params);
    }
}
