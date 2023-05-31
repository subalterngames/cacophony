use strum_macros::EnumString;

/// Input events from either a qwerty keyboard or a MIDI controller.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, EnumString)]
pub enum InputEvent {
    // Cycle panels.
    NextPanel,
    PreviousPanel,
    // TTS.
    StatusTTS,
    InputTTS,
    AppTTS,
    FileTTS,
    StopTTS,
    // Undo-redo.
    Undo,
    Redo,
    // Files.
    OpenFile,
    NewFile,
    SaveFile,
    SaveFileAs,
    ExportFile,
    EditConfig,
    // Quit.
    Quit,
    // Music panel.
    NextMusicPanelField,
    PreviousMusicPanelField,
    IncreaseMusicGain,
    DecreaseMusicGain,
    // Tracks panel.
    AddTrack,
    RemoveTrack,
    NextTrack,
    PreviousTrack,
    EnableSoundFontPanel,
    PreviousPreset,
    NextPreset,
    PreviousBank,
    NextBank,
    IncreaseTrackGain,
    DecreaseTrackGain,
    Mute,
    Solo,
    // Open file panel.
    UpDirectory,
    DownDirectory,
    SelectFile,
    NextPath,
    PreviousPath,
    CloseOpenFile,
    // Piano roll.
    PianoRollCycleMode,
    PianoRollSetTime,
    PianoRollSetView,
    PianoRollSetSelect,
    PianoRollSetEdit,
    Arm,
    InputBeatLeft,
    InputBeatRight,
    IncreaseInputVolume,
    DecreaseInputVolume,
    ToggleInputVolume,
    PlayStop,
    // Piano roll - view mode.
    ViewLeft,
    ViewRight,
    ViewUp,
    ViewDown,
    ViewStart,
    ViewEnd,
    // Piano roll - time mode.
    TimeCursorLeft,
    TimeCursorRight,
    TimeCursorStart,
    TimeCursorEnd,
    TimePlaybackLeft,
    TimePlaybackRight,
    TimePlaybackStart,
    TimePlaybackEnd,
    // Piano roll - edit mode.
    EditStartLeft,
    EditStartRight,
    EditDurationLeft,
    EditDurationRight,
    EditPitchUp,
    EditPitchDown,
    EditVolumeUp,
    EditVolumeDown,
    // Piano roll - select mode.
    SelectStartLeft,
    SelectStartRight,
    SelectEndLeft,
    SelectEndRight,
    SelectAll,
    SelectNone,
    // Copy, cut, paste, delete.
    CopyNotes,
    CutNotes,
    PasteNotes,
    DeleteNotes,
}
