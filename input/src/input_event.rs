use strum_macros::EnumString;

/// Input events from either a qwerty keyboard or a MIDI controller.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, EnumString)]
pub enum InputEvent {
    // Cycle panels.
    NextPanel,
    PreviousPanel,
    // TTS.
    PanelTTS,
    SubPanelTTS,
    AppTTS,
    FileTTS,
    ConfigTTS,
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
    OverwriteConfig,
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
    // Piano roll - view mode.
    ViewLeft,
    ViewRight,
    ViewUp,
    ViewDown,
    ViewStart,
    ViewEnd,
}
