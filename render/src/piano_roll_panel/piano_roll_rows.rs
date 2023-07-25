use super::image::*;
use crate::{ColorKey, FocusableTexture, Renderer};
use common::sizes::*;
use image::imageops::overlay;
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_line_segment_mut;
use ini::Ini;
use macroquad::texture::Texture2D;

/// Returns a `FocusableTexture` of horizontal lines where the notes will be rendered.
pub(crate) fn get_piano_roll_rows(config: &Ini, renderer: &Renderer) -> FocusableTexture {
    // Get the cell size.
    let cell_size = get_cell_size(config);

    // Get the image size.
    let viewport_size = get_viewport_size(config);
    let height_pixels = (viewport_size[1] as f32 * cell_size[1]) as u32;
    let width_pixels = (viewport_size[0] as f32 * cell_size[0]) as u32;

    // Get the background color.
    let bg_color = get_color(&ColorKey::Background, renderer);

    // Get the font width.
    let font_size = get_font(config).2;
    let line_width = get_line_width(config) as usize;
    let pixel_size = [width_pixels, height_pixels];
    let grid_height = viewport_size[1] as i64;
    let cell_height = cell_size[1] as u32;

    let focus = get_texture(
        grid_height,
        cell_height,
        pixel_size,
        font_size,
        line_width,
        get_color(&ColorKey::Separator, renderer),
        bg_color,
    );
    let no_focus = get_texture(
        grid_height,
        cell_height,
        pixel_size,
        font_size,
        line_width,
        get_color(&ColorKey::NoFocus, renderer),
        bg_color,
    );
    FocusableTexture::new(focus, no_focus)
}

fn get_texture(
    grid_height: i64,
    cell_height: u32,
    pixel_size: [u32; 2],
    font_size: (i32, i32),
    line_width: usize,
    color: Rgba<u8>,
    bg_color: Rgba<u8>,
) -> Texture2D {
    let font_width = font_size.0 as f32;
    // Get a single row.
    let mut row = RgbaImage::from_pixel(pixel_size[0], cell_height, bg_color);
    let y = (cell_height / 2) as f32;
    let dash_w = (font_width / 6.0) + 1.0;
    let mut x = 0.0;
    let x1 = pixel_size[0] as f32;
    let x_offset = font_width / 2.0 - dash_w / 2.0;
    while x < x1 {
        for h in 0..line_width {
            draw_line_segment_mut(
                &mut row,
                (x + x_offset, y + h as f32),
                (x + x_offset + dash_w, y + h as f32),
                color,
            );
        }
        x += font_width;
    }
    // Create the image.
    let mut image = RgbaImage::from_pixel(pixel_size[0], pixel_size[1], bg_color);
    // Copy the row.
    let x: i64 = 0;
    let font_height = font_size.1 as i64;
    for y in 0..grid_height {
        overlay(&mut image, &row, x, font_height * y);
    }
    Texture2D::from_rgba8(pixel_size[0] as u16, pixel_size[1] as u16, &image)
}
