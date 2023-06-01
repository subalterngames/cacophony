use crate::{get_font, get_font_section, Ini};
use common::config::parse;
use common::macroquad::prelude::*;

/// The height of the main menu in grid units.
pub(crate) const MAIN_MENU_HEIGHT: u32 = 3;
/// The position of the music panel in grid units.
pub(crate) const MUSIC_PANEL_POSITION: [u32; 2] = [0, 0];
/// The height of the music panel.
pub const MUSIC_PANEL_HEIGHT: u32 = 5;

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
pub(crate) fn get_window_grid_size(config: &Ini) -> [u32; 2] {
    let section = config.section(Some("RENDER")).unwrap();
    [
        parse(section, "window_width"),
        parse(section, "window_height"),
    ]
}

/// Returns the window size in pixels.
pub(crate) fn get_window_pixel_size(config: &Ini) -> [f32; 2] {
    let grid_size = get_window_grid_size(config);
    let cell_size = get_cell_size(config);
    [
        cell_size[0] * grid_size[0] as f32,
        cell_size[1] * grid_size[1] as f32,
    ]
}

/// Returns the width of the tracks panel.
pub(crate) fn get_tracks_panel_width(config: &Ini) -> u32 {
    parse(
        config.section(Some("RENDER")).unwrap(),
        "tracks_panel_width",
    )
}