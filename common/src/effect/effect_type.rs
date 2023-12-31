use crate::MIDDLE_C;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use strum::EnumCount;
use strum_macros::EnumCount as EnumCountMacro;

pub const MAX_PITCH_BEND: u16 = 16383;

/// Types of synthesizer effects.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize, Serialize, EnumCountMacro)]
#[repr(u8)]
pub enum EffectType {
    /// The degree, in 0.1% units, to which the audio output of the note is sent to the chorus effects processor.
    /// Must be between 0 and 1000.
    /// A value of 250 indicates that the signal is sent at 25% of full level to the chorus effects processor.
    /// Documentation source: http://www.synthfont.com/SFSPEC21.PDF
    Chorus(u16) = 0,
    /// The degree, in 0.1% units, to which the audio output of the note is sent to the reverb effects processor.
    /// Must be between 0 and 1000.
    /// A value of 250 indicates that the signal is sent at 25% of full level to the reverb effects processor.
    /// Documentation source: http://www.synthfont.com/SFSPEC21.PDF
    Reverb(u16) = 1,
    /// The degree, in 0.1% units, to which the "dry" audio output of the note is positioned to the left or right output.
    /// A value of 0 places the signal centered between left and right.
    /// A value of -500 indicates that the signal is at 100% of full level to the left output and 0% of full level to the right output.
    /// Documentation source: http://www.synthfont.com/SFSPEC21.PDF
    Pan(i16) = 2,
    /// The MIDI pitch bend. Must be between 0 and 16383.
    PitchBend { value: u16, duration: u64 } = 3,
    /// The MIDI channel pressure. Must be between 0 and 127.
    ChannelPressure(u8) = 4,
    /// The MIDI key pressure (aftertouch). Both parameters must be between 0 and 127.
    PolyphonicKeyPressure { key: u8, value: u8 } = 5,
}

impl EffectType {
    /// Returns an array of each effect type, with each field set to a default value.
    pub fn get_array() -> [Self; Self::COUNT] {
        [
            EffectType::Chorus(500),
            EffectType::Reverb(500),
            EffectType::Pan(0),
            EffectType::PitchBend {
                value: 0,
                duration: 0,
            },
            EffectType::ChannelPressure(0),
            EffectType::PolyphonicKeyPressure {
                key: MIDDLE_C,
                value: 0,
            },
        ]
    }

    pub fn increment(&mut self, up: bool) -> bool {
        match self {
            Self::Reverb(value) | Self::Chorus(value) => {
                if up {
                    if *value < 1000 {
                        *value += 1;
                        return true;
                    }
                } else if *value > 0 {
                    *value -= 1;
                    return true;
                }
            }
            Self::Pan(value) => {
                if up {
                    if *value < 500 {
                        *value += 1;
                        return true;
                    }
                } else if *value > -500 {
                    *value -= 1;
                    return true;
                }
            }
            Self::PitchBend { value, duration: _ } => {
                if up {
                    if *value < MAX_PITCH_BEND {
                        *value += 1;
                        return true;
                    }
                } else if *value > 0 {
                    *value -= 1;
                    return true;
                }
            }
            Self::ChannelPressure(value) | Self::PolyphonicKeyPressure { key: _, value } => {
                if up {
                    if *value < 127 {
                        *value += 1;
                        return true;
                    }
                } else if *value > 0 {
                    *value -= 1;
                    return true;
                }
            }
        }
        false
    }

    /// Returns true if the enum variants are equal, irrespective of the value(s) of each enum's fields.
    pub fn valueless_eq(&self, other: &Self) -> bool {
        self.get_ordinal() == other.get_ordinal()
    }

    /// Source: https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=21e3ab42f76ccbc05b6b61560cbd29ec
    pub fn get_ordinal(&self) -> u8 {
        let ptr_to_option = (self as *const Self) as *const u8;
        unsafe { *ptr_to_option }
    }

    pub fn get_value_string(&self) -> String {
        match self {
            Self::Chorus(value) | Self::Reverb(value) | Self::PitchBend { value, duration: _ } => {
                value.to_string()
            }
            Self::Pan(value) => value.to_string(),
            Self::ChannelPressure(value) | Self::PolyphonicKeyPressure { key: _, value } => {
                value.to_string()
            }
        }
    }

    pub fn get_secondary_value(&self) -> Option<String> {
        match self {
            Self::PitchBend { value: _, duration } => Some(duration.to_string()),
            Self::PolyphonicKeyPressure { key, value: _ } => Some(key.to_string()),
            _ => None,
        }
    }
}

impl Default for EffectType {
    fn default() -> Self {
        Self::Chorus(500)
    }
}

impl Hash for EffectType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_ordinal().hash(state)
    }
}
