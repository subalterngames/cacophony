use crate::{Effect, Note};

/// A note or an effect.
#[derive(Clone)]
pub enum Event<'track> {
    Effect {
        effect: &'track Effect,
        index: usize,
    },
    Note {
        note: &'track Note,
        index: usize,
    },
}

impl<'track> Event<'track> {
    pub fn get_start_time(&self) -> u64 {
        match self {
            Self::Effect { effect, index: _ } => effect.time,
            Self::Note { note, index: _ } => note.start,
        }
    }

    pub fn get_end_time(&self) -> u64 {
        match self {
            Self::Effect { effect, index: _ } => effect.time,
            Self::Note { note, index: _ } => note.end,
        }
    }
}
