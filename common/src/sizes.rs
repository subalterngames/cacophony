use crate::config::parse;
use crate::font::*;
use ini::Ini;
use macroquad::prelude::*;

/// The height of the main menu in grid units.
pub const MAIN_MENU_HEIGHT: u32 = 3;
/// The position of the music panel in grid units.
pub const MUSIC_PANEL_POSITION: [u32; 2] = [0, 0];
/// The height of the music panel.
pub const MUSIC_PANEL_HEIGHT: u32 = 6;
/// The height of the piano roll panel's top bar.
pub const PIANO_ROLL_PANEL_TOP_BAR_HEIGHT: u32 = 3;
/// The width of the column of note names.
pub const PIANO_ROLL_PANEL_NOTE_NAMES_WIDTH: u32 = 3;
/// The height of the piano roll volume sub-panel.
pub const PIANO_ROLL_PANEL_VOLUME_HEIGHT: u32 = 5;
/// The height of the prompt for the open-file panel.
pub const OPEN_FILE_PANEL_PROMPT_HEIGHT: u32 = 3;

/// Returns the font height.
pub fn get_font_size(config: &Ini) -> u16 {
    parse(get_font_section(config), "font_height")
}

/// Returns the subtitle font height.
pub fn get_subtitle_font_size(config: &Ini) -> u16 {
    parse(get_font_section(config), "subtitle_font_height")
}

/// Returns the size of a cell in pixels (width, height).
pub fn get_cell_size(config: &Ini) -> [f32; 2] {
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
        parse(section, "window_height"),
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

/// Returns the size of the piano roll panel.
pub fn get_piano_roll_panel_position(config: &Ini) -> [u32; 2] {
    let tracks_panel_width = get_tracks_panel_width(config);
    [tracks_panel_width, MAIN_MENU_HEIGHT]
}

/// Returns the size of the piano roll panel.
pub fn get_piano_roll_panel_size(config: &Ini) -> [u32; 2] {
    let tracks_panel_width = get_tracks_panel_width(config);
    let window_grid_size = get_window_grid_size(config);
    [
        window_grid_size[0] - tracks_panel_width,
        window_grid_size[1] - MAIN_MENU_HEIGHT - PIANO_ROLL_PANEL_VOLUME_HEIGHT,
    ]
}

/// Returns the width of the tracks panel.
pub fn get_tracks_panel_width(config: &Ini) -> u32 {
    parse(
        config.section(Some("RENDER")).unwrap(),
        "tracks_panel_width",
    )
}

/// Returns the pixel width of all lines.
pub fn get_line_width(config: &Ini) -> f32 {
    parse(config.section(Some("RENDER")).unwrap(), "line_width")
}

/// Returns the size of the piano roll viewport.
pub fn get_viewport_size(config: &Ini) -> [u32; 2] {
    let piano_roll_panel_size = get_piano_roll_panel_size(config);
    let width = piano_roll_panel_size[0] - PIANO_ROLL_PANEL_NOTE_NAMES_WIDTH - 2;
    let height = piano_roll_panel_size[1] - PIANO_ROLL_PANEL_TOP_BAR_HEIGHT - 2;
    [width, height]
}

/// Returns the position and dimensions of the open-file panel.
pub fn get_open_file_rect(config: &Ini) -> ([u32; 2], [u32; 2]) {
    let window_grid_size = get_window_grid_size(config);
    let size = [window_grid_size[0] / 2, window_grid_size[1] / 2];
    let position = [window_grid_size[0] / 2 - size[0] / 2, MAIN_MENU_HEIGHT];
    (position, size)
}

/// Returns the position of the main menu.
pub fn get_main_menu_position(config: &Ini) -> [u32; 2] {
    let tracks_panel_width = get_tracks_panel_width(config);
    [
        MUSIC_PANEL_POSITION[0] + tracks_panel_width,
        MUSIC_PANEL_POSITION[1],
    ]
}
