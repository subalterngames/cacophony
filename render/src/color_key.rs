use strum_macros::EnumString;

/// Enum values for color keys.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, EnumString)]
pub enum ColorKey {
    Background,
    NoFocus,
    FocusDefault,
    Key,
    Value,
    Yes,
    No,
    Arrow,
    TextFieldBG,
    Note,
    NoteSelected,
    NotePlaying,
    TimeCursor,
    TimeT0,
    Subtitle,
}
