use crate::panel::Rectangle;
use crate::{ColorKey, Renderer};
use common::view::View;
use common::{PanelType, State, U64orF32};
use macroquad::prelude::*;

const BACKGROUND_COLOR: ColorKey = ColorKey::Background;

pub(crate) struct PianoRollRows {
    /// A RGBA buffer defining a row. We'll use this to quickly write simple texture data.
    row: Vec<u8>,
    /// A 1-D RGBA buffer used to fill `row`.
    sub_row: Vec<u8>,
    width: f32,
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
}

impl PianoRollRows {
    pub fn new(rect: Rectangle, state: &State, renderer: &Renderer) -> Self {
        // Get the pixel width of the viewport.
        let width = rect.size[0] as f32 * renderer.cell_size[0];
        // Define the row buffers.
        let row_width = width as usize * 4;
        let mut row = vec![0u8; row_width * renderer.line_width as usize];
        let mut sub_row = vec![0u8; row_width];
        // Get the half-height of each cell. This will be used to position the lines in the vertical-center of the cell.
        let half_height = renderer.cell_size[1] / 2.0;
        // Derive the positions of each row from the dimensions of the viewport.
        let positions = (0..rect.size[1])
            .map(|y| {
                let p = renderer.grid_to_pixel([rect.position[0], rect.position[1] + y]);
                [p[0], p[1] + half_height]
            })
            .collect();
        let mut texture = Texture2D::from_rgba8(width as u16, renderer.line_width as u16, &row);
        Self::set_row_texture(&mut texture, &mut sub_row, &mut row, width, false, state, renderer);
        Self {
            row,
            sub_row,
            width,
            positions,
            rect,
            view: state.view.clone(),
            focus: false,
            beat: state.input.beat,
            texture,
        }
    }

    /// Draw the rows.
    pub fn update(&self, renderer: &Renderer) {
        // Draw the background.
        renderer.rectangle(&self.rect, &BACKGROUND_COLOR);
        // Draw each row.
        self.positions
            .iter()
            .for_each(|p| renderer.texture_pixel(&self.texture, p, None));
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
            Self::set_row_texture(&mut self.texture, &mut self.sub_row, &mut self.row, self.width, focus, state, renderer);
        }
    }

    /// Write color data to the row buffer and use it to create a very thin texture.
    fn set_row_texture(texture: &mut Texture2D, sub_row: &mut [u8],
        row: &mut [u8],
        w: f32,
        focus: bool,
        state: &State,
        renderer: &Renderer,
    ) {
        let len = sub_row.len();
        let line_segment_width = super::viewable_notes::get_note_x(
            state.input.beat.get_u(),
            0.0,
            w,
            &[U64orF32::from(state.view.dt[0]), U64orF32::from(state.view.dt[1])],
        ).clamp(1.0, f32::MAX).ceil() as usize;
        let color: [u8; 4] = renderer
            .get_color(if focus {
                &ColorKey::Separator
            } else {
                &ColorKey::NoFocus
            })
            .into();
        let clear = [0u8; 4];
        // Copy the color into the sub-row.
        let mut draw_color = false;
        let lsw = line_segment_width * 4;
        for i in (0..sub_row.len()).step_by(4) {
            if i % lsw == 0 {
                draw_color = !draw_color;
            }
            sub_row[i..i + 4].copy_from_slice(if draw_color { &color } else { &clear });
        }
        let num_sub_rows = row.len() / sub_row.len();
        // Copy the sub-row into the row.
        for i in 0..num_sub_rows {
            let ir = i * len;
            row[ir..ir + len].copy_from_slice(&sub_row);
        }
        let image = Image { bytes: row.to_vec(), width: w as u16, height: num_sub_rows as u16};
        texture.update(&image);
    }
}
