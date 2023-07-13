use crate::get_bytes;
use ini::{Ini, Properties};
use macroquad::prelude::*;

/// Returns the font data section in the config file.
pub fn get_font_section(config: &Ini) -> &Properties {
    config.section(Some("FONTS")).unwrap()
}

/// Reads the font to a byte buffer.
pub fn get_font_bytes(config: &Ini) -> Vec<u8> {
    get_bytes(get_font_section(config).get("font").unwrap())
}

/// Returns the main font.
pub fn get_font(config: &Ini) -> Font {
    load_ttf_font_from_bytes(&get_font_bytes(config)).unwrap()
}

pub fn get_subtitle_font(config: &Ini) -> Font {
    load_ttf_font_from_bytes(&get_bytes(
        get_font_section(config).get("subtitle_font").unwrap(),
    ))
    .unwrap()
}
