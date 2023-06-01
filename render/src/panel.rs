use crate::{ColorKey, Renderer};

pub struct Panel {
    /// The title string for the panel.
    title: String,
    /// The position of the panel in grid units.
    pub position: [u32; 2],
    /// The size of the panel in grid units.
    pub size: [u32; 2],
    /// The position of the title in grid units.
    title_position: [u32; 2],
    /// The size of the title in grid units.
    title_size: [u32; 2],
}

impl Panel {
    fn new(title: String, position: [u32; 2], size: [u32; 2]) -> Self {
        let title_position = [position[0] + 2, position[1]];
        let title_size = [title.chars().count() as u32, 1];
        Self {
            title,
            position,
            size,
            title_position,
            title_size,
        }
    }

    /// Draw an empty panel. The color will be defined by the value of `focus`.
    pub fn draw(&self, focus: bool, renderer: &Renderer) {
        let color: ColorKey = if focus {
            ColorKey::FocusDefault
        } else {
            ColorKey::NoFocus
        };
        self.draw_ex(&color, renderer);
    }

    /// Draw an empty panel. The border and title text will be an explicitly defined color.
    pub fn draw_ex(&self, color: &ColorKey, renderer: &Renderer) {
        renderer.rectangle(self.position, self.size, &ColorKey::Background);
        renderer.border(self.position, self.size, color);
        renderer.rectangle(self.title_position, self.title_size, &ColorKey::Background);
        renderer.text(&self.title, self.title_position, color);
    }
}
