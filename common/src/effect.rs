pub(crate) mod effect_type;
pub(crate) mod valueless_effect_type;
use serde::{Deserialize, Serialize};

use std::cmp::Ordering;

/// A timed synthesizer effect.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct Effect {
    /// The time of the event in PPQ.
    pub time: u64,
    /// The type of effect.
    pub effect: effect_type::EffectType,
}

impl Ord for Effect {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for Effect {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
