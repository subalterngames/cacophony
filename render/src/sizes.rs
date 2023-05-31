use crate::get_bytes;
use common::config::parse;
use common::ini::{Ini, Properties};
use common::macroquad::prelude::*;

/// Returns the font data section in the config file.
fn get_font_section(config: &Ini) -> &Properties {
    config.section(Some("FONTS")).unwrap()
}

/// Returns the main font.
pub(crate) fn get_font(config: &Ini) -> Font {
    let bytes = get_bytes(get_font_section(config).get("font").unwrap());
    load_ttf_font_from_bytes(&bytes).unwrap()
}

/// Returns the font height.
pub(crate) fn get_font_size(config: &Ini) -> u16 {
    parse(get_font_section(config), "font_height")
}

/// Returns the size of a cell in pixels (width, height).
pub(crate) fn get_cell_size(config: &Ini) -> [f32; 2] {
    let font_size: u16 = get_font_size(config);
    let font = get_font(config);
    let size = measure_text("█", Some(font), font_size, 1.0);
    [size.width, size.height]
}

/// Returns the window size in grid units.
pub fn get_window_grid_size(config: &Ini) -> [u32; 2] {
    let section = config.section(Some("RENDER")).unwrap();
    [
        parse(section, "window_width"),
        parse(section, "window)height"),
    ]
}

/// Returns the window size in pixels.
pub fn get_window_pixel_size(config: &Ini) -> [f32; 2] {
    let grid_size = get_window_grid_size(config);
    let cell_size = get_cell_size(config);
    [
        cell_size[0] * grid_size[0] as f32,
        cell_size[1] * grid_size[1] as f32,
    ]
}
