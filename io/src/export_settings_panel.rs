use crate::panel::*;
use crate::{edit_optional_string, edit_string};
use audio::exporter::*;
use audio::{Command, Conn};
use common::{IndexedValues, U64orF32};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

const FRAMERATES: [u64; 3] = [22050, 44100, 48000];

/// Enum values for export settings.
#[derive(Eq, PartialEq, Copy, Clone, Default, Deserialize, Serialize)]
enum ExportSetting {
    #[default]
    Framerate,
    Title,
    Artist,
    Copyright,
    Album,
    TrackNumber,
    Genre,
    Comment,
    Mp3BitRate,
    Mp3Quality,
    OggQuality,
}

pub struct ExportSettingsPanel {
    mid_settings: IndexedValues<ExportSetting, 3>,
    wav_settings: IndexedValues<ExportSetting, 2>,
    mp3_settings: IndexedValues<ExportSetting, 10>,
    ogg_settings: IndexedValues<ExportSetting, 9>,
}

impl Default for ExportSettingsPanel {
    fn default() -> Self {
        let mid_settings = IndexedValues::new(
            0,
            [
                ExportSetting::Title,
                ExportSetting::Artist,
                ExportSetting::Copyright,
            ],
        );
        let wav_settings = IndexedValues::new(0, [ExportSetting::Framerate, ExportSetting::Title]);
        let mp3_settings = IndexedValues::new(
            0,
            [
                ExportSetting::Framerate,
                ExportSetting::Mp3BitRate,
                ExportSetting::Mp3Quality,
                ExportSetting::Title,
                ExportSetting::Artist,
                ExportSetting::Copyright,
                ExportSetting::Album,
                ExportSetting::TrackNumber,
                ExportSetting::Genre,
                ExportSetting::Comment,
            ],
        );
        let ogg_settings = IndexedValues::new(
            0,
            [
                ExportSetting::Framerate,
                ExportSetting::OggQuality,
                ExportSetting::Title,
                ExportSetting::Artist,
                ExportSetting::Copyright,
                ExportSetting::Album,
                ExportSetting::TrackNumber,
                ExportSetting::Genre,
                ExportSetting::Comment,
            ],
        );
        Self {
            mid_settings,
            wav_settings,
            mp3_settings,
            ogg_settings,
        }
    }
}

impl ExportSettingsPanel {
    fn set_exporter(c0: Vec<Command>, conn: &mut Conn, exporter: &Exporter) -> Option<Snapshot> {
        let c1 = vec![Command::SetExporter {
            exporter: Box::new(exporter.clone()),
        }];
        let snapshot = Some(Snapshot::from_commands(c0, &c1));
        conn.send(c1);
        snapshot
    }

    fn get_status_ab123_tts(
        if_true: &str,
        if_false: &str,
        value: &Option<String>,
        state: &State,
        input: &Input,
        text: &Text,
    ) -> String {
        let n = text.get("NONE");
        let value = value.as_ref().unwrap_or(&n);
        if state.input.alphanumeric_input {
            text.get_with_values(if_true, &[value])
        } else {
            get_tooltip_with_values(
                if_false,
                &[InputEvent::ToggleAlphanumericInput],
                &[value],
                input,
                text,
            )
        }
    }

    fn get_input_abc123_tts(
        if_true: &str,
        if_false: &str,
        state: &State,
        input: &Input,
        text: &Text,
    ) -> String {
        if state.input.alphanumeric_input {
            get_tooltip(if_true, &[InputEvent::ToggleAlphanumericInput], input, text)
        } else {
            let mut s = get_tooltip(
                if_false,
                &[InputEvent::ToggleAlphanumericInput],
                input,
                text,
            );
            s.push(' ');
            s.push_str(&get_tooltip(
                "EXPORT_SETTINGS_PANEL_INPUT_TTS_SCROLL",
                &[
                    InputEvent::PreviousExportSetting,
                    InputEvent::NextExportSetting,
                ],
                input,
                text,
            ));
            s
        }
    }

    fn get_input_lr_tts(key: &str, input: &Input, text: &Text) -> String {
        get_tooltip(
            key,
            &[
                InputEvent::PreviousExportSettingValue,
                InputEvent::NextExportSettingValue,
            ],
            input,
            text,
        )
    }

    fn set_framerate(conn: &mut Conn, exporter: &mut Exporter, up: bool) -> Option<Snapshot> {
        let c0 = vec![Command::SetExporter {
            exporter: Box::new(exporter.clone()),
        }];
        let i = FRAMERATES
            .iter()
            .position(|f| *f == exporter.framerate.get_u())
            .unwrap();
        let mut index = Index::new(i, FRAMERATES.len());
        index.increment(up);
        exporter.framerate = U64orF32::from(FRAMERATES[index.get()]);
        Self::set_exporter(c0, conn, exporter)
    }

    fn set_track_number(conn: &mut Conn, exporter: &mut Exporter, up: bool) -> Option<Snapshot> {
        let c0 = vec![Command::SetExporter {
            exporter: Box::new(exporter.clone()),
        }];
        exporter.metadata.track_number = if up {
            match &exporter.metadata.track_number {
                Some(n) => Some(n + 1),
                None => Some(0),
            }
        } else {
            match &exporter.metadata.track_number {
                Some(n) => n.checked_sub(1),
                None => None,
            }
        };
        Self::set_exporter(c0, conn, exporter)
    }

    fn set_index<F>(
        mut f: F,
        conn: &mut Conn,
        input: &Input,
        exporter: &mut Exporter,
    ) -> Option<Snapshot>
    where
        F: FnMut(&mut Exporter) -> &mut Index,
    {
        if input.happened(&InputEvent::PreviousExportSettingValue) {
            let c0 = vec![Command::SetExporter {
                exporter: Box::new(exporter.clone()),
            }];
            f(exporter).increment(false);
            Self::set_exporter(c0, conn, exporter)
        } else if input.happened(&InputEvent::NextExportSettingValue) {
            let c0 = vec![Command::SetExporter {
                exporter: Box::new(exporter.clone()),
            }];
            f(exporter).increment(true);
            Self::set_exporter(c0, conn, exporter)
        } else {
            None
        }
    }

    fn update_settings<const N: usize>(
        settings: &mut IndexedValues<ExportSetting, N>,
        state: &mut State,
        conn: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &Text,
        exporter: &mut Exporter,
    ) -> Option<Snapshot>
    where
        [ExportSetting; N]: Serialize + DeserializeOwned,
    {
        // Status TTS.
        if input.happened(&InputEvent::StatusTTS) {
            let s = match &settings.get() {
                ExportSetting::Framerate => text.get("EXPORT_SETTINGS_PANEL_STATUS_TTS_FRAMERATE"),
                ExportSetting::Title => Self::get_status_ab123_tts(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_TITLE_ABC123",
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_TITLE_NO_ABC123",
                    &Some(exporter.metadata.title.clone()),
                    state,
                    input,
                    text,
                ),
                ExportSetting::Artist => Self::get_status_ab123_tts(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_ARTIST",
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_ARTIST_NO_ABC123",
                    &exporter.metadata.artist,
                    state,
                    input,
                    text,
                ),
                ExportSetting::Copyright => get_tooltip(
                    if exporter.copyright {
                        "EXPORT_SETTINGS_PANEL_STATUS_TTS_COPYRIGHT_ENABLED"
                    } else {
                        "EXPORT_SETTINGS_PANEL_STATUS_TTS_COPYRIGHT_DISABLED"
                    },
                    &[InputEvent::ToggleExportSettingBoolean],
                    input,
                    text,
                ),
                ExportSetting::Album => Self::get_status_ab123_tts(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_ALBUM_ABC123",
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_ALBUM_NO_ABC123",
                    &exporter.metadata.album,
                    state,
                    input,
                    text,
                ),
                ExportSetting::Genre => Self::get_status_ab123_tts(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_GENRE_ABC123",
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_GENRE_NO_ABC123",
                    &exporter.metadata.genre,
                    state,
                    input,
                    text,
                ),
                ExportSetting::Comment => Self::get_status_ab123_tts(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_COMMENT_ABC123",
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_COMMENT_NO_ABC123",
                    &exporter.metadata.comment,
                    state,
                    input,
                    text,
                ),
                ExportSetting::Mp3BitRate => text.get_with_values(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_BIT_RATE",
                    &[
                        &((MP3_BIT_RATES[exporter.bit_rate.get()] as u16) as u32 * 1000)
                            .to_string(),
                    ],
                ),
                ExportSetting::Mp3Quality => text.get_with_values(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_QUALITY",
                    &[&exporter.mp3_quality.get().to_string()],
                ),
                ExportSetting::OggQuality => text.get_with_values(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_QUALITY",
                    &[&exporter.ogg_quality.get().to_string()],
                ),
                ExportSetting::TrackNumber => text.get_with_values(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_TRACK_NUMBER",
                    &[&match exporter.metadata.track_number {
                        Some(track_number) => track_number.to_string(),
                        None => text.get("NONE"),
                    }],
                ),
            };
            tts.say(&s);
            None
        }
        // Input TTS.
        else if input.happened(&InputEvent::InputTTS) {
            let s = match &settings.get() {
                ExportSetting::Framerate => {
                    Self::get_input_lr_tts("EXPORT_SETTINGS_PANEL_INPUT_TTS_FRAMERATE", input, text)
                }
                ExportSetting::Title => Self::get_input_abc123_tts(
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_TITLE_ABC123",
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_TITLE_NO_ABC123",
                    state,
                    input,
                    text,
                ),
                ExportSetting::Artist => Self::get_input_abc123_tts(
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_ARTIST_ABC123",
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_ARTIST_NO_ABC123",
                    state,
                    input,
                    text,
                ),
                ExportSetting::Copyright => get_tooltip(
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_COPYRIGHT",
                    &[InputEvent::ToggleExportSettingBoolean],
                    input,
                    text,
                ),
                ExportSetting::Album => Self::get_input_abc123_tts(
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_ALBUM_ABC123",
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_ALBUM_NO_ABC123",
                    state,
                    input,
                    text,
                ),
                ExportSetting::Genre => Self::get_input_abc123_tts(
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_GENRE_ABC123",
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_GENRE_NO_ABC123",
                    state,
                    input,
                    text,
                ),
                ExportSetting::Comment => Self::get_input_abc123_tts(
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_COMMENT_ABC123",
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_COMMENT_NO_ABC123",
                    state,
                    input,
                    text,
                ),
                ExportSetting::TrackNumber => Self::get_input_lr_tts(
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_TRACK_NUMBER",
                    input,
                    text,
                ),
                ExportSetting::Mp3BitRate => Self::get_input_lr_tts(
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_MP3_BIT_RATE",
                    input,
                    text,
                ),
                ExportSetting::Mp3Quality | ExportSetting::OggQuality => {
                    Self::get_input_lr_tts("EXPORT_SETTINGS_PANEL_INPUT_TTS_QUALITY", input, text)
                }
            };
            tts.say(&s);
            None
        }
        // Previous setting.
        else if input.happened(&InputEvent::PreviousExportSetting) {
            settings.index.increment(false);
            None
        }
        // Next setting.
        else if input.happened(&InputEvent::NextExportSetting) {
            settings.index.increment(true);
            None
        } else {
            match &settings.get() {
                // Framerate.
                ExportSetting::Framerate => {
                    if input.happened(&InputEvent::PreviousExportSettingValue) {
                        Self::set_framerate(conn, exporter, false)
                    } else if input.happened(&InputEvent::NextExportSettingValue) {
                        Self::set_framerate(conn, exporter, true)
                    } else {
                        None
                    }
                }
                ExportSetting::Copyright => {
                    if input.happened(&InputEvent::ToggleExportSettingBoolean) {
                        let c0 = vec![Command::SetExporter {
                            exporter: Box::new(exporter.clone()),
                        }];
                        exporter.copyright = !exporter.copyright;
                        Self::set_exporter(c0, conn, exporter)
                    } else {
                        None
                    }
                }
                ExportSetting::Title => {
                    edit_string(|e| &mut e.metadata.title, input, conn, state, exporter)
                }
                ExportSetting::Artist => {
                    edit_optional_string(|e| &mut e.metadata.artist, input, conn, state, exporter)
                }
                ExportSetting::Album => {
                    edit_optional_string(|e| &mut e.metadata.album, input, conn, state, exporter)
                }
                ExportSetting::Genre => {
                    edit_optional_string(|e| &mut e.metadata.genre, input, conn, state, exporter)
                }
                ExportSetting::Comment => {
                    edit_optional_string(|e| &mut e.metadata.comment, input, conn, state, exporter)
                }
                ExportSetting::TrackNumber => {
                    if input.happened(&InputEvent::PreviousExportSettingValue) {
                        Self::set_track_number(conn, exporter, false)
                    } else if input.happened(&InputEvent::NextExportSettingValue) {
                        Self::set_track_number(conn, exporter, true)
                    } else {
                        None
                    }
                }
                ExportSetting::Mp3BitRate => {
                    Self::set_index(|e| &mut e.mp3_bit_rate, conn, input, exporter)
                }
                ExportSetting::Mp3Quality => {
                    Self::set_index(|e| &mut e.mp3_quality, conn, input, exporter)
                }
                ExportSetting::OggQuality => {
                    Self::set_index(|e| &mut e.ogg_quality, conn, input, exporter)
                }
            }
        }
    }
}

impl Panel for ExportSettingsPanel {
    fn update(
        &mut self,
        state: &mut State,
        conn: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &Text,
        _: &mut PathsState,
        exporter: &mut Exporter,
    ) -> Option<Snapshot> {
        match exporter.export_type.get() {
            ExportType::Mid => Self::update_settings(
                &mut self.mid_settings,
                state,
                conn,
                input,
                tts,
                text,
                exporter,
            ),
            ExportType::MP3 => Self::update_settings(
                &mut self.mp3_settings,
                state,
                conn,
                input,
                tts,
                text,
                exporter,
            ),
            ExportType::Ogg => Self::update_settings(
                &mut self.ogg_settings,
                state,
                conn,
                input,
                tts,
                text,
                exporter,
            ),
            ExportType::Wav => Self::update_settings(
                &mut self.wav_settings,
                state,
                conn,
                input,
                tts,
                text,
                exporter,
            ),
        }
    }
}
