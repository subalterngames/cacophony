use super::image::*;
use crate::{ColorKey, FocusableTexture, Renderer};
use common::sizes::*;
use common::{MAX_NOTE, MIN_NOTE, NOTE_NAMES};
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use ini::Ini;
use macroquad::texture::Texture2D;
use rusttype::{Font, Scale};

pub(crate) fn get_note_names(config: &Ini, renderer: &Renderer) -> FocusableTexture {
    let (font, font_scale, font_size) = get_font(config);
    // Get the background color.
    let bg_color = get_color(&ColorKey::Background, renderer);

    // Get the width of the image.
    let font_size = [font_size.0 as u32, font_size.1 as u32];
    let pixel_width = PIANO_ROLL_PANEL_NOTE_NAMES_WIDTH * font_size[0];

    // Get the height of the image.
    let dn = MAX_NOTE - MIN_NOTE;
    let pixel_height = dn as u32 * font_size[1];
    let font_height = font_size[1] as i32;

    let pixel_size = [pixel_width, pixel_height];

    // Get the textures.
    let focus = get_texture(
        pixel_size,
        dn,
        &font,
        font_scale,
        font_height,
        get_color(&ColorKey::Separator, renderer),
        bg_color,
    );
    let no_focus = get_texture(
        pixel_size,
        dn,
        &font,
        font_scale,
        font_height,
        get_color(&ColorKey::NoFocus, renderer),
        bg_color,
    );
    FocusableTexture::new(focus, no_focus)
}

/// Returns a note names texture.
fn get_texture(
    pixel_size: [u32; 2],
    dn: u8,
    font: &Font,
    font_scale: Scale,
    font_height: i32,
    color: Rgba<u8>,
    bg_color: Rgba<u8>,
) -> Texture2D {
    // Create the image.
    let mut image = RgbaImage::from_pixel(pixel_size[0], pixel_size[1], bg_color);
    for (y, note_name) in (0..dn as i32).zip(NOTE_NAMES) {
        draw_text_mut(
            &mut image,
            color,
            0,
            y * font_height,
            font_scale,
            font,
            note_name,
        );
    }
    Texture2D::from_rgba8(pixel_size[0] as u16, pixel_size[1] as u16, &image)
}
