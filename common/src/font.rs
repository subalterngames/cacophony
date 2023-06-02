use crate::get_bytes;
use ini::{Ini, Properties};
use macroquad::prelude::*;

/// Returns the font data section in the config file.
pub fn get_font_section(config: &Ini) -> &Properties {
    config.section(Some("FONTS")).unwrap()
}

/// Returns the main font.
pub fn get_font(config: &Ini) -> Font {
    let bytes = get_bytes(get_font_section(config).get("font").unwrap());
    load_ttf_font_from_bytes(&bytes).unwrap()
}
