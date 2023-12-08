use super::effect_type::EffectType;
use serde::{Deserialize, Serialize};

/// A hashable EffectType.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum ValuelessEffectType {
    #[default]
    Chorus,
    Reverb,
    Pan,
    PitchBend,
    ChannelPressure,
    PolyphonicKeyPressure,
}

impl From<EffectType> for ValuelessEffectType {
    fn from(value: EffectType) -> Self {
        match value {
            EffectType::Chorus(_) => Self::Chorus,
            EffectType::Reverb(_) => Self::Reverb,
            EffectType::Pan(_) => Self::Pan,
            EffectType::PitchBend(_) => Self::PitchBend,
            EffectType::ChannelPressure(_) => Self::ChannelPressure,
            EffectType::PolyphonicKeyPressure { key: _, value: _ } => Self::PolyphonicKeyPressure,
        }
    }
}

impl ValuelessEffectType {
    pub fn eq(&self, value: &EffectType) -> bool {
        match value {
            EffectType::Chorus(_) => *self == Self::Chorus,
            EffectType::Reverb(_) => *self == Self::Reverb,
            EffectType::Pan(_) => *self == Self::Pan,
            EffectType::PitchBend(_) => *self == Self::PitchBend,
            EffectType::ChannelPressure(_) => *self == Self::ChannelPressure,
            EffectType::PolyphonicKeyPressure { key: _, value: _ } => {
                *self == Self::PolyphonicKeyPressure
            }
        }
    }
}
