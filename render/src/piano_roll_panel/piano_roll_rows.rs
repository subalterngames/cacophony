use crate::panel::Rectangle;
use crate::{ColorKey, Renderer};
use common::view::View;
use common::{PanelType, State, U64orF32};
use macroquad::prelude::*;

const BACKGROUND_COLOR: ColorKey = ColorKey::Background;

pub(crate) struct PianoRollRows {
    /// A 1-D linear RGBA buffer defining a row. We'll use this to quickly write simple texture data.
    row: Vec<u8>,
    // The position of each row on the screen.
    positions: Vec<[f32; 2]>,
    /// The viewport rectangle.
    rect: Rectangle,
    /// A copy of the view. This is used to decide whether we need to redraw the rows.
    view: View,
    /// As far as this struct knows, this is whether the piano roll panel has focus.
    focus: bool,
    /// As far as this struct knows, this is the input beat.
    beat: U64orF32,
    /// The row texture.
    texture: Texture2D,
    /// The parameters used to draw each row (the actual size).
    texture_params: DrawTextureParams,
}

impl PianoRollRows {
    pub fn new(rect: Rectangle, state: &State, renderer: &Renderer) -> Self {
        // Get the pixel width of the viewport.
        let width = rect.size[0] as f32 * renderer.cell_size[0];
        // Define the row buffer.
        let mut row = vec![0u8; width as usize * 4];
        // Get the half-height of each cell. This will be used to position the lines in the vertical-center of the cell.
        let half_height = renderer.cell_size[1] / 2.0;
        // Derive the positions of each row from the dimensions of the viewport.
        let positions = (0..rect.size[1])
            .map(|y| {
                let p = renderer.grid_to_pixel([rect.position[0], rect.position[1] + y]);
                [p[0], p[1] + half_height]
            })
            .collect();
        let texture = Self::get_row_texture(&mut row, false, state, renderer);
        let texture_params = DrawTextureParams {
            dest_size: Some(Vec2::new(width, renderer.line_width)),
            ..Default::default()
        };
        Self {
            row,
            positions,
            rect,
            view: state.view.clone(),
            focus: false,
            beat: state.input.beat,
            texture,
            texture_params,
        }
    }

    /// Draw the rows.
    pub fn update(&self, renderer: &Renderer) {
        // Draw the background.
        renderer.rectangle(&self.rect, &BACKGROUND_COLOR);
        // Draw each row.
        self.positions
            .iter()
            .for_each(|p| renderer.texture_pixel_ex(&self.texture, p, &self.texture_params));
    }

    /// Check if we need to re-define the row pattern and, if so, do it.
    pub fn late_update(&mut self, state: &State, renderer: &Renderer) {
        let focus = state.panels[state.focus.get()] == PanelType::PianoRoll;
        // The focus or the view changed.
        if state.view.single_track
            && (focus != self.focus || self.beat != state.input.beat || state.view != self.view)
        {
            self.focus = focus;
            self.beat = state.input.beat;
            self.view = state.view.clone();
            self.texture = Self::get_row_texture(&mut self.row, focus, state, renderer);
        }
    }

    /// Write color data to the row buffer and use it to create a very thin texture.
    fn get_row_texture(
        row: &mut [u8],
        focus: bool,
        state: &State,
        renderer: &Renderer,
    ) -> Texture2D {
        let width = row.len() / 4;
        // Get the length of each line segment as a fraction of the viewport time-width.
        let line_segment_width = (width as f32
            * ((state.input.beat.get_f() - state.view.dt[0] as f32)
                / (state.view.dt[1] as f32 - state.view.dt[0] as f32)))
            .clamp(1.0, f32::MAX)
            .floor() as usize;
        let color: [u8; 4] = renderer
            .get_color(if focus {
                &ColorKey::Separator
            } else {
                &ColorKey::NoFocus
            })
            .into();
        let clear = [0u8; 4];
        // Copy the color into the row.
        let mut draw_color = false;
        let lsw = line_segment_width * 4;
        for i in (0..row.len()).step_by(4) {
            if i % lsw == 0 {
                draw_color = !draw_color;
            }
            row[i..i + 4].copy_from_slice(if draw_color { &color } else { &clear });
        }
        Texture2D::from_rgba8(width as u16, 1, row)
    }
}
