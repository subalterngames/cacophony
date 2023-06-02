use crate::note::SerializableNote;
use crate::{
    deserialize_fraction, serialize_fraction, Fraction, Index, Note, SerializableFraction, MAX_VOLUME
};
use serde::{Deserialize, Serialize};

/// Booleans and numerical values describing the input state.
#[derive(Clone)]
pub struct InputState {
    /// If true, we will accept musical input.
    pub armed: bool,
    /// If true, we're inputting an alphanumeric string and we should ignore certain key bindings.
    pub alphanumeric_input: bool,
    /// The volume for all new notes.
    pub volume: Index,
    /// If true, we'll use the volume value.
    pub use_volume: bool,
    /// The input beat.
    pub beat: Fraction,
    /// If true, we can undo and redo.
    pub can_undo: bool,
    /// A buffer of cut/copied notes.
    pub copied: Vec<Note>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            armed: false,
            alphanumeric_input: false,
            volume: Index::new(MAX_VOLUME as usize, MAX_VOLUME as usize + 1),
            use_volume: true,
            beat: Fraction::from(1u8),
            can_undo: true,
            copied: vec![]
        }
    }

    pub(crate) fn serialize(&self) -> SerializableInputState {
        SerializableInputState {
            armed: self.armed,
            alphanumeric_input: self.alphanumeric_input,
            volume: self.volume,
            use_volume: self.use_volume,
            beat: serialize_fraction(&self.beat),
            can_undo: self.can_undo,
            copied: self.copied.iter().map(|n| n.serialize()).collect(),
        }
    }
}

/// A serializable version of an `InputState`.
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct SerializableInputState {
    /// If true, we will accept musical input.
    pub armed: bool,
    /// If true, we're inputting an alphanumeric string and we should ignore certain key bindings.
    pub alphanumeric_input: bool,
    /// The volume for all new notes.
    pub volume: Index,
    /// If true, we'll use the volume value.
    pub use_volume: bool,
    /// If true, we can undo and redo.
    pub can_undo: bool,
    /// A buffer of cut/copied notes.
    pub copied: Vec<SerializableNote>,
    /// The input beat.
    pub beat: SerializableFraction,
}

impl SerializableInputState {
    /// Deserialize to a `Note`.
    pub(crate) fn deserialize(&self) -> InputState {
        InputState {
            armed: self.armed,
            alphanumeric_input: self.alphanumeric_input,
            volume: self.volume,
            use_volume: self.use_volume,
            beat: deserialize_fraction(&self.beat),
            can_undo: self.can_undo,
            copied: self.copied.iter().map(|n| n.deserialize()).collect(),
        }
    }
}
