use serde::{Deserialize, Serialize};

/// Types of synthesizer effects.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum EffectType {
    /// The degree, in 0.1% units, to which the audio output of the note is sent to the chorus effects processor.
    /// Must be between 0 and 1000.
    /// A value of 250 indicates that the signal is sent at 25% of full level to the chorus effects processor.
    /// Documentation source: http://www.synthfont.com/SFSPEC21.PDF
    Chorus(u16),
    /// The degree, in 0.1% units, to which the audio output of the note is sent to the reverb effects processor.
    /// Must be between 0 and 1000.
    /// A value of 250 indicates that the signal is sent at 25% of full level to the reverb effects processor.
    /// Documentation source: http://www.synthfont.com/SFSPEC21.PDF
    Reverb(u16),
    /// The degree, in 0.1% units, to which the "dry" audio output of the note is positioned to the left or right output.
    /// A value of 0 places the signal centered between left and right.
    /// A value of -500 indicates that the signal is at 100% of full level to the left output and 0% of full level to the right output.
    /// Documentation source: http://www.synthfont.com/SFSPEC21.PDF
    Pan(i16),
    /// The MIDI pitch bend. Must be between 0 and 16383.
    PitchBend(u16),
    /// The MIDI channel pressure. Must be between 0 and 127.
    ChannelPressure(u8),
    /// The MIDI key pressure (aftertouch). Both parameters must be between 0 and 127.
    PolyphonicKeyPressure { key: u8, value: u8 },
}

impl EffectType {
    pub fn increment(&mut self, up: bool) -> bool {
        match self {
            Self::Reverb(value) | Self::Chorus(value) => {
                if up {
                    if *value < 1000 {
                        *value += 1;
                        return true
                    }
                }
                else {
                    if *value > 0 {
                        *value -= 1;
                        return true
                    }
                }
            },
            Self::Pan(value) => {
                if up {
                    if *value < 500 {
                        *value += 1;
                        return true
                    }
                }
                else {
                    if *value > -500 {
                        *value -= 1;
                        return true
                    }
                }
            }
            Self::PitchBend(value) => {
                if up {
                    if *value < 16383 {
                        *value += 1;
                        return true
                    }
                }
                else {
                    if *value > 0 {
                        *value -= 1;
                        return true
                    }
                }
            }
            Self::ChannelPressure(value) | Self::PolyphonicKeyPressure { key: _, value } => {
                if up {
                    if *value < 127 {
                        *value += 1;
                        return true
                    }
                }
                else {
                    if *value > 0 {
                        *value -= 1;
                        return true
                    }
                }
            }
        } 
        false
    }
}
