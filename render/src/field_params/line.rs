use crate::Renderer;

/// A horizontal or vertical line.
pub(crate) struct Line {
    pub a0: f32,
    pub a1: f32,
    pub b: f32,
    pub vertical: bool
}

impl Line {
    pub(crate) fn vertical(x: f32, y0: f32, y1: f32) -> Self {
        Self { a0: y0, a1: y1, b: x, vertical: true}
    }

    pub(crate) fn horizontal(x0: f32, x1: f32, y: f32) -> Self {
        Self { a0: x0, a1: x1, b: y, vertical: false}
    }
}