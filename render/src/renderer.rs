use crate::ColorKey;
use common::config::{parse, parse_bool};
use common::hashbrown::HashMap;
use common::ini::Ini;
use common::macroquad::prelude::*;
use serde_json::{from_str, Error};

pub struct Renderer {
    /// Color key - Macroquad color map.
    colors: HashMap<ColorKey, Color>,
    /// The font for everything except subtitltes.
    font: Font,
    /// The font used for subtitles.
    subtitle_font: Font,
    /// The size of a single cell.
    cell_size: [f32; 2],
    /// The top-left pixel position of the subtitle text.
    subtitle_position: [f32; 2],
    /// This is used to flip captured textures.
    flip_y: bool,
}

impl Renderer {
    pub async fn new(config: &Ini) -> Self {
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
        let fonts_section = config.section(Some("FONTS")).unwrap();
        let font = load_ttf_font(fonts_section.get("font").unwrap()).await.unwrap();
        let subtitle_font = load_ttf_font(fonts_section.get("subtitle_font").unwrap()).await.unwrap();
        let font_size: u16 = parse(fonts_section, "font_height");
        let size = measure_text("█", Some(font), font_size, 1.0);
        let cell_size = [size.width, size.height];

        // Render settings.
        let render_section = config.section(Some("RENDER")).unwrap();
        let flip_y = parse_bool(render_section, "flip_y");

        Self { colors, font, subtitle_font, cell_size, flip_y }
    }

    /// Parse a serialized 3-element array as an RGBA color.
    fn parse_color(value: &str) -> Color {
        let c: Result<[u8; 3], Error> = from_str(value);
        match c {
            Ok(c) => color_u8!(c[0], c[1], c[2], 255),
            Err(error) => panic!("Invalid color alias: {} {}", value, error),
        }
    }
}
