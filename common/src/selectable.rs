use crate::{Effect, Note};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Selectable {
    Effect(Effect),
    Note(Note),
}

impl Selectable {
    pub fn get_start_time(&self) -> u64 {
        match self {
            Self::Effect(effect) => effect.time,
            Self::Note(note) => note.start,
        }
    }

    pub fn get_end_time(&self) -> u64 {
        match self {
            Self::Effect(effect) => effect.time,
            Self::Note(note) => note.end,
        }
    }
}
