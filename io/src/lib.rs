use common::hashbrown::HashMap;
use common::{PanelType, State};
use input::{Input, InputEvent};
use text::Text;
mod tooltip;
pub(crate) use tooltip::get_tooltip;

/// The maximum size of the undo stack.
const MAX_UNDOS: usize = 100;

#[derive(Default)]
pub struct IO {
    /// A stack of states that can be popped to undo an action.
    undo: Vec<State>,
    /// A stack of states that can be popped to redo an action.
    redo: Vec<State>,
    /// Top-level text-to-speech lookups.
    tts: HashMap<InputEvent, String>,
}

impl IO {
    pub fn new(input: &Input, text: &Text) -> Self {
        let mut tts = HashMap::new();
        // App TTS.
        let app = get_tooltip(
            "APP_TTS",
            &[
                InputEvent::PanelTTS,
                InputEvent::WidgetTTS,
                InputEvent::AppTTS,
                InputEvent::FileTTS,
                InputEvent::ConfigTTS,
                InputEvent::Quit,
                InputEvent::NextPanel,
                InputEvent::PreviousPanel,
                InputEvent::Undo,
                InputEvent::Redo,
                InputEvent::StopTTS,
            ],
            input,
            text,
        );
        tts.insert(InputEvent::AppTTS, app);
        // File TTS.
        let file = get_tooltip(
            "FILE_TTS",
            &[
                InputEvent::NewFile,
                InputEvent::OpenFile,
                InputEvent::SaveFile,
                InputEvent::SaveFileAs,
                InputEvent::ExportFile,
            ],
            input,
            text,
        );
        tts.insert(InputEvent::FileTTS, file);
        // Config TTS.
        let config = get_tooltip(
            "CONFIG_TTS",
            &[InputEvent::EditConfig, InputEvent::OverwriteConfig],
            input,
            text,
        );
        tts.insert(InputEvent::ConfigTTS, config);
        Self {
            tts,
            ..Default::default()
        }
    }

    pub fn update(&mut self, state: &mut State, input: &mut Input, text: &Text) {
        // Update the input state.
        input.update(state);

        // Undo.
        if input.happened(&InputEvent::Undo) && !self.undo.is_empty() {
            // Pop s1.
            let s1 = self.undo.remove(0);
            // Push s1 to the redo stack.
            self.redo.push(s1.clone());
            // Assign s1 to state.
            *state = s1;
        // Redo.
        } else if input.happened(&InputEvent::Redo) && !self.redo.is_empty() {
            // Pop s1.
            let s1 = self.redo.remove(0);
            // Push s1 to the undo stack.
            self.undo.push(s1.clone());
            // Assign s1 to state.
            *state = s1;
        }

        // Cycle panels.
        if input.happened(&InputEvent::NextPanel) {
            state.focus.increment(true);
            self.push_undo(state);
        } else if input.happened(&InputEvent::PreviousPanel) {
            state.focus.increment(false);
            self.push_undo(state);
        }
    }

    /// Push this state to the undo stack and clear the redo stack.
    fn push_undo(&mut self, state: &State) {
        self.undo.push(state.clone());
        self.redo.clear();
        // Remove an undo if there are too many.
        if self.undo.len() > MAX_UNDOS {
            self.undo.remove(0);
        }
    }
}
