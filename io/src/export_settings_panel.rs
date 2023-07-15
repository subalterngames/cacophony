use crate::abc123::abc123_exporter;
use crate::panel::*;
use audio::exporter::*;
use audio::Conn;
use common::{IndexedValues, U64orF32};
use serde::de::DeserializeOwned;
use serde::Serialize;

const FRAMERATES: [u64; 3] = [22050, 44100, 48000];

pub(crate) struct ExportSettingsPanel {
    tooltips: Tooltips
}

impl ExportSettingsPanel {
    pub fn new(text: &Text) -> Self {
        Self { tooltips: Tooltips::new(text)}
    }

    fn get_status_ab123_tts(
        &mut self,
        if_true: &str,
        if_false: &str,
        value: &Option<String>,
        state: &State,
        input: &Input,
        text: &Text,
    ) -> TtsString {
        let n = text.get("NONE");
        let value = value.as_ref().unwrap_or(&n);
        if state.input.alphanumeric_input {
            TtsString::from(text.get_with_values(if_true, &[value]))
        } else {
            self.tooltips.get_tooltip_with_values(
                if_false,
                &[InputEvent::ToggleAlphanumericInput],
                &[value],
                input,
                text,
            )
        }
    }

    fn get_status_bool_tts(
        &mut self,
        if_true: &str,
        if_false: &str,
        value: bool,
        input: &Input,
        text: &Text,
    ) -> TtsString {
        self.tooltips.get_tooltip(
            if value { if_true } else { if_false },
            &[InputEvent::ToggleExportSettingBoolean],
            input,
            text,
        )
    }

    fn get_input_abc123_tts(
        &mut self,
        if_true: &str,
        if_false: &str,
        state: &State,
        input: &Input,
        text: &Text,
    ) -> TtsString {
        if state.input.alphanumeric_input {
            self.tooltips.get_tooltip(if_true, &[InputEvent::ToggleAlphanumericInput], input, text)
        } else {
            let mut s = self.tooltips.get_tooltip(
                if_false,
                &[InputEvent::ToggleAlphanumericInput],
                input,
                text,
            );
            s.append(&self.tooltips.get_tooltip(
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

    fn get_input_lr_tts(&mut self, key: &str, input: &Input, text: &Text) -> TtsString {
        self.tooltips.get_tooltip(
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
        let i = FRAMERATES
            .iter()
            .position(|f| *f == exporter.framerate.get_u())
            .unwrap();
        let mut index = Index::new(i, FRAMERATES.len());
        index.increment(up);
        Some(Snapshot::from_exporter_value(
            |e| &mut e.framerate,
            U64orF32::from(FRAMERATES[index.get()]),
            conn,
            exporter,
        ))
    }

    fn set_track_number(conn: &mut Conn, exporter: &mut Exporter, up: bool) -> Option<Snapshot> {
        let track_number = if up {
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
        Some(Snapshot::from_exporter_value(
            |e| &mut e.metadata.track_number,
            track_number,
            conn,
            exporter,
        ))
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
            Some(Snapshot::from_exporter(
                |e| f(e).increment(false),
                conn,
                exporter,
            ))
        } else if input.happened(&InputEvent::NextExportSettingValue) {
            Some(Snapshot::from_exporter(
                |e| f(e).increment(true),
                conn,
                exporter,
            ))
        } else {
            None
        }
    }

    fn update_settings<F, const N: usize>(
        &mut self,
        mut f: F,
        state: &mut State,
        conn: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &Text,
        exporter: &mut Exporter,
    ) -> Option<Snapshot>
    where
        F: FnMut(&mut Exporter) -> &mut IndexedValues<ExportSetting, N>,
        [ExportSetting; N]: Serialize + DeserializeOwned,
    {
        // Status TTS.
        if input.happened(&InputEvent::StatusTTS) {
            let s = match &f(exporter).get() {
                ExportSetting::Framerate => TtsString::from(text.get("EXPORT_SETTINGS_PANEL_STATUS_TTS_FRAMERATE")),
                ExportSetting::Title => self.get_status_ab123_tts(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_TITLE_ABC123",
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_TITLE_NO_ABC123",
                    &Some(exporter.metadata.title.clone()),
                    state,
                    input,
                    text,
                ),
                ExportSetting::Artist => self.get_status_ab123_tts(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_ARTIST",
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_ARTIST_NO_ABC123",
                    &exporter.metadata.artist,
                    state,
                    input,
                    text,
                ),
                ExportSetting::Copyright => self.get_status_bool_tts(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_COPYRIGHT_ENABLED",
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_COPYRIGHT_DISABLED",
                    exporter.copyright,
                    input,
                    text,
                ),
                ExportSetting::Album => self.get_status_ab123_tts(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_ALBUM_ABC123",
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_ALBUM_NO_ABC123",
                    &exporter.metadata.album,
                    state,
                    input,
                    text,
                ),
                ExportSetting::Genre => self.get_status_ab123_tts(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_GENRE_ABC123",
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_GENRE_NO_ABC123",
                    &exporter.metadata.genre,
                    state,
                    input,
                    text,
                ),
                ExportSetting::Comment => self.get_status_ab123_tts(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_COMMENT_ABC123",
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_COMMENT_NO_ABC123",
                    &exporter.metadata.comment,
                    state,
                    input,
                    text,
                ),
                ExportSetting::Mp3BitRate => TtsString::from(text.get_with_values(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_BIT_RATE",
                    &[
                        &((MP3_BIT_RATES[exporter.bit_rate.get()] as u16) as u32 * 1000)
                            .to_string(),
                    ]),
                ),
                ExportSetting::Mp3Quality => TtsString::from(text.get_with_values(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_QUALITY",
                    &[&exporter.mp3_quality.get().to_string()],
                )),
                ExportSetting::OggQuality => TtsString::from(text.get_with_values(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_QUALITY",
                    &[&exporter.ogg_quality.get().to_string()],
                )),
                ExportSetting::TrackNumber => TtsString::from(text.get_with_values(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_TRACK_NUMBER",
                    &[&match exporter.metadata.track_number {
                        Some(track_number) => track_number.to_string(),
                        None => text.get("NONE"),
                    }],
                )),
                ExportSetting::MultiFile => TtsString::from(self.get_status_bool_tts(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_MULTI_FILE_ENABLED",
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_MULTI_FILE_DISABLED",
                    exporter.multi_file,
                    input,
                    text,
                )),
                ExportSetting::MultiFileSuffix => {
                    if exporter.multi_file {
                        let key = match &exporter.multi_file_suffix.get() {
                            MultiFile::Preset => {
                                "EXPORT_SETTINGS_PANEL_STATUS_TTS_MULTI_FILE_PRESET"
                            }
                            MultiFile::Channel => {
                                "EXPORT_SETTINGS_PANEL_STATUS_TTS_MULTI_FILE_CHANNEL"
                            }
                            MultiFile::ChannelAndPreset => {
                                "EXPORT_SETTINGS_PANEL_STATUS_TTS_MULTI_FILE_CHANNEL_AND_PRESET"
                            }
                        };
                        TtsString::from(text.get(key))
                    } else {
                        return None;
                    }
                }
            };
            tts.say(s);
            None
        }
        // Input TTS.
        else if input.happened(&InputEvent::InputTTS) {
            let s = match &f(exporter).get() {
                ExportSetting::Framerate => {
                    self.get_input_lr_tts("EXPORT_SETTINGS_PANEL_INPUT_TTS_FRAMERATE", input, text)
                }
                ExportSetting::Title => self.get_input_abc123_tts(
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_TITLE_ABC123",
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_TITLE_NO_ABC123",
                    state,
                    input,
                    text,
                ),
                ExportSetting::Artist => self.get_input_abc123_tts(
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_ARTIST_ABC123",
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_ARTIST_NO_ABC123",
                    state,
                    input,
                    text,
                ),
                ExportSetting::Copyright => self.tooltips.get_tooltip(
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_COPYRIGHT",
                    &[InputEvent::ToggleExportSettingBoolean],
                    input,
                    text,
                ),
                ExportSetting::Album => self.get_input_abc123_tts(
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_ALBUM_ABC123",
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_ALBUM_NO_ABC123",
                    state,
                    input,
                    text,
                ),
                ExportSetting::Genre => self.get_input_abc123_tts(
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_GENRE_ABC123",
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_GENRE_NO_ABC123",
                    state,
                    input,
                    text,
                ),
                ExportSetting::Comment => self.get_input_abc123_tts(
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_COMMENT_ABC123",
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_COMMENT_NO_ABC123",
                    state,
                    input,
                    text,
                ),
                ExportSetting::TrackNumber => self.get_input_lr_tts(
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_TRACK_NUMBER",
                    input,
                    text,
                ),
                ExportSetting::Mp3BitRate => self.get_input_lr_tts(
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_MP3_BIT_RATE",
                    input,
                    text,
                ),
                ExportSetting::Mp3Quality | ExportSetting::OggQuality => {
                    self.get_input_lr_tts("EXPORT_SETTINGS_PANEL_INPUT_TTS_QUALITY", input, text)
                }
                ExportSetting::MultiFile => self.tooltips.get_tooltip(
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_MULTI_FILE",
                    &[InputEvent::ToggleExportSettingBoolean],
                    input,
                    text,
                ),
                ExportSetting::MultiFileSuffix => self.get_input_lr_tts(
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_MULTI_FILE_SUFFIX",
                    input,
                    text,
                ),
            };
            tts.say(s);
            None
        }
        // Previous setting.
        else if input.happened(&InputEvent::PreviousExportSetting) {
            let s = f(exporter);
            s.index.increment(false);
            None
        }
        // Next setting.
        else if input.happened(&InputEvent::NextExportSetting) {
            let s = f(exporter);
            s.index.increment(true);
            None
        } else {
            match &f(exporter).get() {
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
                        Some(Snapshot::from_exporter_value(
                            |e| &mut e.copyright,
                            !exporter.copyright,
                            conn,
                            exporter,
                        ))
                    } else {
                        None
                    }
                }
                ExportSetting::Title => abc123_exporter(
                    |e| &mut e.metadata.title,
                    state,
                    conn,
                    input,
                    exporter,
                    "My Music".to_string(),
                ),
                ExportSetting::Artist => abc123_exporter(
                    |e| &mut e.metadata.artist,
                    state,
                    conn,
                    input,
                    exporter,
                    None,
                ),
                ExportSetting::Album => abc123_exporter(
                    |e| &mut e.metadata.album,
                    state,
                    conn,
                    input,
                    exporter,
                    None,
                ),
                ExportSetting::Genre => abc123_exporter(
                    |e| &mut e.metadata.genre,
                    state,
                    conn,
                    input,
                    exporter,
                    None,
                ),
                ExportSetting::Comment => abc123_exporter(
                    |e| &mut e.metadata.comment,
                    state,
                    conn,
                    input,
                    exporter,
                    None,
                ),
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
                ExportSetting::MultiFile => {
                    if input.happened(&InputEvent::ToggleExportSettingBoolean) {
                        Some(Snapshot::from_exporter_value(
                            |e: &mut Exporter| &mut e.multi_file,
                            !exporter.multi_file,
                            conn,
                            exporter,
                        ))
                    } else {
                        None
                    }
                }
                ExportSetting::MultiFileSuffix => Self::set_index(
                    |e: &mut Exporter| &mut e.multi_file_suffix.index,
                    conn,
                    input,
                    exporter,
                ),
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
        // Close this.
        if input.happened(&InputEvent::CloseOpenFile) {
            return Some(Snapshot::from_io_commands(vec![IOCommand::CloseOpenFile]));
        }
        match exporter.export_type.get() {
            ExportType::Mid => self.update_settings(
                |e| &mut e.mid_settings,
                state,
                conn,
                input,
                tts,
                text,
                exporter,
            ),
            ExportType::MP3 => self.update_settings(
                |e: &mut Exporter| &mut e.mp3_settings,
                state,
                conn,
                input,
                tts,
                text,
                exporter,
            ),
            ExportType::Ogg => self.update_settings(
                |e| &mut e.ogg_settings,
                state,
                conn,
                input,
                tts,
                text,
                exporter,
            ),
            ExportType::Wav => self.update_settings(
                |e| &mut e.wav_settings,
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
