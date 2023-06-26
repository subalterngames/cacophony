use serde::Serialize;
use serde::de::DeserializeOwned;
use audio::exporter::*;
use common::IndexedValues;
use crate::panel::*;

/// Export settings panel.
pub(crate) struct ExportSettingsPanel {
    /// The position of the panel.
    position: [u32; 2],
    /// The width of the panel.
    width: u32,
    /// The title label for the panel.
    title: Label,
    /// The position and size of the title in grid units.
    title_rect: Rectangle,
    /// The framerate field.
    framerate: KeyList,
    /// The MP3 bit rate field.
    mp3_bit_rate: KeyList,
    /// The MP3/ogg quality field.
    quality: KeyList,
}

impl ExportSettingsPanel {
    pub fn new(config: &Ini, text: &Text) -> Self {
        let (open_file_position, open_file_size) = get_open_file_rect(config);
        let position = [open_file_position[0], open_file_position[1] + open_file_size[1]];
        let width: u32 = open_file_size[0];
        let title = text.get("TITLE_EXPORT_SETTINGS");
        let title_position = [position[0] + 2, position[1]];
        let title_width = title.chars().count() as u32;
        let title = Label {
            position: title_position,
            text: title,
        };
        let title_rect = Rectangle::new(title_position, [title_width, 1]);
        let x = position[0] + 1;
        let y = position[1] + 1;
        let w = width - 2;
        let framerate = KeyList::new("EXPORT_SETTINGS_PANEL_FRAMERATE", [x, y], w, 5);
        let mp3_bit_rate = KeyList::new("EXPORT_SETTINGS_PANEL_MP3_BIT_RATE", [x, y + 1], w, 3);
        let quality = KeyList::new("EXPORT_SETTINGS_PANEL_QUALITY", [x, y + 2], w, 1);
        Self { position, width, title, title_rect, framerate, mp3_bit_rate, quality }
    }

    fn update_settings<F, const N: usize>(&self, f: F, renderer: &Renderer, exporter: &Exporter, focus: bool) 
    where
    F: Fn(&Exporter) -> &IndexedValues<ExportSetting, N>,
    [ExportSetting; N]: Serialize + DeserializeOwned {
        let x = self.position[0] + 1;
        let mut y = self.position[1] + 1;
        let mut i = 0;
        let (settings, values) = f(exporter).get_values();
        // Data settings.
        for (setting, value) in settings.iter().zip(values) {
            let setting_focus = [focus, value];
            match setting {
                ExportSetting::Framerate => renderer.key_list(&exporter.framerate.to_string(), &self.framerate, setting_focus),
                ExportSetting::Mp3BitRate => renderer.key_list(&exporter.mp3_bit_rate.get().to_string(), &self.mp3_bit_rate, setting_focus),
                ExportSetting::Mp3Quality => renderer.key_list(&exporter.mp3_quality.get().to_string(), &self.quality, setting_focus),
                ExportSetting::OggQuality => renderer.key_list(&exporter.ogg_quality.get().to_string(), &self.quality, setting_focus),
                _ => ()
            }
            y += 1;
            i += 1;
        }
        let line_color = if focus { ColorKey::Separator } else { ColorKey::NoFocus };
        // There are more settings. Add a divider.
        if i < N {
            renderer.horizontal_line(x, x + self.width - 2, [0.0, 0.0], y, 0.0, &line_color);
            y += 1;
            // Metadata settings.
            for (setting, value) in settings.iter().zip(values) {
                let setting_focus = [focus, value];
                match setting {
                }
                y += 1;
                i += 1;
            }
        }
    }
}

impl Drawable for ExportSettingsPanel {
    fn update(
            &self,
            renderer: &Renderer,
            state: &State,
            conn: &Conn,
            input: &Input,
            text: &Text,
            paths_state: &PathsState,
            exporter: &Exporter,
        ) {
        // Get the focus.
        let focus = state.panels[state.focus.get()] == PanelType::ExportSettings;

        // Get the height of the panel.
        let mut h = exporter.export_type.index.get_length() as u32 + 2;
        let e = exporter.export_type.get();

        // Add spaces for divider lines.
        if e == ExportType::MP3 || e == ExportType::Ogg {
            h += 2;
        }
        else if e == ExportType::Wav {
            h += 1;
        }

        // Draw the panel.
        let color: ColorKey = if focus {
            ColorKey::FocusDefault
        } else {
            ColorKey::NoFocus
        };
        let rect = Rectangle::new(self.position, [self.width, h]);
        renderer.rectangle(&rect, &ColorKey::Background);
        renderer.border(&rect, &color);
        renderer.rectangle(&self.title_rect, &ColorKey::Background);
        renderer.text(&self.title, &color);

        // Draw the fields.
        match &exporter.export_type.get() {
            ExportType::Wav => self.update_settings(|e| &e.wav_settings, renderer, exporter),
            ExportType::Mid => self.update_settings(|e| &e.mid_settings, renderer, exporter),
            ExportType::MP3 => self.update_settings(|e| &e.mp3_settings, renderer, exporter),
            ExportType::Ogg => self.update_settings(|e| &e.ogg_settings, renderer, exporter),
        }
    }
}