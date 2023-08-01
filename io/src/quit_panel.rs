use crate::panel::*;
use common::PanelType;

/// Are you sure you want to quit?
#[derive(Default)]
pub(crate) struct QuitPanel {
    popup: Popup,
}

impl QuitPanel {
    pub fn enable(&mut self, state: &mut State) {
        self.popup.enable(state, vec![PanelType::Quit]);
    }
}

impl Panel for QuitPanel {
    fn update(
        &mut self,
        state: &mut State,
        _: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &mut Text,
        _: &mut PathsState,
        _: &mut SharedExporter,
    ) -> Option<Snapshot> {
        if input.happened(&InputEvent::QuitPanelYes) {
            Some(Snapshot::from_io_commands(vec![IOCommand::Quit]))
        } else {
            if input.happened(&InputEvent::InputTTS) {
                tts.enqueue(text.get_tooltip(
                    "QUIT_PANEL_INPUT_TTS",
                    &[InputEvent::QuitPanelNo, InputEvent::QuitPanelYes],
                    input,
                ));
            } else if input.happened(&InputEvent::QuitPanelNo) {
                self.popup.disable(state);
            }
            None
        }
    }

    fn allow_alphanumeric_input(&self, _: &State, _: &SharedExporter) -> bool {
        false
    }

    fn allow_play_music(&self) -> bool {
        false
    }

    fn on_disable_abc123(&mut self, _: &mut State, _: &mut SharedExporter) {}

    fn update_abc123(
        &mut self,
        _: &mut State,
        _: &Input,
        _: &mut SharedExporter,
    ) -> (Option<Snapshot>, bool) {
        (None, false)
    }
}
