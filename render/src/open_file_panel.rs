use crate::get_page;
use crate::panel::*;
use crate::Popup;
use common::open_file::*;
use text::truncate;

const EXTENSION_WIDTH: u32 = 4;

/// The open-file dialogue box.
pub(crate) struct OpenFilePanel {
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
    /// The popup handler.
    pub popup: Popup,
}

impl OpenFilePanel {
    pub fn new(config: &Ini, text: &Text) -> Self {
        let (position, size) = get_open_file_rect(config);
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
                prompt.position[1] + 1,
            ],
            EXTENSION_WIDTH as usize,
        );

        let input = Width::new(
            [prompt.position[0] + 1, prompt.position[1] + 1],
            (prompt.size[0] - EXTENSION_WIDTH - 3) as usize,
        );
        let input_rect = Rectangle::new(input.position, [input.width_u32, 1]);

        let popup = Popup::new(PanelType::OpenFile);

        Self {
            panel,
            prompt,
            extension,
            input,
            input_rect,
            popup,
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
        paths_state: &PathsState,
        exporter: &Exporter,
    ) {
        self.popup.update(renderer);
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
                &paths_state
                    .get_directory()
                    .to_str()
                    .unwrap()
                    .replace('\\', "/"),
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
        let elements: Vec<u32> = vec![1; paths_state.children.children.len()];
        let page = get_page(&paths_state.children.selected, &elements, height);

        // Show the elements.
        for index in page {
            let path = &paths_state.children.children[index];
            // Get the color of the text. Flip the fg/bg colors for the selected element.
            let c = if path.is_file {
                ColorKey::Value
            } else {
                ColorKey::FocusDefault
            };
            let (text_color, bg_color) = match paths_state.children.selected {
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
        if let Some(filename) = &paths_state.get_filename() {
            // Draw the background of the prompt.
            renderer.rectangle(&self.prompt, &ColorKey::Background);
            renderer.border(&self.prompt, &ColorKey::FocusDefault);

            // Draw the extension.
            let mut extension = String::from(".");
            let e = exporter.export_type.get();
            let ext = match paths_state.open_file_type {
                OpenFileType::ReadSave | OpenFileType::WriteSave => ".cac",
                OpenFileType::SoundFont => ".sf2",
                OpenFileType::Export => e.get_extension(true),
            };
            extension.push_str(ext);
            renderer.text(&self.extension.to_label(&extension), &ColorKey::Arrow);

            // Draw the input background.
            renderer.rectangle(&self.input_rect, &ColorKey::TextFieldBG);

            // Draw the input text.
            renderer.text(&self.input.to_label(filename), &ColorKey::Key);
        }
    }
}
