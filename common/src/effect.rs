mod effect_type;
pub use effect_type::EffectType;
use serde::{Deserialize, Serialize};

use std::cmp::Ordering;

/// A Synthesizer effect.
#[derive(Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct Effect {
    /// The time of the event in PPQ.
    pub time: u64,
    /// The type of effect.
    pub effect: EffectType,
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
