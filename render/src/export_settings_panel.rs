use crate::panel::*;
use crate::Focus;
use audio::export::{ExportSetting, ExportType, MultiFileSuffix};
use audio::exporter::{Exporter, MP3_BIT_RATES};
use common::IndexedValues;
use hashbrown::HashMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use text::ValueMap;
use util::KV_PADDING;

struct SeparatorLines {
    framerate: Line,
    mp3_bit_rate: Line,
    ogg_quality: Line,
    title: Line,
}

/// Export settings panel.
pub(crate) struct ExportSettingsPanel {
    /// The panel position.
    position: [u32; 2],
    /// The panel width.
    width: u32,
    /// The title label for the panel.
    title: Label,
    /// The position and size of the title in grid units.
    title_rect: Rectangle,
    /// The framerate field.
    framerate: KeyListCorners,
    /// The MP3 bit rate field.
    mp3_bit_rate: KeyListCorners,
    /// The MP3/ogg quality field.
    quality: KeyListCorners,
    /// String values of multi-file suffixes.
    multi_file_suffixes: ValueMap<MultiFileSuffix>,
    /// Panel background sizes per export type.
    backgrounds: HashMap<ExportType, PanelBackground>,
    separator_lines: SeparatorLines,
}

impl ExportSettingsPanel {
    pub fn new(config: &Ini, renderer: &Renderer, exporter: &Exporter, text: &Text) -> Self {
        let (open_file_position, open_file_size) = get_open_file_rect(config);
        let position = [
            open_file_position[0],
            open_file_position[1] + open_file_size[1] + OPEN_FILE_PANEL_PROMPT_HEIGHT,
        ];
        let width: u32 = open_file_size[0];
        let title = text.get("TITLE_EXPORT_SETTINGS");
        let title_position = [position[0] + 2, position[1]];
        let title_width = title.chars().count() as u32;
        let title = Label::new(title_position, title, renderer);
        let title_rect = Rectangle::new(title_position, [title_width, 1]);
        let x = position[0] + 1;
        let y = position[1] + 1;
        let w = width - 2;
        let framerate = KeyListCorners::new(
            text.get("EXPORT_SETTINGS_PANEL_FRAMERATE"),
            [x, y],
            w,
            5,
            renderer,
        );
        let quality = KeyListCorners::new(
            text.get("EXPORT_SETTINGS_PANEL_QUALITY"),
            [x, y + 1],
            w,
            1,
            renderer,
        );
        let mp3_bit_rate = KeyListCorners::new(
            text.get("EXPORT_SETTINGS_PANEL_MP3_BIT_RATE"),
            [x, y + 2],
            w,
            6,
            renderer,
        );

        let separator_lines = SeparatorLines {
            framerate: Self::get_separator([x, framerate.y + 1], width, renderer),
            mp3_bit_rate: Self::get_separator([x, mp3_bit_rate.y + 1], width, renderer),
            ogg_quality: Self::get_separator([x, quality.y + 1], width, renderer),
            title: Self::get_separator([x, y + 1], width, renderer),
        };

        let multi_file_suffixes = ValueMap::new(
            [
                MultiFileSuffix::Channel,
                MultiFileSuffix::Preset,
                MultiFileSuffix::ChannelAndPreset,
            ],
            [
                "EXPORT_SETTINGS_PANEL_FILE_SUFFIX_CHANNEL",
                "EXPORT_SETTINGS_PANEL_FILE_SUFFIX_PRESET",
                "EXPORT_SETTINGS_PANEL_FILE_SUFFIX_CHANNEL_AND_PRESET",
            ],
            text,
        );

        // Calculate the background sizes per export type.
        let mut backgrounds = HashMap::new();
        backgrounds.insert(
            ExportType::Wav,
            PanelBackground::new(
                position,
                [width, exporter.wav_settings.index.get_length() as u32 + 3],
                renderer,
            ),
        );
        backgrounds.insert(
            ExportType::Mid,
            PanelBackground::new(
                position,
                [width, exporter.mid_settings.index.get_length() as u32 + 2],
                renderer,
            ),
        );
        backgrounds.insert(
            ExportType::MP3,
            PanelBackground::new(
                position,
                [width, exporter.mp3_settings.index.get_length() as u32 + 4],
                renderer,
            ),
        );
        backgrounds.insert(
            ExportType::Ogg,
            PanelBackground::new(
                position,
                [width, exporter.ogg_settings.index.get_length() as u32 + 4],
                renderer,
            ),
        );
        backgrounds.insert(
            ExportType::Flac,
            PanelBackground::new(
                position,
                [width, exporter.flac_settings.index.get_length() as u32 + 4],
                renderer,
            ),
        );

        Self {
            position,
            width,
            title,
            title_rect,
            framerate,
            mp3_bit_rate,
            quality,
            multi_file_suffixes,
            backgrounds,
            separator_lines,
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
        if export_type == ExportType::Flac {
            y += 1;
        }
        let (settings, values) = f(exporter).get_values();
        for (setting, value) in settings.iter().zip(values) {
            let setting_focus = [focus, value];
            match setting {
                ExportSetting::Framerate => {
                    renderer.key_list_corners(
                        &exporter.framerate.to_string(),
                        &self.framerate,
                        setting_focus,
                    );
                    // For .wav and .flac files, draw a separator here.
                    y = self.framerate.y + 1;
                    if export_type == ExportType::Wav || export_type == ExportType::Flac {
                        renderer.horizontal_line(&self.separator_lines.framerate, &line_color);
                        y += 1;
                    }
                }
                ExportSetting::Mp3BitRate => {
                    renderer.key_list_corners(
                        &((MP3_BIT_RATES[exporter.mp3_bit_rate.get()] as u16) as u32 * 1000)
                            .to_string(),
                        &self.mp3_bit_rate,
                        setting_focus,
                    );
                    renderer.horizontal_line(&self.separator_lines.mp3_bit_rate, &line_color);
                    y = self.mp3_bit_rate.y + 2;
                }
                ExportSetting::Mp3Quality => renderer.key_list_corners(
                    &exporter.mp3_quality.get().to_string(),
                    &self.quality,
                    setting_focus,
                ),
                ExportSetting::OggQuality => {
                    renderer.key_list_corners(
                        &exporter.ogg_quality.get().to_string(),
                        &self.quality,
                        setting_focus,
                    );
                    renderer.horizontal_line(&self.separator_lines.ogg_quality, &line_color);
                    y = self.quality.y + 2;
                }
                ExportSetting::Title => {
                    let key_input = KeyInput::new_from_padding(
                        text.get_ref("EXPORT_SETTINGS_PANEL_TITLE"),
                        &exporter.metadata.title,
                        [x, y],
                        self.width - 2,
                        KV_PADDING,
                        renderer,
                    );
                    renderer.key_input(
                        &exporter.metadata.title,
                        &key_input,
                        state.input.alphanumeric_input,
                        setting_focus,
                    );
                    y += 1;
                    // For .wav files, draw a separator here.
                    if export_type == ExportType::Wav {
                        renderer.horizontal_line(&self.separator_lines.title, &line_color);
                        y += 1;
                    }
                }
                ExportSetting::Artist => self.draw_optional_input(
                    text.get_ref("EXPORT_SETTINGS_PANEL_ARTIST"),
                    &exporter.metadata.artist,
                    (x, &mut y),
                    renderer,
                    state,
                    setting_focus,
                ),
                ExportSetting::Copyright => {
                    self.draw_boolean(
                        text.get("EXPORT_SETTINGS_PANEL_COPYRIGHT"),
                        exporter.copyright,
                        (x, &mut y),
                        renderer,
                        text,
                        setting_focus,
                    );
                }
                ExportSetting::Album => self.draw_optional_input(
                    text.get_ref("EXPORT_SETTINGS_PANEL_ALBUM"),
                    &exporter.metadata.album,
                    (x, &mut y),
                    renderer,
                    state,
                    setting_focus,
                ),
                ExportSetting::TrackNumber => {
                    let n = text.get("NONE");
                    let value_width = n.chars().count() as u32;
                    let value = match &exporter.metadata.track_number {
                        Some(value) => value.to_string(),
                        None => n,
                    };
                    let key_list = KeyListCorners::new(
                        text.get("EXPORT_SETTINGS_PANEL_TRACK_NUMBER"),
                        [x, y],
                        self.width - 2,
                        value_width,
                        renderer,
                    );
                    renderer.key_list_corners(&value, &key_list, setting_focus);
                    y += 1;
                }
                ExportSetting::Genre => self.draw_optional_input(
                    text.get_ref("EXPORT_SETTINGS_PANEL_GENRE"),
                    &exporter.metadata.genre,
                    (x, &mut y),
                    renderer,
                    state,
                    setting_focus,
                ),
                ExportSetting::Comment => {
                    self.draw_optional_input(
                        text.get_ref("EXPORT_SETTINGS_PANEL_COMMENT"),
                        &exporter.metadata.comment,
                        (x, &mut y),
                        renderer,
                        state,
                        setting_focus,
                    );
                    // This is always the last of the metadata. Draw a line.
                    let separator = Self::get_separator([x, y], self.width, renderer);
                    renderer.horizontal_line(&separator, &line_color);
                    y += 1;
                }
                ExportSetting::MultiFile => self.draw_boolean(
                    text.get("EXPORT_SETTINGS_PANEL_MULTI_FILE"),
                    exporter.multi_file,
                    (x, &mut y),
                    renderer,
                    text,
                    setting_focus,
                ),
                ExportSetting::MultiFileSuffix => {
                    let value = self
                        .multi_file_suffixes
                        .get(&exporter.multi_file_suffix.get());
                    let key_list = KeyListCorners::new(
                        text.get("EXPORT_SETTINGS_PANEL_MULTI_FILE_SUFFIX"),
                        [x, y],
                        self.width - 2,
                        self.multi_file_suffixes.max_length,
                        renderer,
                    );
                    renderer.key_list_corners(value, &key_list, setting_focus);
                    y += 1;
                }
            }
        }
    }

    fn get_separator(position: [u32; 2], width: u32, renderer: &Renderer) -> Line {
        let mut position = renderer.grid_to_pixel(position);
        // Apply an offset to the y value.
        position[1] += 0.5 * renderer.cell_size[1];
        let x1 = position[0] + (width - 2) as f32 * renderer.cell_size[0];
        Line::horizontal(position[0], x1, position[1])
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
            renderer,
        );
        renderer.key_input(value, &key_input, state.input.alphanumeric_input, focus);
        *position.1 += 1;
    }

    /// Draw a boolean field.
    fn draw_boolean(
        &self,
        key: String,
        value: bool,
        position: (u32, &mut u32),
        renderer: &Renderer,
        text: &Text,
        focus: Focus,
    ) {
        let boolean = BooleanCorners::new(
            key,
            [position.0, *position.1],
            self.width - 2,
            text,
            renderer,
        );
        renderer.boolean_corners(value, &boolean, focus);
        *position.1 += 1;
    }
}

impl Drawable for ExportSettingsPanel {
    fn update(&self, renderer: &Renderer, state: &State, conn: &Conn, text: &Text, _: &PathsState) {
        // Get the focus.
        let focus = state.panels[state.focus.get()] == PanelType::ExportSettings;
        // Draw the panel.
        let color: ColorKey = if focus {
            ColorKey::FocusDefault
        } else {
            ColorKey::NoFocus
        };
        let background = &self.backgrounds[&conn.exporter.export_type.get()];
        renderer.rectangle_pixel(&background.background, &color);
        renderer.rectangle_lines(&background.border, &color);
        renderer.rectangle(&self.title_rect, &ColorKey::Background);
        renderer.text(&self.title, &color);

        // Draw the fields.
        match &conn.exporter.export_type.get() {
            ExportType::Wav => self.update_settings(
                |e| &e.wav_settings,
                renderer,
                state,
                text,
                &conn.exporter,
                focus,
            ),
            ExportType::Mid => self.update_settings(
                |e| &e.mid_settings,
                renderer,
                state,
                text,
                &conn.exporter,
                focus,
            ),
            ExportType::MP3 => self.update_settings(
                |e| &e.mp3_settings,
                renderer,
                state,
                text,
                &conn.exporter,
                focus,
            ),
            ExportType::Ogg => self.update_settings(
                |e| &e.ogg_settings,
                renderer,
                state,
                text,
                &conn.exporter,
                focus,
            ),
            ExportType::Flac => self.update_settings(
                |e| &e.flac_settings,
                renderer,
                state,
                text,
                &conn.exporter,
                focus,
            ),
        }
    }
}
