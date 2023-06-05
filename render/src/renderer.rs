use crate::field_params::*;
use crate::ColorKey;
use common::config::{parse, parse_bool};
use common::font::*;
use common::get_bytes;
use common::hashbrown::HashMap;
use common::ini::Ini;
use common::macroquad::prelude::*;
use common::serde_json;
use common::sizes::*;
use text::Text;

const TEXTURE_COLOR: Color = common::macroquad::color::colors::WHITE;
type Focus = [bool; 2];

/// Draw shapes and text. This also stores colors, fonts, etc.
pub struct Renderer {
    /// Color key - Macroquad color map.
    colors: HashMap<ColorKey, Color>,
    /// The font for everything except subtitltes.
    font: Font,
    /// The font used for subtitles.
    subtitle_font: Font,
    /// The size of a single cell.
    cell_size: [f32; 2],
    /// The font size.
    font_size: u16,
    /// The size of the window in pixels.
    pixel_size: [f32; 2],
    /// The top-left pixel position of the subtitle text.
    subtitle_position: [f32; 2],
    /// The width of all lines.
    line_width: f32,
    /// Half-width line.
    half_line_width: f32,
    /// The offsets used when drawing a border.
    border_offsets: [f32; 4],
    /// The length of each line when drawing corners.
    corner_line_length: f32,
    /// This is used to flip captured textures.
    flip_y: bool,
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
        let subtitle_font = load_ttf_font_from_bytes(&get_bytes(
            get_font_section(config).get("subtitle_font").unwrap(),
        ))
        .unwrap();

        // Sizes.
        let font_size = get_font_size(config);
        let cell_size = get_cell_size(config);
        let grid_size = get_window_grid_size(config);
        let pixel_size = get_window_pixel_size(config);
        let subtitle_position = [cell_size[0], cell_size[1] * (grid_size[1] - 2) as f32];
        let border_offsets: [f32; 4] = [
            cell_size[0] / 2.0,
            cell_size[1] / 3.0,
            -cell_size[0],
            -cell_size[1] * (2.0 / 3.0),
        ];
        let corner_line_length = cell_size[0] / 2.0;

        // Render settings.
        let render_section = config.section(Some("RENDER")).unwrap();
        let line_width = parse(render_section, "line_width");
        let half_line_width = line_width / 2.0;
        let flip_y = parse_bool(render_section, "flip_y");

        Self {
            colors,
            font,
            subtitle_font,
            font_size,
            pixel_size,
            cell_size,
            line_width,
            half_line_width,
            corner_line_length,
            border_offsets,
            flip_y,
            subtitle_position,
        }
    }

    /// Draw a rectangle.
    ///
    /// - `position` The top-left position in grid coordinates.
    /// - `size` The width-height in grid coordinates.
    /// - `color` A `ColorKey` for the rectangle.
    pub fn rectangle(&self, position: [u32; 2], size: [u32; 2], color: &ColorKey) {
        let xy = self.grid_to_pixel(position);
        let wh = self.grid_to_pixel(size);
        let color = self.colors[color];
        draw_rectangle(xy[0], xy[1], wh[0], wh[1], color);
    }

    /// Draw a rectangle with a pixel offset.
    ///
    /// - `position` The top-left position in grid coordinates.
    /// - `position_offset` Floats between 0.0 and 1.0 to offset `position` in pixel coordinates.
    /// - `size` The width-height in grid coordinates.
    /// - `size_offset` Floats between 0.0 and 1.0 to offset `size` in pixel coordinates.
    /// - `color` A `ColorKey` for the rectangle.
    pub fn rectangle_offset(
        &self,
        position: [u32; 2],
        position_offset: [f32; 2],
        size: [u32; 2],
        size_offset: [f32; 2],
        color: &ColorKey,
    ) {
        let xy = self.grid_to_pixel(position);
        let x = xy[0] + self.cell_size[0] * position_offset[0];
        let y = xy[1] + self.cell_size[1] * position_offset[1];
        let wh = self.grid_to_pixel(size);
        let w = wh[0] + self.cell_size[0] * size_offset[0];
        let h = wh[1] + self.cell_size[1] * size_offset[1];
        let color = self.colors[color];
        draw_rectangle(x, y, w, h, color);
    }

    /// Draw a border that is slightly offset from the edges of the cells.
    ///
    /// - `position` The top-left position in grid coordinates.
    /// - `size` The width-height in grid coordinates.
    /// - `color` A `ColorKey` for the rectangle.
    pub fn border(&self, position: [u32; 2], size: [u32; 2], color: &ColorKey) {
        let mut xy = self.grid_to_pixel(position);
        xy[0] += self.border_offsets[0];
        xy[1] += self.border_offsets[1];
        let mut wh = self.grid_to_pixel(size);
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
        let mut xy = self.grid_to_pixel(label.position);
        let dim = measure_text(&label.text, Some(self.font), self.font_size, 1.0);
        xy[1] += self.cell_size[1] - dim.offset_y / 3.0;
        let color = self.colors[text_color];
        let text_params = TextParams {
            font: self.font,
            font_size: self.font_size,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            rotation: 0.0,
            color,
        };
        draw_text_ex(&label.text, xy[0], xy[1], text_params);
    }

    /// Draw corner borders around a row.
    ///
    /// - `position` The top-left position in grid coordinates.
    /// - `size` The width-height in grid coordinates.
    /// - `focus` If true, the panel has focus. This determines the color of the corners.
    pub fn corners(&self, position: [u32; 2], size: [u32; 2], focus: bool) {
        // Get the color.
        let color = self.colors[&if focus {
            ColorKey::FocusDefault
        } else {
            ColorKey::NoFocus
        }];
        // Top-left.
        let mut p = self.grid_to_pixel(position);
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
        p = self.grid_to_pixel([position[0] + size[0], position[1]]);
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
        p = self.grid_to_pixel([position[0] + size[0], position[1] + size[1]]);
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
        p = self.grid_to_pixel([position[0], position[1] + size[1]]);
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

    /// Draw an arbitrary texture.
    ///
    /// - `texture` The texture.
    /// - `position` The top-left position in grid coordinates.
    /// - `rect` An array of grid coordinates (left, top, width, height) that defines the area of the texture to draw.
    pub fn texture(&self, texture: Texture2D, position: [u32; 2], rect: Option<[u32; 4]>) {
        let xy = self.grid_to_pixel(position);
        match rect {
            Some(r) => {
                let source = Rect {
                    x: r[0] as f32 * self.cell_size[0],
                    y: r[1] as f32 * self.cell_size[1],
                    w: r[2] as f32 * self.cell_size[0],
                    h: r[3] as f32 * self.cell_size[1],
                };
                let params = DrawTextureParams {
                    source: Some(source),
                    ..Default::default()
                };
                draw_texture_ex(texture, xy[0], xy[1], TEXTURE_COLOR, params);
            }
            None => draw_texture(texture, xy[0], xy[1], TEXTURE_COLOR),
        }
    }

    /// Draw an arbitrary texture with texture parameters.
    ///
    /// - `texture` The texture.
    /// - `position` The top-left position in grid coordinates.
    /// - `params` Draw texture parameters.
    pub fn texture_ex(&self, texture: Texture2D, position: [u32; 2], params: DrawTextureParams) {
        let xy = self.grid_to_pixel(position);
        draw_texture_ex(texture, xy[0], xy[1], TEXTURE_COLOR, params);
    }

    /// Draw a line from top to bottom.
    ///
    /// - `x` The x grid coordinate.
    /// - `x_offset` A float between 0.0 and 1.0 to offset `x` in pixel coordinates. 0.5 will put the x coordinate at the mid-point of the grid cell.
    /// - `top` The top y grid coordinate.
    /// - `bottom` The bottom y grid coordinate.
    /// - `y_offsets` Two floats between 0.0 and 1.0 to offset `top` and `bottom` in pixel coordinates. 0.5 will put the y coordinate at the mid-point of the grid cell.
    /// - `color` A `ColorKey` for the rectangle.
    pub fn vertical_line(
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

    /// Draw a line from left to right.
    ///
    /// - `left` The left grid coordinate.
    /// - `right` The right grid coordinate.
    /// - `x_offsets` Two floats between 0.0 and 1.0 to offset `left` and `right` in pixel coordinates. 0.5 will put the y coordinate at the mid-point of the grid cell.
    /// - `y` The y grid coordinate.
    /// - `y_offset` A float between 0.0 and 1.0 to offset `y` in pixel coordinates. 0.5 will put the y coordinate at the mid-point of the grid cell.
    /// - `color` A `ColorKey` for the rectangle.
    pub fn horizontal_line(
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

    /// Draw subtitles.
    ///
    /// - `text` The text.
    pub fn subtitle(&self, text: &str) {
        let mut xy = self.subtitle_position;
        let dim = measure_text(text, Some(self.subtitle_font), self.font_size, 1.0);
        let color = self.colors[&ColorKey::Subtitle];
        let text_params = TextParams {
            font: self.subtitle_font,
            font_size: self.font_size,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            rotation: 0.0,
            color,
        };
        xy[1] += self.cell_size[1] - dim.offset_y / 3.0;
        let width = self.pixel_size[0] - xy[0];
        // One row.
        if dim.width < width {
            draw_text_ex(text, xy[0], xy[1], text_params);
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
                let dim = measure_text(&row1, Some(self.subtitle_font), self.font_size, 1.0);
                // The row doesn't fit.
                if dim.width >= width {
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
            for (i, row) in rows.iter().enumerate() {
                let dim = measure_text(row, Some(self.subtitle_font), self.font_size, 1.0);
                let y =
                    xy[1] - (self.cell_size[1] - dim.offset_y / 3.0) * (rows.len() - i - 1) as f32;
                draw_text_ex(row, xy[0], y, text_params);
            }
        }
    }

    /// Capture the screen, flipping the image as needed.
    pub fn screen_capture(&self) -> (Texture2D, DrawTextureParams) {
        let texture = Texture2D::from_image(&get_screen_data());
        let params = DrawTextureParams {
            flip_y: self.flip_y,
            ..Default::default()
        };
        (texture, params)
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

    /// An input box.
    ///
    /// - `text` The text.
    /// - `position` The top-left position in grid coordinates.
    /// - `focus` A two-element array. Element 0: Panel focus. Element 1: widget focus.
    pub(crate) fn input(&self, text: &str, input: &TextWidth, focus: Focus) {
        // Draw indicators of widget focus.
        if focus[1] {
            // Draw corners.
            self.corners(
                input.width.position,
                [input.width.width as u32, 1],
                focus[0],
            );
            // Draw a rectangle.
            self.rectangle(
                input.value.position,
                [input.value.width as u32, 1],
                &ColorKey::TextFieldBG,
            );
        }
        let value = input.get_value(text);
        self.text(&value, &Self::get_key_color(focus[0]));
    }

    /// Draw a key + value text input field.
    ///
    /// - `text` The value text.s
    /// - `kv` The key label and the value width.
    /// - `focus` A two-element array. Element 0: Panel focus. Element 1: widget focus.
    pub(crate) fn key_input(&self, text: &str, kv: &KeyWidth, focus: Focus) {
        // Draw indicators of widget focus.
        if focus[1] {
            // Draw corners.
            self.corners(kv.key.position, [kv.width, 1], focus[0]);
            // Draw a rectangle.
            self.rectangle(
                kv.value.position,
                [kv.value.width as u32, 1],
                &ColorKey::TextFieldBG,
            );
        }
        // Draw the key text.
        self.text(&kv.key, &Self::get_key_color(focus[0]));
        self.text(&kv.get_value(text), &Self::get_value_color(focus));
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

    /// Draw a horizontally-aligned key-value boolean pair.
    ///
    /// - `value` The value of the boolean.
    /// - `boolean` Parameters for drawing a key-value string-bool pair.
    /// - `focus` If true, the panel has focus.
    /// - `text` This is used to get the boolean data.
    pub(crate) fn boolean(&self, value: bool, boolean: &Boolean, focus: bool, text: &Text) {
        self.text(&boolean.key, &Renderer::get_key_color(focus));
        self.text(
            &boolean.get_boolean_label(value, text),
            &Renderer::get_boolean_color(focus, value),
        );
    }

    /// Returns the color of the value text.
    ///
    /// - `focus` A two-element array. Element 0: Panel focus. Element 1: widget focus.
    pub fn get_value_color(focus: Focus) -> ColorKey {
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
    pub fn get_key_color(focus: bool) -> ColorKey {
        if focus {
            ColorKey::Key
        } else {
            ColorKey::NoFocus
        }
    }

    pub fn get_boolean_color(focus: bool, value: bool) -> ColorKey {
        if !focus {
            ColorKey::NoFocus
        } else if value {
            ColorKey::True
        } else {
            ColorKey::False
        }
    }

    /// Returns a color.
    pub fn get_color(&self, color_key: &ColorKey) -> Color {
        self.colors[color_key]
    }

    /// Converts a grid point to a pixel point.
    ///
    /// - `point` The point in grid coordinates.
    fn grid_to_pixel(&self, point: [u32; 2]) -> [f32; 2] {
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
}
