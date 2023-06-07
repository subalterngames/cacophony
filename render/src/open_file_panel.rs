use crate::get_page;
use crate::panel::*;
use text::truncate;

const EXTENSION_WIDTH: u32 = 4;

/// The open-file dialogue box.
pub struct OpenFilePanel {
    /// The panel.
    panel: Panel,
    /// The background of the filename prompt.
    prompt: Rectangle,
    /// The filename extension.
    extension: Width,
    /// The filename input.
    input: Width,
    /// The filename input background rectangle.
    input_rect: Rectangle,
}

impl OpenFilePanel {
    pub fn new(config: &Ini, text: &Text) -> Self {
        let window_grid_size = get_window_grid_size(config);
        let size = [window_grid_size[0] / 2, window_grid_size[1] / 2];
        let position = [
            window_grid_size[0] / 2 - size[0] / 2,
            window_grid_size[1] / 2 - size[1] / 2,
        ];
        let panel = Panel::new(PanelType::OpenFile, position, size, text);

        let prompt = Rectangle::new(
            [
                panel.rect.position[0],
                panel.rect.position[1] + panel.rect.size[1],
            ],
            [panel.rect.size[0], 3],
        );
        let extension = Width::new(
            [
                prompt.position[0] + prompt.size[0] - EXTENSION_WIDTH - 1,
                prompt.position[1],
            ],
            EXTENSION_WIDTH as usize,
        );

        let input = Width::new(
            [prompt.position[0] + 1, prompt.position[1]],
            (prompt.size[0] - EXTENSION_WIDTH - 2) as usize,
        );
        let input_rect = Rectangle::new(input.position, [input.width_u32, 1]);

        Self {
            panel,
            prompt,
            extension,
            input,
            input_rect,
        }
    }
}

impl Drawable for OpenFilePanel {
    fn update(
        &self,
        renderer: &Renderer,
        _: &State,
        _: &Conn,
        _: &Input,
        _: &Text,
        open_file: &OpenFile,
    ) {
        // Draw the panel background.
        self.panel.update(true, renderer);
        // Draw the working directory.
        let mut x = self.panel.rect.position[0] + 1;
        let mut y = self.panel.rect.position[1] + 1;
        let mut length = (self.panel.rect.size[0] - 2) as usize;

        // Show the current directory.
        let cwd = Label {
            position: [x, y],
            text: truncate(
                &open_file.directory.to_str().unwrap().replace('\\', "/"),
                length,
                true,
            ),
        };
        renderer.text(&cwd, &ColorKey::Key);

        // Prepare to show the children.
        x += 1;
        let height: u32 = self.panel.rect.size[1] - 3;
        y += 1;
        length -= 1;
        let width = length as u32;

        // Get a page of elements.
        let elements: Vec<u32> = vec![1; open_file.paths.len()];
        let page = get_page(&open_file.selected, &elements, height);

        // Show the elements.
        for index in page {
            let path = &open_file.paths[index];
            // Get the color of the text. Flip the fg/bg colors for the selected element.
            let c = if path.is_file {
                ColorKey::Value
            } else {
                ColorKey::FocusDefault
            };
            let (text_color, bg_color) = match open_file.selected {
                Some(selected) => match selected == index {
                    true => (ColorKey::Background, Some(c)),
                    false => (c, None),
                },
                None => (c, None),
            };
            let position = [x, y];
            // Draw the background.
            if let Some(bg_color) = bg_color {
                renderer.rectangle(&Rectangle::new(position, [width, 1]), &bg_color);
            }
            // Draw the text.
            let s = truncate(
                &path.path.to_str().unwrap().replace('\\', "/"),
                length,
                true,
            );
            let p = Label { position, text: s };
            renderer.text(&p, &text_color);
            y += 1;
        }

        // Possibly show the input dialogue.
        if let Some(filename) = &open_file.filename {
            // Draw the background of the prompt.
            renderer.rectangle(&self.prompt, &ColorKey::Background);
            renderer.border(&self.prompt, &ColorKey::FocusDefault);

            // Draw the extension.
            renderer.text(
                &self.extension.to_label(&open_file.extensions[0]),
                &ColorKey::Key,
            );

            // Draw the input background.
            renderer.rectangle(&self.input_rect, &ColorKey::TextFieldBG);

            // Draw the input text.
            renderer.text(&self.input.to_label(filename), &ColorKey::TextInput);
        }
    }
}
