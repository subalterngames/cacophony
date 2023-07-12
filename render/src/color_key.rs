use strum_macros::EnumString;

/// Enum values for color keys.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, EnumString)]
pub enum ColorKey {
    Background,
    NoFocus,
    FocusDefault,
    Key,
    Value,
    True,
    False,
    Arrow,
    TextFieldBG,
    Note,
    NoteSelected,
    NotePlaying,
    TimeCursor,
    TimePlayback,
    Subtitle,
    Separator,
    TextInput,
    SelectedNotesBackground,
    Track0Focus,
    Track0NoFocus,
    Track1Focus,
    Track1NoFocus,
    Track2Focus,
    Track2NoFocus,
    Track3Focus,
    Track3NoFocus,
    Track4Focus,
    Track4NoFocus,
    Track5Focus,
    Track5NoFocus,
    SubtitleBackground
}
