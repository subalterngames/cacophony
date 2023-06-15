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
    Track0,
    Track1,
    Track2,
    Track3,
    Track4,
    Track5,
    Track6,
}
