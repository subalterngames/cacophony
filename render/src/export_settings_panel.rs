use crate::panel::*;
use crate::Focus;
use audio::exporter::*;
use common::IndexedValues;
use serde::de::DeserializeOwned;
use serde::Serialize;
use util::KV_PADDING;

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
        let position = [
            open_file_position[0],
            open_file_position[1] + open_file_size[1],
        ];
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
        let quality = KeyList::new("EXPORT_SETTINGS_PANEL_QUALITY", [x, y + 1], w, 1);
        let mp3_bit_rate = KeyList::new("EXPORT_SETTINGS_PANEL_MP3_BIT_RATE", [x, y + 2], w, 3);
        Self {
            position,
            width,
            title,
            title_rect,
            framerate,
            mp3_bit_rate,
            quality,
        }
    }

    fn update_settings<F, const N: usize>(
        &self,
        f: F,
        renderer: &Renderer,
        state: &State,
        text: &Text,
        exporter: &Exporter,
        focus: bool,
    ) where
        F: Fn(&Exporter) -> &IndexedValues<ExportSetting, N>,
        [ExportSetting; N]: Serialize + DeserializeOwned,
    {
        // This is used to decide where to draw separators.
        let export_type = exporter.export_type.get();
        // Get the color of the separator line.
        let line_color = if focus {
            ColorKey::Separator
        } else {
            ColorKey::NoFocus
        };
        // Get the start positions.
        let x = self.position[0] + 1;
        let mut y = self.position[1] + 1;
        let (settings, values) = f(exporter).get_values();
        for (setting, value) in settings.iter().zip(values) {
            let setting_focus = [focus, value];
            match setting {
                ExportSetting::Framerate => {
                    renderer.key_list(
                        &exporter.framerate.to_string(),
                        &self.framerate,
                        setting_focus,
                    );
                    // For .wav files, draw a separator here.
                    if export_type == ExportType::Wav {
                        y = self.framerate.key.position[1] + 1;
                        self.draw_separator((x, &mut y), renderer, &line_color);
                    }
                }
                ExportSetting::Mp3BitRate => {
                    renderer.key_list(
                        &exporter.mp3_bit_rate.get().to_string(),
                        &self.mp3_bit_rate,
                        setting_focus,
                    );
                    y = self.mp3_bit_rate.key.position[1] + 1;
                    self.draw_separator((x, &mut y), renderer, &line_color);
                }
                ExportSetting::Mp3Quality => renderer.key_list(
                    &exporter.mp3_quality.get().to_string(),
                    &self.quality,
                    setting_focus,
                ),
                ExportSetting::OggQuality => {
                    renderer.key_list(
                        &exporter.ogg_quality.get().to_string(),
                        &self.quality,
                        setting_focus,
                    );
                    y = self.quality.key.position[1] + 1;
                    self.draw_separator((x, &mut y), renderer, &line_color);
                }
                ExportSetting::Title => {
                    let key_input = KeyInput::new_from_padding(
                        "EXPORT_SETTINGS_PANEL_TITLE",
                        &exporter.metadata.title,
                        [x, y],
                        self.width,
                        KV_PADDING,
                    );
                    renderer.key_input(
                        &exporter.metadata.title,
                        &key_input,
                        state.input.alphanumeric_input,
                        setting_focus,
                    );
                    y += 1;
                }
                ExportSetting::Artist => self.draw_optional_input(
                    "EXPORT_SETTINGS_PANEL_ARTIST",
                    &exporter.metadata.artist,
                    (x, &mut y),
                    renderer,
                    state,
                    setting_focus,
                ),
                ExportSetting::Copyright => {
                    self.draw_boolean(
                        "EXPORT_SETTINGS_PANEL_COPYRIGHT",
                        exporter.copyright,
                        (x, &mut y),
                        renderer,
                        text,
                        focus,
                    );
                    // For .mid files, draw a separator here.
                    if export_type == ExportType::Mid {
                        self.draw_separator((x, &mut y), renderer, &line_color);
                    }
                }
                ExportSetting::Album => self.draw_optional_input(
                    "EXPORT_SETTINGS_PANEL_ALBUM",
                    &exporter.metadata.album,
                    (x, &mut y),
                    renderer,
                    state,
                    setting_focus,
                ),
                ExportSetting::TrackNumber => {
                    let value = match &exporter.metadata.track_number {
                        Some(value) => value.to_string(),
                        None => String::new(),
                    };
                    let key_input = KeyInput::new_from_padding(
                        "EXPORT_SETTINGS_PANEL_TRACK_NUMBER",
                        &value,
                        [x, y],
                        self.width - 2,
                        KV_PADDING,
                    );
                    renderer.key_input(
                        &value,
                        &key_input,
                        state.input.alphanumeric_input,
                        setting_focus,
                    );
                    y += 1;
                }
                ExportSetting::Genre => self.draw_optional_input(
                    "EXPORT_SETTINGS_PANEL_GENRE",
                    &exporter.metadata.album,
                    (x, &mut y),
                    renderer,
                    state,
                    setting_focus,
                ),
                ExportSetting::Comment => {
                    self.draw_optional_input(
                        "EXPORT_SETTINGS_PANEL_COMMENT",
                        &exporter.metadata.album,
                        (x, &mut y),
                        renderer,
                        state,
                        setting_focus,
                    );
                    // This is always the last of the metadata. Draw a line.
                    self.draw_separator((x, &mut y), renderer, &line_color);
                }
                ExportSetting::MultiFile => self.draw_boolean(
                    "EXPORT_SETTINGS_PANEL_MULTI_FILE",
                    exporter.multi_file,
                    (x, &mut y),
                    renderer,
                    text,
                    focus,
                ),
                ExportSetting::MultiFileSuffix => {
                    let value_key = match &exporter.multi_file_suffix.get() {
                        MultiFile::Channel => "EXPORT_SETTINGS_PANEL_FILE_SUFFIX_CHANNEL",
                        MultiFile::Preset => "EXPORT_SETTINGS_PANEL_FILE_SUFFIX_PRESET",
                        MultiFile::ChannelAndPreset => {
                            "EXPORT_SETTINGS_PANEL_FILE_SUFFIX_CHANNEL_AND_PRESET"
                        }
                    };
                    let value = text.get(value_key);
                    let key_list = KeyList::new(
                        "EXPORT_SETTINGS_PANEL_MULTI_FILE_SUFFIX",
                        [x, y],
                        self.width - 2,
                        (self.width - 2) / 2,
                    );
                    renderer.key_list(&value, &key_list, setting_focus);
                    y += 1;
                }
            }
        }
    }

    /// Draw a separator line after a section.
    fn draw_separator(&self, position: (u32, &mut u32), renderer: &Renderer, color: &ColorKey) {
        renderer.horizontal_line(
            position.0,
            position.0 + self.width - 2,
            [0.0, 0.0],
            *position.1,
            0.0,
            color,
        );
        *position.1 += 1;
    }

    /// Draw an input with optional text.
    fn draw_optional_input(
        &self,
        key: &str,
        value: &Option<String>,
        position: (u32, &mut u32),
        renderer: &Renderer,
        state: &State,
        focus: Focus,
    ) {
        let value = match value {
            Some(value) => value,
            None => "",
        };
        let key_input = KeyInput::new_from_padding(
            key,
            value,
            [position.0, *position.1],
            self.width - 2,
            KV_PADDING,
        );
        renderer.key_input(value, &key_input, state.input.alphanumeric_input, focus);
        *position.1 += 1;
    }

    fn draw_boolean(
        &self,
        key: &str,
        value: bool,
        position: (u32, &mut u32),
        renderer: &Renderer,
        text: &Text,
        focus: bool,
    ) {
        let boolean = Boolean::new(key, [position.0, *position.1], text);
        renderer.boolean(value, &boolean, focus, text);
        *position.1 += 1;
    }
}

impl Drawable for ExportSettingsPanel {
    fn update(
        &self,
        renderer: &Renderer,
        state: &State,
        _: &Conn,
        _: &Input,
        text: &Text,
        _: &PathsState,
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
        } else if e == ExportType::Wav {
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
            ExportType::Wav => {
                self.update_settings(|e| &e.wav_settings, renderer, state, text, exporter, focus)
            }
            ExportType::Mid => {
                self.update_settings(|e| &e.mid_settings, renderer, state, text, exporter, focus)
            }
            ExportType::MP3 => {
                self.update_settings(|e| &e.mp3_settings, renderer, state, text, exporter, focus)
            }
            ExportType::Ogg => {
                self.update_settings(|e| &e.ogg_settings, renderer, state, text, exporter, focus)
            }
        }
    }
}
