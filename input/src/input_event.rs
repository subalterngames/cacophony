use strum_macros::EnumString;

/// Input events from either a qwerty keyboard or a MIDI controller.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, EnumString)]
pub enum InputEvent {
    // Cycle panels.
    NextPanel,
    PreviousPanel,
    // Alphanumeric input.
    ToggleAlphanumericInput,
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
    PianoRollToggleTracks,
    Arm,
    InputBeatLeft,
    InputBeatRight,
    IncreaseInputVolume,
    DecreaseInputVolume,
    ToggleInputVolume,
    PlayStop,
    PianoRollPreviousTrack,
    PianoRollNextTrack,
    // Piano roll - view mode.
    ViewLeft,
    ViewRight,
    ViewUp,
    ViewDown,
    ViewStart,
    ViewEnd,
    ViewZoomIn,
    ViewZoomOut,
    ViewZoomDefault,
    // Piano roll - time mode.
    TimeCursorLeft,
    TimeCursorRight,
    TimeCursorStart,
    TimeCursorEnd,
    TimePlaybackLeft,
    TimePlaybackRight,
    TimePlaybackStart,
    TimePlaybackEnd,
    TimeCursorPlayback,
    TimePlaybackCursor,
    TimeCursorBeat,
    TimePlaybackBeat,
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
    // Qwerty note input.
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
    OctaveUp,
    OctaveDown,
}
