use crate::get_bytes;
use crate::paths::get_data_directory;
use ini::{Ini, Properties};
use macroquad::prelude::*;

/// Returns the font data section in the config file.
pub fn get_font_section(config: &Ini) -> &Properties {
    config.section(Some("FONTS")).unwrap()
}

/// Reads the font to a byte buffer.
pub fn get_font_bytes(config: &Ini) -> Vec<u8> {
    get_font_from_bytes(config, "font")
}

/// Returns the main font.
pub fn get_font(config: &Ini) -> Font {
    load_ttf_font_from_bytes(&get_font_bytes(config)).unwrap()
}

/// Returns the subtitle font.
pub fn get_subtitle_font(config: &Ini) -> Font {
    load_ttf_font_from_bytes(&get_font_from_bytes(config, "subtitle_font")).unwrap()
}

/// Returns the path to a font.
fn get_font_from_bytes(config: &Ini, key: &str) -> Vec<u8> {
    get_bytes(
        get_data_directory()
            .join(get_font_section(config).get(key).unwrap())
            .to_str()
            .unwrap(),
    )
}
