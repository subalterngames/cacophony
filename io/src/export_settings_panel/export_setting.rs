use common::State;
use input::{Input, InputEvent};
use text::{Text, TTS};
use tooltip::get_tooltip;

#[derive(Eq, PartialEq, Copy, Clone)]
pub(super) enum ExportSetting {
    Title,
    Artist,
    Copyright,
}

impl ExportSetting {
    pub(super) fn say(&self, state: &State, input: &Input, text: &Text, tts: &mut TTS) -> bool {
        if input.happened(&InputEvent::StatusTTS) {
            tts.say(&self.get_status_tts(state, input, text));
            true
        } else if input.happened(&InputEvent::InputTTS) {
            tts.say(&self.get_input_tts(state, input, text));
            true
        } else {
            false
        }
    }

    fn get_status_tts(&self, state: &State, input: &Input, text: &Text) -> String {
        match self {
            Self::Title => {
                if state.input.alphanumeric_input {
                    text.get("EXPORT_SETTINGS_PANEL_STATUS_TTS_TITLE_ABC123")
                } else {
                    get_tooltip(
                        "EXPORT_SETTINGS_PANEL_STATUS_TTS_TITLE_NO_ABC123",
                        &[InputEvent::ToggleAlphanumericInput],
                        input,
                        text,
                    )
                }
            }
            Self::Artist => {
                if state.input.alphanumeric_input {
                    text.get("EXPORT_SETTINGS_PANEL_STATUS_TTS_ARTIST")
                } else {
                    get_tooltip(
                        "EXPORT_SETTINGS_PANEL_STATUS_TTS_ARTIST_NO_ABC123",
                        &[InputEvent::ToggleAlphanumericInput],
                        input,
                        text,
                    )
                }
            }
            Self::Copyright => get_tooltip(
                "EXPORT_SETTINGS_PANEL_STATUS_TTS_COPYRIGHT",
                &[InputEvent::ToggleExportSettingBoolean],
                input,
                text,
            ),
        }
    }

    fn get_input_tts(&self, state: &State, input: &Input, text: &Text) -> String {
        match self {
            Self::Title => self.get_input_abc123_tts(
                "EXPORT_SETTINGS_PANEL_INPUT_TTS_TITLE_ABC123",
                "EXPORT_SETTINGS_PANEL_INPUT_TTS_TITLE_NO_ABC123",
                state,
                input,
                text,
            ),
            Self::Artist => self.get_input_abc123_tts(
                "EXPORT_SETTINGS_PANEL_INPUT_TTS_ARTIST_ABC123",
                "EXPORT_SETTINGS_PANEL_INPUT_TTS_ARTIST_NO_ABC123",
                state,
                input,
                text,
            ),
            Self::Copyright => get_tooltip(
                "EXPORT_SETTINGS_PANEL_INPUT_TTS_COPYRIGHT",
                &[InputEvent::ToggleExportSettingBoolean],
                input,
                text,
            ),
        }
    }

    fn get_input_abc123_tts(
        &self,
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
}
