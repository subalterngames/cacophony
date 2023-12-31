pub(crate) mod effect_type;
use serde::{Deserialize, Serialize};

use std::cmp::Ordering;

use crate::EffectType;

/// A timed synthesizer effect.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct Effect {
    /// The time of the event in PPQ.
    pub time: u64,
    /// The type of effect.
    pub effect: effect_type::EffectType,
}

impl Effect {
    /// Returns true if the effect is at, or is ongoing during, time `time` (in PPQ).
    pub fn at_time(&self, time: u64) -> bool {
        match &self.effect {
            EffectType::PitchBend { value: _, duration } => {
                self.time == time || (self.time < time && self.time + duration >= time)
            }
            _ => self.time == time,
        }
    }
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
