use crate::get_page;
use crate::panel::*;
use text::truncate;

/// The open-file dialogue box.
pub struct OpenFilePanel {
    /// The panel.
    panel: Panel,
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

        Self { panel }
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
        self.panel.draw(true, renderer);
        // Draw the working directory.
        let mut x = self.panel.position[0] + 1;
        let mut y = self.panel.position[1] + 1;
        let mut length = (self.panel.size[0] - 2) as usize;

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
        let height: u32 = self.panel.size[1] - 2;
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
                renderer.rectangle(position, [width, 1], &bg_color);
            }
            // Draw the text.
            let s = truncate(
                &path.path.to_str().unwrap().replace('\\', "/"),
                length,
                true,
            );
            let p = Label { position, text: s };
            renderer.text(&p, &text_color);
        }

        // Possibly show the input dialogue.
        if let Some(filename) = &open_file.filename {
            let position = [
                self.panel.position[0],
                self.panel.position[1] + self.panel.size[1],
            ];
            let size = [self.panel.size[0], 3];
            let mut extension = open_file.extensions[0].clone();
            extension.insert(0, '.');
            let extension_length = extension.chars().count();
            let prompt_width = size[0] - extension_length as u32;
            let prompt_position = [position[0] + prompt_width - 1, position[1]];
            let prompt_size = [extension_length as u32, size[1]];
            // The background of the panel.
            renderer.rectangle(position, size, &ColorKey::Background);
            // The background of the prompt.
            renderer.rectangle(prompt_position, prompt_size, &ColorKey::TextFieldBG);
            // Draw the border.
            renderer.border(position, size, &ColorKey::FocusDefault);
            // Truncate the filename.
            let s = truncate(filename, length + extension_length, true);
            // Render the filename.
            let filename = Label {
                position: [position[0] + 1, position[1] + 1],
                text: s,
            };
            renderer.text(&filename, &ColorKey::TextInput);
            // Render the prompt.
            let prompt = Label {
                position: prompt_position,
                text: extension,
            };
            renderer.text(&prompt, &ColorKey::Key);
        }
    }
}
