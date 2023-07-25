use crate::{ColorKey, Renderer};
use common::font::get_font_bytes;
use common::sizes::*;
use image::Rgba;
use imageproc::drawing::text_size;
use ini::Ini;
use rusttype::{Font, Scale};

/// Returns a `rusttype` font, the scale, and a font size.
pub(super) fn get_font(config: &Ini) -> (Font, Scale, (i32, i32)) {
    let font = Font::try_from_vec(get_font_bytes(config)).unwrap();
    let cell_size = get_cell_size(config);
    // Get the scale.
    let scale = Scale {
        x: cell_size[1] + 1.0,
        y: cell_size[1] + 1.0,
    };
    let font_size = text_size(scale, &font, "█");
    (font, scale, font_size)
}

/// Converts a macroquad color to an image color.
pub(super) fn get_color(color: &ColorKey, renderer: &Renderer) -> Rgba<u8> {
    let color: [u8; 4] = renderer.get_color(color).into();
    Rgba(color)
}
