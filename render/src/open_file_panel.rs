use crate::panel::*;
use crate::{Page, PagePosition, Popup};
use common::open_file::*;
use hashbrown::HashMap;
use text::truncate;

const EXTENSION_WIDTH: u32 = 4;

/// The open-file dialogue box.
pub(crate) struct OpenFilePanel {
    /// The panel.
    panel: Panel,
    /// The titles for each open-file type.
    titles: HashMap<OpenFileType, LabelRectangle>,
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
    /// Labels for scrolling through pages.
    scroll_labels: HashMap<PagePosition, Label>,
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
            [panel.rect.size[0], OPEN_FILE_PANEL_PROMPT_HEIGHT],
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

        // Define the titles.
        let mut titles = HashMap::new();
        titles.insert(
            OpenFileType::SoundFont,
            LabelRectangle::new(
                panel.title.label.position,
                text.get("OPEN_FILE_PANEL_TITLE_SOUNDFONT"),
            ),
        );
        titles.insert(
            OpenFileType::ReadSave,
            LabelRectangle::new(
                panel.title.label.position,
                text.get("OPEN_FILE_PANEL_TITLE_READ_SAVE"),
            ),
        );
        titles.insert(
            OpenFileType::WriteSave,
            LabelRectangle::new(
                panel.title.label.position,
                text.get("OPEN_FILE_PANEL_TITLE_WRITE_SAVE"),
            ),
        );
        titles.insert(
            OpenFileType::Export,
            LabelRectangle::new(
                panel.title.label.position,
                text.get("OPEN_FILE_PANEL_TITLE_EXPORT"),
            ),
        );

        // Get the scroll labels.
        let mut scroll_labels = HashMap::new();
        let panel_x = panel.rect.position[0];
        let panel_w = panel.rect.size[0];
        let label_y = panel.rect.position[1] + panel.rect.size[1] - 2;
        scroll_labels.insert(
            PagePosition::First,
            Self::get_scroll_label("OPEN_FILE_PANEL_DOWN", text, panel_x, panel_w, label_y),
        );
        scroll_labels.insert(
            PagePosition::Mid,
            Self::get_scroll_label("OPEN_FILE_PANEL_UP_DOWN", text, panel_x, panel_w, label_y),
        );
        scroll_labels.insert(
            PagePosition::Last,
            Self::get_scroll_label("OPEN_FILE_PANEL_UP", text, panel_x, panel_w, label_y),
        );

        Self {
            panel,
            prompt,
            extension,
            input,
            input_rect,
            popup,
            titles,
            scroll_labels,
        }
    }

    /// Returns a label for scrolling, e.g. "MORE v".
    ///
    /// - `key` The lookup key.
    /// - `text` The text.
    /// - `panel_x` The x coordinate of the panel.
    /// - `panel_w` The width of the panel.
    /// - `label_y` the y coordinate of the label (the x coordinate varies).
    fn get_scroll_label(key: &str, text: &Text, panel_x: u32, panel_w: u32, label_y: u32) -> Label {
        let string = text.get(key);
        let x = panel_x + panel_w - (string.chars().count() as u32 + 2);
        Label::new([x, label_y], string)
    }
}

impl Drawable for OpenFilePanel {
    fn update(
        &self,
        renderer: &Renderer,
        state: &State,
        _: &Conn,
        _: &Text,
        paths_state: &PathsState,
        exporter: &SharedExporter,
    ) {
        let focus = self.panel.has_focus(state);
        self.popup.update(renderer);
        let focus_color = if focus {
            ColorKey::FocusDefault
        } else {
            ColorKey::NoFocus
        };
        // Draw the panel background.
        renderer.rectangle(&self.panel.rect, &ColorKey::Background);
        renderer.border(&self.panel.rect, &focus_color);
        // Draw the title.
        let title = &self.titles[&paths_state.open_file_type];
        renderer.rectangle(&title.rect, &ColorKey::Background);
        renderer.text(&title.label, &focus_color);
        // Draw the working directory.
        let mut x = self.panel.rect.position[0] + 1;
        let mut y = self.panel.rect.position[1] + 1;
        let mut length = (self.panel.rect.size[0] - 2) as usize;

        // Show the current directory.
        let cwd = Label {
            position: [x, y],
            text: truncate(
                &format!("{}/", paths_state.get_directory().stem),
                length,
                true,
            ),
        };
        renderer.text(&cwd, &Renderer::get_key_color(focus));

        // Prepare to show the children.
        x += 1;
        let height: u32 = self.panel.rect.size[1] - 4;
        y += 1;
        length -= 1;
        let width = length as u32;

        // Get a page of elements.
        let elements: Vec<u32> = vec![1; paths_state.children.children.len()];
        let page = Page::new(&paths_state.children.selected, &elements, height);

        // Show the elements.
        for index in page.visible {
            let path = &paths_state.children.children[index];
            // Get the color of the text. Flip the fg/bg colors for the selected element.
            let c = if focus {
                if path.is_file {
                    ColorKey::Value
                } else {
                    ColorKey::FocusDefault
                }
            } else {
                ColorKey::NoFocus
            };
            let (text_color, bg_color) = if focus {
                match paths_state.children.selected {
                    Some(selected) => match selected == index {
                        true => (ColorKey::Background, Some(c)),
                        false => (c, None),
                    },
                    None => (c, None),
                }
            } else {
                (ColorKey::NoFocus, None)
            };
            let position = [x, y];
            // Draw the background.
            if let Some(bg_color) = bg_color {
                renderer.rectangle(&Rectangle::new(position, [width, 1]), &bg_color);
            }
            // Draw the text.
            let s = if path.path.parent().is_some() {
                truncate(&path.stem, length, true)
            } else {
                path.path.to_str().unwrap().to_string()
            };
            let p = Label { position, text: s };
            renderer.text(&p, &text_color);
            y += 1;
        }

        // Possibly show the input dialogue.
        if let Some(filename) = &paths_state.get_filename() {
            // Draw the background of the prompt.
            renderer.rectangle(&self.prompt, &ColorKey::Background);
            renderer.border(
                &self.prompt,
                &if focus {
                    ColorKey::FocusDefault
                } else {
                    ColorKey::NoFocus
                },
            );

            // Draw the extension.
            let mut extension = String::from(".");
            let ex = exporter.lock();
            let e = ex.export_type.get();
            let ext = match paths_state.open_file_type {
                OpenFileType::ReadSave | OpenFileType::WriteSave => ".cac",
                OpenFileType::SoundFont => ".sf2",
                OpenFileType::Export => e.get_extension(true),
            };
            extension.push_str(ext);
            renderer.text(
                &self.extension.to_label(&extension),
                &if focus {
                    ColorKey::Arrow
                } else {
                    ColorKey::NoFocus
                },
            );

            // Draw the input background.
            if focus {
                renderer.rectangle(&self.input_rect, &ColorKey::TextFieldBG);
            }

            // Draw the input text.
            renderer.text(
                &self.input.to_label(filename),
                &if focus {
                    ColorKey::Key
                } else {
                    ColorKey::NoFocus
                },
            );
        }
        // Possible draw a scroll indicator.
        if page.position != PagePosition::Only {
            renderer.text(&self.scroll_labels[&page.position], &ColorKey::Value);
        }
    }
}
