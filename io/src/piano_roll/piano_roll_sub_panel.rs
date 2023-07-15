use crate::panel::*;
use common::{EditMode, IndexedEditModes};

/// A sub-panel (a mode) of the piano roll panel.
pub(crate) trait PianoRollSubPanel {
    /// Returns the status text-to-speech text.
    fn get_status_tts(&mut self, state: &State, text: &Text) -> TtsString;

    /// Returns the input text-to-speech text.
    fn get_input_tts(&mut self, state: &State, input: &Input, text: &Text) -> TtsString;
}

/// Returns the edit mode text-to-speech string.
pub(crate) fn get_edit_mode_status_tts(mode: &EditMode, text: &Text) -> TtsString {
    TtsString::from(text.get_with_values(
        "PIANO_ROLL_PANEL_STATUS_TTS_EDIT_MODE",
        &[&text.get_edit_mode(mode)],
    ))
}

pub(crate) fn get_cycle_edit_mode_input_tts(
    mode: &IndexedEditModes,
    input: &Input,
    text: &Text,
    tooltips: &mut Tooltips
) -> TtsString {
    let mut m1 = *mode;
    m1.index.increment(true);
    tooltips.get_tooltip_with_values(
        "PIANO_ROLL_PANEL_INPUT_TTS_EDIT_MODE",
        &[InputEvent::PianoRollCycleMode],
        &[&text.get_edit_mode(m1.get_ref())],
        input,
        text,
    )
}

/// Returns the text-to-speech string if no notes are selected.
pub(crate) fn get_no_selection_status_tts(text: &Text) -> TtsString {
    TtsString::from(text.get("PIANO_ROLL_PANEL_STATUS_TTS_NO_SELECTION"))
}
