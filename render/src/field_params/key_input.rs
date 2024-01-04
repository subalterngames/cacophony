use super::{KeyWidth, RectanglePixel};
use crate::Renderer;

/// A key, a value, a rectangle for corners, and a rectangle for input.
pub(crate) struct KeyInput {
    /// The key and the input.
    pub key_width: KeyWidth,
    /// A rectangle that will be used to render corners when focused.
    pub corners_rect: RectanglePixel,
    /// A rectangle that will appear under the input text when focused and selected.
    pub input_rect: RectanglePixel,
}

impl KeyInput {
    /// - The `key` is at `position[0] + 1`.
    /// - The value is at a position that tries to fill `width - 2`.
    /// - The `corners_rect` is at `position` and is size `[width, 1]`.
    /// - The `input_rect` is at `position[0] + 1`.
    ///
    /// The `key` will be truncated and the `value` will match `value_width`.
    pub fn new_from_value_width(
        key: &str,
        position: [u32; 2],
        width: u32,
        value_width: u32,
        renderer: &Renderer,
    ) -> Self {
        let key_width = KeyWidth::new_from_width(
            key,
            [position[0] + 1, position[1]],
            width - 2,
            value_width,
            renderer,
        );
        let corners_rect = RectanglePixel::new_from_u(position, [width, 1], renderer);
        let input_rect = RectanglePixel::new_from_u(
            key_width.value.position,
            [key_width.value.width_u32, 1],
            renderer,
        );
        Self {
            key_width,
            corners_rect,
            input_rect,
        }
    }

    /// - The `key` is at `position[0] + 1`.
    /// - The value is at `position[0] + 1 + key_width + padding`.
    /// - The `corners_rect` is at `position` and is size `[width, 1]`.
    ///
    /// The `key` won't be truncated. The `value` will be trunacted.
    pub fn new_from_padding(
        key: &str,
        value: &str,
        position: [u32; 2],
        width: u32,
        padding: u32,
        renderer: &Renderer,
    ) -> Self {
        let key_width_x = position[0] + 1;
        // The input rect can be larger than the value width.
        let input_x = key_width_x + key.chars().count() as u32 + padding;
        let input_width = width - 2 - (input_x - key_width_x);
        let mut value_width = value.chars().count() as u32;
        // Truncate to the input width.
        if input_width < value_width {
            value_width = input_width;
        }
        let input_rect =
            RectanglePixel::new_from_u([input_x, position[1]], [input_width, 1], renderer);
        let key_width = KeyWidth::new_from_width(
            key,
            [position[0] + 1, position[1]],
            width - 2,
            value_width,
            renderer,
        );
        let corners_rect = RectanglePixel::new_from_u(position, [width, 1], renderer);
        Self {
            key_width,
            corners_rect,
            input_rect,
        }
    }
}
