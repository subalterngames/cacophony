use crate::{Effect, EffectType, Note, MAX_VOLUME, MAX_PITCH_BEND};
use serde::{Deserialize, Serialize};

/// A MIDI track has some notes.
#[derive(Debug, Deserialize, Serialize)]
pub struct MidiTrack {
    /// The channel used for audio synthesis.
    pub channel: u8,
    /// A gain value (0-127) for this track.
    pub gain: u8,
    /// The notes in the track.
    pub notes: Vec<Note>,
    /// Synthesizer and MIDI effects.
    #[serde(default = "Vec::new")]
    pub effects: Vec<Effect>,
    /// True if the track is muted.
    pub mute: bool,
    /// True if the track is soloed.
    pub solo: bool,
}

impl MidiTrack {
    pub fn new(channel: u8) -> Self {
        Self {
            channel,
            gain: MAX_VOLUME,
            notes: vec![],
            effects: vec![],
            mute: false,
            solo: false,
        }
    }

    /// Returns the end time of the track in PPQ.
    pub fn get_end(&self) -> Option<u64> {
        self.notes.iter().map(|n| n.end).max()
    }

    /// Returns the track gain as a float between 0 and 1.
    pub fn get_gain_f(&self) -> f32 {
        self.gain as f32 / MAX_VOLUME as f32
    }

    /// Returns all notes in the track that can be played (they are after t0).
    pub fn get_playback_notes(&self, start: u64) -> Vec<Note> {
        let gain = self.get_gain_f();
        let mut notes = vec![];
        for note in self.notes.iter().filter(|n| n.start >= start) {
            let mut n1 = *note;
            n1.velocity = (n1.velocity as f32 * gain) as u8;
            notes.push(n1);
        }
        notes.sort();
        notes
    }

    /// Get this track's *audio* effects. This will convert all pitch bends to a lerped sequence of events..
    pub fn get_audio_effects(&self) -> Vec<Effect> {
        let mut effects = vec![];
        for effect in self.effects.iter() {
            // Lerp the pitch bends.
            if let EffectType::PitchBend { value, duration} = effect.effect {
                // Prevent a divide by zero error.
                if duration == 0 {
                    continue;
                }
                let dv = (value as f64 / duration as f64) as u16;
                let mut pitch_value = 0;
                for t in 0..duration {
                    effects.push(Effect { time: effect.time + t, effect: EffectType::PitchBend { value: pitch_value, duration: 1 }});
                    pitch_value += dv;
                    if pitch_value > MAX_PITCH_BEND {
                        break;
                    }
                }
            }
            // Add the effect.
            else {
                effects.push(effect.clone());
            }
        }
        effects.sort();
        effects
    }
}

impl Clone for MidiTrack {
    fn clone(&self) -> Self {
        Self {
            channel: self.channel,
            gain: self.gain,
            notes: self.notes.clone(),
            effects: self.effects.clone(),
            mute: self.mute,
            solo: self.solo,
        }
    }
}
