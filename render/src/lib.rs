mod color_key;
mod renderer;
mod sizes;
pub(crate) use color_key::ColorKey;
pub(crate) use common::ini::Ini;
use common::ini::Properties;
use common::macroquad::prelude::*;
pub use renderer::Renderer;
use common::get_bytes;

/// Returns the font data section in the config file.
fn get_font_section(config: &Ini) -> &Properties {
    config.section(Some("FONTS")).unwrap()
}

/// Returns the main font.
fn get_font(config: &Ini) -> Font {
    let bytes = get_bytes(get_font_section(config).get("font").unwrap());
    load_ttf_font_from_bytes(&bytes).unwrap()
}