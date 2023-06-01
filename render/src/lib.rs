mod color_key;
mod drawable;
mod field;
mod main_menu;
mod music_panel;
mod panel;
mod panels;
mod renderer;
mod sizes;
mod tracks_panel;
pub(crate) use color_key::ColorKey;
use common::get_bytes;
pub(crate) use common::ini::Ini;
use common::ini::Properties;
use common::macroquad::prelude::*;
pub use panels::Panels;
pub use sizes::*;
pub use renderer::Renderer;

/// Returns the font data section in the config file.
fn get_font_section(config: &Ini) -> &Properties {
    config.section(Some("FONTS")).unwrap()
}

/// Returns the main font.
fn get_font(config: &Ini) -> Font {
    let bytes = get_bytes(get_font_section(config).get("font").unwrap());
    load_ttf_font_from_bytes(&bytes).unwrap()
}