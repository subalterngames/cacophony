use crate::abc123::{on_disable_exporter, update_exporter};
use crate::panel::*;
use audio::exporter::*;
use audio::Conn;
use common::{IndexedValues, U64orF32};
use serde::de::DeserializeOwned;
use serde::Serialize;

/// All possible audio framerates.
const FRAMERATES: [u64; 3] = [22050, 44100, 48000];

/// Set the values of export settings.
#[derive(Default)]
pub(crate) struct ExportSettingsPanel {
    tooltips: Tooltips,
}

impl ExportSettingsPanel {
    /// Returns the text-to-speech status string for an alphanumeric field.
    ///
    /// - `tooltips` The tooltips handler.
    /// - `if_true` The text key to use if alphanumeric input is enabled.
    /// - `if_false` The text key to use if alphanumeric input isn't enabled.
    /// - `value` The value string, if any. If none, a default string will be used.
    /// - `state` The app state.
    /// - `input` The input state.
    /// - `text` The text state.
    fn get_status_abc123_tts(
        tooltips: &mut Tooltips,
        if_true: &str,
        if_false: &str,
        value: &Option<String>,
        state: &State,
        input: &Input,
        text: &Text,
    ) -> TtsString {
        let n = text.get_ref("NONE");
        let value = match value {
            Some(value) => value.as_str(),
            None => n,
        };
        if state.input.alphanumeric_input {
            TtsString::from(text.get_with_values(if_true, &[value]))
        } else {
            tooltips.get_tooltip_with_values(
                if_false,
                &[InputEvent::ToggleAlphanumericInput],
                &[value],
                input,
                text,
            )
        }
    }

    /// Returns the text-to-speech status string for an boolean field.
    ///
    /// - `tooltips` The tooltips handler.
    /// - `if_true` The text key to use if the boolean is true.
    /// - `if_false` The text key to use if the boolean is false.
    /// - `value` The value.
    /// - `input` The input state.
    /// - `text` The text state.
    fn get_status_bool_tts(
        tooltips: &mut Tooltips,
        if_true: &str,
        if_false: &str,
        value: bool,
        input: &Input,
        text: &Text,
    ) -> TtsString {
        tooltips.get_tooltip(
            if value { if_true } else { if_false },
            &[InputEvent::ToggleExportSettingBoolean],
            input,
            text,
        )
    }

    /// Returns the text-to-speech input string for scrolling.
    ///
    /// - `tooltips` The tooltips handler.
    /// - `input` The input state.
    /// - `text` The text state.
    fn get_input_scroll_tts(tooltips: &mut Tooltips, input: &Input, text: &Text) -> TtsString {
        tooltips.get_tooltip(
            "EXPORT_SETTINGS_PANEL_INPUT_TTS_SCROLL",
            &[
                InputEvent::PreviousExportSetting,
                InputEvent::NextExportSetting,
            ],
            input,
            text,
        )
    }

    /// Returns the text-to-speech input string for an alphanumeric field.
    ///
    /// - `tooltips` The tooltips handler.
    /// - `if_true` The text key to use if alphanumeric input is enabled.
    /// - `if_false` The text key to use if alphanumeric input isn't enabled.
    /// - `state` The app state.
    /// - `input` The input state.
    /// - `text` The text state.
    fn get_input_abc123_tts(
        tooltips: &mut Tooltips,
        if_true: &str,
        if_false: &str,
        state: &State,
        input: &Input,
        text: &Text,
    ) -> Vec<TtsString> {
        if state.input.alphanumeric_input {
            vec![tooltips
                .get_tooltip(if_true, &[InputEvent::ToggleAlphanumericInput], input, text)
                .clone()]
        } else {
            vec![
                tooltips
                    .get_tooltip(
                        if_false,
                        &[InputEvent::ToggleAlphanumericInput],
                        input,
                        text,
                    )
                    .clone(),
                Self::get_input_scroll_tts(tooltips, input, text),
            ]
        }
    }

    /// Returns the text-to-speech input string cycling a field's value up or down.
    ///
    /// - `tooltips` The tooltips handler.
    /// - `key` The text key.
    /// - `input` The input state.
    /// - `text` The text state.
    fn get_input_lr_tts(
        tooltips: &mut Tooltips,
        key: &str,
        input: &Input,
        text: &Text,
    ) -> Vec<TtsString> {
        vec![
            tooltips.get_tooltip(
                key,
                &[
                    InputEvent::PreviousExportSettingValue,
                    InputEvent::NextExportSettingValue,
                ],
                input,
                text,
            ),
            Self::get_input_scroll_tts(tooltips, input, text),
        ]
    }

    /// Set the export framerate.
    ///
    /// - `exporter` The exporter. This will have its framerate set.
    /// - `up` Increment or decrement along the `FRAMERATES` array.
    fn set_framerate(exporter: &mut Exporter, up: bool) {
        let i = FRAMERATES
            .iter()
            .position(|f| *f == exporter.framerate.get_u())
            .unwrap();
        let mut index = Index::new(i, FRAMERATES.len());
        index.increment(up);
        exporter.framerate = U64orF32::from(FRAMERATES[index.get()]);
    }

    /// Set the track number.
    ///
    /// - `exporter` The exporter. This will have its framerate set.
    /// - `up` Add or subtract the frame number.
    fn set_track_number(exporter: &mut Exporter, up: bool) {
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
    }

    /// Set an `Index` field within an `Exporter`.
    ///
    /// - `f` A closure that returns a mutable reference to an `Index`.
    /// - `input` The input state.
    /// - `exporter` The exporter.
    fn set_index<F>(mut f: F, input: &Input, exporter: &mut Exporter)
    where
        F: FnMut(&mut Exporter) -> &mut Index<usize>,
    {
        if input.happened(&InputEvent::PreviousExportSettingValue) {
            f(exporter).increment(false);
        } else if input.happened(&InputEvent::NextExportSettingValue) {
            f(exporter).increment(true);
        }
    }

    /// Update settings for a given export type.
    ///
    /// - `f` A closure that returns a mutable reference to an `IndexValues` of export settings (corresponding to the export type).
    /// - `tooltips` The tooltips handler.
    /// - `state` The app state.
    /// - `input` The input state.
    /// - `tts` Text-to-speech.
    /// - `text` The text state.
    /// - `exporter` The exporter. This will have its framerate set.
    fn update_settings<F, const N: usize>(
        mut f: F,
        state: &mut State,
        tooltips: &mut Tooltips,
        input: &Input,
        tts: &mut TTS,
        text: &Text,
        exporter: &mut SharedExporter,
    ) -> Option<Snapshot>
    where
        F: FnMut(&mut Exporter) -> &mut IndexedValues<ExportSetting, N>,
        [ExportSetting; N]: Serialize + DeserializeOwned,
    {
        // Status TTS.
        if input.happened(&InputEvent::StatusTTS) {
            let mut ex = exporter.lock();
            let s = match &f(&mut ex).get() {
                ExportSetting::Framerate => {
                    TtsString::from(text.get("EXPORT_SETTINGS_PANEL_STATUS_TTS_FRAMERATE"))
                }
                ExportSetting::Title => Self::get_status_abc123_tts(
                    tooltips,
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_TITLE_ABC123",
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_TITLE_NO_ABC123",
                    &Some(ex.metadata.title.clone()),
                    state,
                    input,
                    text,
                ),
                ExportSetting::Artist => Self::get_status_abc123_tts(
                    tooltips,
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_ARTIST",
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_ARTIST_NO_ABC123",
                    &ex.metadata.artist,
                    state,
                    input,
                    text,
                ),
                ExportSetting::Copyright => Self::get_status_bool_tts(
                    tooltips,
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_COPYRIGHT_ENABLED",
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_COPYRIGHT_DISABLED",
                    ex.copyright,
                    input,
                    text,
                ),
                ExportSetting::Album => Self::get_status_abc123_tts(
                    tooltips,
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_ALBUM_ABC123",
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_ALBUM_NO_ABC123",
                    &ex.metadata.album,
                    state,
                    input,
                    text,
                ),
                ExportSetting::Genre => Self::get_status_abc123_tts(
                    tooltips,
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_GENRE_ABC123",
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_GENRE_NO_ABC123",
                    &ex.metadata.genre,
                    state,
                    input,
                    text,
                ),
                ExportSetting::Comment => Self::get_status_abc123_tts(
                    tooltips,
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_COMMENT_ABC123",
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_COMMENT_NO_ABC123",
                    &ex.metadata.comment,
                    state,
                    input,
                    text,
                ),
                ExportSetting::Mp3BitRate => TtsString::from(text.get_with_values(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_BIT_RATE",
                    &[&((MP3_BIT_RATES[ex.mp3_bit_rate.get()] as u16) as u32 * 1000).to_string()],
                )),
                ExportSetting::Mp3Quality => TtsString::from(text.get_with_values(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_QUALITY",
                    &[&ex.mp3_quality.get().to_string()],
                )),
                ExportSetting::OggQuality => TtsString::from(text.get_with_values(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_QUALITY",
                    &[&exporter.lock().ogg_quality.get().to_string()],
                )),
                ExportSetting::TrackNumber => TtsString::from(text.get_with_values(
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_TRACK_NUMBER",
                    &[&match ex.metadata.track_number {
                        Some(track_number) => track_number.to_string(),
                        None => text.get("NONE"),
                    }],
                )),
                ExportSetting::MultiFile => Self::get_status_bool_tts(
                    tooltips,
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_MULTI_FILE_ENABLED",
                    "EXPORT_SETTINGS_PANEL_STATUS_TTS_MULTI_FILE_DISABLED",
                    ex.multi_file,
                    input,
                    text,
                ),
                ExportSetting::MultiFileSuffix => {
                    let key = match &ex.multi_file_suffix.get() {
                        MultiFileSuffix::Preset => {
                            "EXPORT_SETTINGS_PANEL_STATUS_TTS_MULTI_FILE_PRESET"
                        }
                        MultiFileSuffix::Channel => {
                            "EXPORT_SETTINGS_PANEL_STATUS_TTS_MULTI_FILE_CHANNEL"
                        }
                        MultiFileSuffix::ChannelAndPreset => {
                            "EXPORT_SETTINGS_PANEL_STATUS_TTS_MULTI_FILE_CHANNEL_AND_PRESET"
                        }
                    };
                    TtsString::from(text.get_ref(key))
                }
            };
            tts.enqueue(s);
        }
        // Input TTS.
        else if input.happened(&InputEvent::InputTTS) {
            let mut ex = exporter.lock();
            let s = match &f(&mut ex).get() {
                ExportSetting::Framerate => Self::get_input_lr_tts(
                    tooltips,
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_FRAMERATE",
                    input,
                    text,
                ),
                ExportSetting::Title => Self::get_input_abc123_tts(
                    tooltips,
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_TITLE_ABC123",
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_TITLE_NO_ABC123",
                    state,
                    input,
                    text,
                ),
                ExportSetting::Artist => Self::get_input_abc123_tts(
                    tooltips,
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_ARTIST_ABC123",
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_ARTIST_NO_ABC123",
                    state,
                    input,
                    text,
                ),
                ExportSetting::Copyright => vec![
                    tooltips
                        .get_tooltip(
                            "EXPORT_SETTINGS_PANEL_INPUT_TTS_COPYRIGHT",
                            &[InputEvent::ToggleExportSettingBoolean],
                            input,
                            text,
                        )
                        .clone(),
                    Self::get_input_scroll_tts(tooltips, input, text),
                ],
                ExportSetting::Album => Self::get_input_abc123_tts(
                    tooltips,
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_ALBUM_ABC123",
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_ALBUM_NO_ABC123",
                    state,
                    input,
                    text,
                ),
                ExportSetting::Genre => Self::get_input_abc123_tts(
                    tooltips,
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_GENRE_ABC123",
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_GENRE_NO_ABC123",
                    state,
                    input,
                    text,
                ),
                ExportSetting::Comment => Self::get_input_abc123_tts(
                    tooltips,
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_COMMENT_ABC123",
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_COMMENT_NO_ABC123",
                    state,
                    input,
                    text,
                ),
                ExportSetting::TrackNumber => Self::get_input_lr_tts(
                    tooltips,
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_TRACK_NUMBER",
                    input,
                    text,
                ),
                ExportSetting::Mp3BitRate => Self::get_input_lr_tts(
                    tooltips,
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_MP3_BIT_RATE",
                    input,
                    text,
                ),
                ExportSetting::Mp3Quality | ExportSetting::OggQuality => Self::get_input_lr_tts(
                    tooltips,
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_QUALITY",
                    input,
                    text,
                ),
                ExportSetting::MultiFile => vec![
                    tooltips
                        .get_tooltip(
                            "EXPORT_SETTINGS_PANEL_INPUT_TTS_MULTI_FILE",
                            &[InputEvent::ToggleExportSettingBoolean],
                            input,
                            text,
                        )
                        .clone(),
                    Self::get_input_scroll_tts(tooltips, input, text),
                ],
                ExportSetting::MultiFileSuffix => Self::get_input_lr_tts(
                    tooltips,
                    "EXPORT_SETTINGS_PANEL_INPUT_TTS_MULTI_FILE_SUFFIX",
                    input,
                    text,
                ),
            };
            tts.enqueue(s);
        }
        // Previous setting.
        else if input.happened(&InputEvent::PreviousExportSetting) {
            let mut ex = exporter.lock();
            let s = f(&mut ex);
            s.index.increment(false);
        }
        // Next setting.
        else if input.happened(&InputEvent::NextExportSetting) {
            let mut ex = exporter.lock();
            let s = f(&mut ex);
            s.index.increment(true);
        } else {
            let mut ex = exporter.lock();
            match &f(&mut ex).get() {
                // Framerate.
                ExportSetting::Framerate => {
                    if input.happened(&InputEvent::PreviousExportSettingValue) {
                        Self::set_framerate(&mut ex, false);
                    } else if input.happened(&InputEvent::NextExportSettingValue) {
                        Self::set_framerate(&mut ex, true);
                    }
                }
                ExportSetting::Copyright => {
                    if input.happened(&InputEvent::ToggleExportSettingBoolean) {
                        ex.copyright = !ex.copyright;
                    }
                }
                ExportSetting::TrackNumber => {
                    if input.happened(&InputEvent::PreviousExportSettingValue) {
                        Self::set_track_number(&mut ex, false);
                    } else if input.happened(&InputEvent::NextExportSettingValue) {
                        Self::set_track_number(&mut ex, true);
                    }
                }
                ExportSetting::Mp3BitRate => {
                    Self::set_index(|e| &mut e.mp3_bit_rate, input, &mut ex);
                }
                ExportSetting::Mp3Quality => {
                    Self::set_index(|e| &mut e.mp3_quality, input, &mut ex);
                }
                ExportSetting::OggQuality => {
                    Self::set_index(|e| &mut e.ogg_quality, input, &mut ex);
                }
                ExportSetting::MultiFile => {
                    if input.happened(&InputEvent::ToggleExportSettingBoolean) {
                        ex.multi_file = !ex.multi_file;
                    }
                }
                ExportSetting::MultiFileSuffix => {
                    Self::set_index(
                        |e: &mut Exporter| &mut e.multi_file_suffix.index,
                        input,
                        &mut ex,
                    );
                }
                _ => (),
            }
        }
        None
    }

    /// Update settings for a given export type during alphanumeric input.
    ///
    /// - `f` A closure that returns a mutable reference to an `IndexValues` of export settings (corresponding to the export type).
    /// - `input` The input state.
    /// - `exporter` The exporter. This will have its framerate set.
    fn update_settings_abc123<F, const N: usize>(
        mut f: F,
        input: &Input,
        exporter: &mut SharedExporter,
    ) -> bool
    where
        F: FnMut(&mut Exporter) -> &mut IndexedValues<ExportSetting, N>,
        [ExportSetting; N]: Serialize + DeserializeOwned,
    {
        let mut ex = exporter.lock();
        match &f(&mut ex).get() {
            ExportSetting::Title => update_exporter(|e| &mut e.metadata.title, input, &mut ex),
            ExportSetting::Artist => update_exporter(|e| &mut e.metadata.artist, input, &mut ex),
            ExportSetting::Album => update_exporter(|e| &mut e.metadata.album, input, &mut ex),
            ExportSetting::Genre => update_exporter(|e| &mut e.metadata.genre, input, &mut ex),
            ExportSetting::Comment => update_exporter(|e| &mut e.metadata.comment, input, &mut ex),
            _ => false,
        }
    }

    /// Do something when alphanumeric input is disabled.
    ///
    /// - `f` A closure that returns a mutable reference to an `IndexValues` of export settings (corresponding to the export type).
    /// - `exporter` The exporter. This will have its framerate set.
    fn disable_abc123<F, const N: usize>(mut f: F, exporter: &mut SharedExporter)
    where
        F: FnMut(&mut Exporter) -> &mut IndexedValues<ExportSetting, N>,
        [ExportSetting; N]: Serialize + DeserializeOwned,
    {
        let mut ex = exporter.lock();
        match &f(&mut ex).get() {
            ExportSetting::Title => {
                on_disable_exporter(|e| &mut e.metadata.title, &mut ex, "My Music".to_string())
            }
            ExportSetting::Artist => on_disable_exporter(|e| &mut e.metadata.artist, &mut ex, None),
            ExportSetting::Album => on_disable_exporter(|e| &mut e.metadata.album, &mut ex, None),
            ExportSetting::Genre => on_disable_exporter(|e| &mut e.metadata.genre, &mut ex, None),
            ExportSetting::Comment => {
                on_disable_exporter(|e| &mut e.metadata.comment, &mut ex, None)
            }
            _ => (),
        }
    }

    /// Returns true if we can toggle alphanumeric input.
    ///
    /// - `f` A closure that returns a mutable reference to an `IndexValues` of export settings (corresponding to the export type).
    /// - `exporter` The exporter. This will have its framerate set.
    fn allow_abc123<F, const N: usize>(f: F, exporter: &SharedExporter) -> bool
    where
        F: Fn(&Exporter) -> &IndexedValues<ExportSetting, N>,
        [ExportSetting; N]: Serialize + DeserializeOwned,
    {
        let ex = exporter.lock();
        matches!(
            &f(&ex).get(),
            ExportSetting::Title
                | ExportSetting::Artist
                | ExportSetting::Album
                | ExportSetting::Genre
                | ExportSetting::Comment,
        )
    }
}

impl Panel for ExportSettingsPanel {
    fn update(
        &mut self,
        state: &mut State,
        _: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &Text,
        _: &mut PathsState,
        exporter: &mut SharedExporter,
    ) -> Option<Snapshot> {
        // Close this.
        if input.happened(&InputEvent::CloseOpenFile) {
            return Some(Snapshot::from_io_commands(vec![IOCommand::CloseOpenFile]));
        }
        let export_type = exporter.lock().export_type.get();
        match export_type {
            ExportType::Mid => Self::update_settings(
                |e| &mut e.mid_settings,
                state,
                &mut self.tooltips,
                input,
                tts,
                text,
                exporter,
            ),
            ExportType::MP3 => Self::update_settings(
                |e| &mut e.mp3_settings,
                state,
                &mut self.tooltips,
                input,
                tts,
                text,
                exporter,
            ),
            ExportType::Ogg => Self::update_settings(
                |e| &mut e.ogg_settings,
                state,
                &mut self.tooltips,
                input,
                tts,
                text,
                exporter,
            ),
            ExportType::Flac => Self::update_settings(
                |e| &mut e.flac_settings,
                state,
                &mut self.tooltips,
                input,
                tts,
                text,
                exporter,
            ),
            ExportType::Wav => Self::update_settings(
                |e| &mut e.wav_settings,
                state,
                &mut self.tooltips,
                input,
                tts,
                text,
                exporter,
            ),
        }
    }

    fn update_abc123(
        &mut self,
        _: &mut State,
        input: &Input,
        exporter: &mut SharedExporter,
    ) -> (Option<Snapshot>, bool) {
        let export_type = exporter.lock().export_type.get();
        let updated = match export_type {
            ExportType::Mid => {
                Self::update_settings_abc123(|e| &mut e.mid_settings, input, exporter)
            }
            ExportType::MP3 => {
                Self::update_settings_abc123(|e| &mut e.mp3_settings, input, exporter)
            }
            ExportType::Ogg => {
                Self::update_settings_abc123(|e| &mut e.ogg_settings, input, exporter)
            }
            ExportType::Flac => {
                Self::update_settings_abc123(|e| &mut e.flac_settings, input, exporter)
            }
            ExportType::Wav => {
                Self::update_settings_abc123(|e| &mut e.wav_settings, input, exporter)
            }
        };
        (None, updated)
    }

    fn on_disable_abc123(&mut self, _: &mut State, exporter: &mut SharedExporter) {
        let export_type = exporter.lock().export_type.get();
        match export_type {
            ExportType::Mid => Self::disable_abc123(|e| &mut e.mid_settings, exporter),
            ExportType::MP3 => Self::disable_abc123(|e| &mut e.mp3_settings, exporter),
            ExportType::Ogg => Self::disable_abc123(|e| &mut e.ogg_settings, exporter),
            ExportType::Flac => Self::disable_abc123(|e| &mut e.flac_settings, exporter),
            ExportType::Wav => Self::disable_abc123(|e| &mut e.wav_settings, exporter),
        };
    }

    fn allow_alphanumeric_input(&self, _: &State, exporter: &SharedExporter) -> bool {
        let export_type = exporter.lock().export_type.get();
        match export_type {
            ExportType::Mid => Self::allow_abc123(|e| &e.mid_settings, exporter),
            ExportType::MP3 => Self::allow_abc123(|e| &e.mp3_settings, exporter),
            ExportType::Ogg => Self::allow_abc123(|e| &e.ogg_settings, exporter),
            ExportType::Flac => Self::allow_abc123(|e| &e.flac_settings, exporter),
            ExportType::Wav => Self::allow_abc123(|e: &Exporter| &e.wav_settings, exporter),
        }
    }

    fn allow_play_music(&self) -> bool {
        false
    }
}
