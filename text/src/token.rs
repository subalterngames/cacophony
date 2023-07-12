/// A token of a string. This is used for rendering.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Word(String),
    MIDI(String),
    Qwerty(String)
}