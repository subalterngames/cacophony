use crate::Renderer;

/// A horizontal or vertical line.
pub(crate) struct Line {
    pub a0: f32,
    pub a1: f32,
    pub b: f32,
}

impl Line {
    pub(crate) fn vertical(x: f32, y0: f32, y1: f32) -> Self {
        Self {
            a0: y0,
            a1: y1,
            b: x,
        }
    }

    pub(crate) fn horizontal(x0: f32, x1: f32, y: f32) -> Self {
        Self {
            a0: x0,
            a1: x1,
            b: y,
        }
    }

    pub(crate) fn vertical_line_separator(position: [u32; 2], renderer: &Renderer) -> Self {
        let x = position[0] as f32 * renderer.cell_size[0] + 0.5 * renderer.cell_size[0];
        let y0 = position[1] as f32 * renderer.cell_size[1] - 0.6 * renderer.cell_size[1];
        let y1 = (position[1] + 1) as f32 * renderer.cell_size[1] + 0.4 * renderer.cell_size[1];
        Self::vertical(x, y0, y1)
    }
}
