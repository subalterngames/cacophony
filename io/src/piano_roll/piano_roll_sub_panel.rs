use crate::panel::*;
use common::{EditMode, EDIT_MODES};

/// A sub-panel (a mode) of the piano roll panel.
pub(crate) trait PianoRollSubPanel {
    /// Returns the status text-to-speech text.
    fn get_status_tts(&self, state: &State, text: &Text) -> String;

    /// Returns the input text-to-speech text.
    fn get_input_tts(&self, state: &State, input: &Input, text: &Text) -> String;
}

/// Returns the edit mode text-to-speech string.
pub(crate) fn get_edit_mode_status_tts(mode: &EditMode, text: &Text) -> String {
    text.get_with_values(
        "PIANO_ROLL_PANEL_STATUS_TTS_EDIT_MODE",
        &[&text.get_edit_mode(mode)],
    )
}

pub(crate) fn get_cycle_edit_mode_input_tts(mode: &Index, input: &Input, text: &Text) -> String {
    let mut m1 = *mode;
    m1.increment(true);
    let index = m1.get();
    get_tooltip_with_values(
        "PIANO_ROLL_PANEL_INPUT_TTS_EDIT_MODE",
        &[InputEvent::PianoRollCycleMode],
        &[&text.get_edit_mode(&EDIT_MODES[index])],
        input,
        text,
    )
}

/// Returns the text-to-speech string if no notes are selected.
pub(crate) fn get_no_selection_status_tts(text: &Text) -> String {
    text.get("PIANO_ROLL_PANEL_STATUS_TTS_NO_SELECTION")
}
