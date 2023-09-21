use crate::panel::*;
use common::PanelType;
use hashbrown::HashMap;
use webbrowser::open;

/// Open a link in a browser.
pub(crate) struct LinksPanel {
    /// URLs per input event.
    links: HashMap<InputEvent, String>,
    /// The popup.
    popup: Popup,
    /// The tooltips handler.
    tooltips: Tooltips,
}

impl LinksPanel {
    pub fn enable(&mut self, state: &mut State) {
        self.popup.enable(state, vec![PanelType::Links]);
    }
}

impl Default for LinksPanel {
    fn default() -> Self {
        let mut links = HashMap::new();
        links.insert(
            InputEvent::WebsiteUrl,
            "https://subalterngames.com/cacophony".to_string(),
        );
        links.insert(
            InputEvent::DiscordUrl,
            "https://discord.gg/fUapDXgTYj".to_string(),
        );
        links.insert(
            InputEvent::GitHubUrl,
            "https://github.com/subalterngames/cacophony".to_string(),
        );
        let popup = Popup::default();
        let tooltips = Tooltips::default();
        Self {
            links,
            popup,
            tooltips,
        }
    }
}

impl Panel for LinksPanel {
    fn update(
        &mut self,
        state: &mut State,
        _: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &Text,
        _: &mut PathsState,
        _: &mut SharedExporter,
    ) -> Option<Snapshot> {
        if input.happened(&InputEvent::InputTTS) {
            tts.enqueue(TtsString::from(text.get("LINKS_PANEL_INPUT_TTS_0")));
            tts.enqueue(self.tooltips.get_tooltip(
                "LINKS_PANEL_INPUT_TTS_1",
                &[InputEvent::WebsiteUrl],
                input,
                text,
            ));
            tts.enqueue(self.tooltips.get_tooltip(
                "LINKS_PANEL_INPUT_TTS_2",
                &[InputEvent::DiscordUrl],
                input,
                text,
            ));
            tts.enqueue(self.tooltips.get_tooltip(
                "LINKS_PANEL_INPUT_TTS_2",
                &[InputEvent::GitHubUrl],
                input,
                text,
            ));
            tts.enqueue(self.tooltips.get_tooltip(
                "LINKS_PANEL_INPUT_TTS_3",
                &[InputEvent::CloseLinksPanel],
                input,
                text,
            ));
        } else {
            // Try to open links.
            for (input_event, url) in self.links.iter() {
                if input.happened(input_event) && open(url).is_ok() {
                    return None;
                }
            }
            // Disable the popup.
            if input.happened(&InputEvent::CloseLinksPanel) {
                self.popup.disable(state);
            }
        }
        None
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
