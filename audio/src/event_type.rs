use common::effect::EffectType;
use oxisynth::{GeneratorType, MidiEvent, Synth};

/// A type of MIDI or synthesizer event.
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum EventType {
    NoteOn { channel: u8, key: u8, vel: u8 },
    NoteOff { channel: u8, key: u8 },
    Effect { channel: u8, effect: EffectType },
}

impl EventType {
    pub(crate) fn occur(&self, synth: &mut Synth) {
        match self {
            EventType::NoteOn { channel, key, vel } => {
                let _ = synth.send_event(MidiEvent::NoteOn {
                    channel: *channel,
                    key: *key,
                    vel: *vel,
                });
            }
            EventType::NoteOff { channel, key } => {
                let _ = synth.send_event(MidiEvent::NoteOff {
                    channel: *channel,
                    key: *key,
                });
            }
            EventType::Effect { channel, effect } => match effect {
                EffectType::PitchBend(value) => {
                    let _ = synth.send_event(MidiEvent::PitchBend {
                        channel: *channel,
                        value: *value,
                    });
                }
                EffectType::ChannelPressure(value) => {
                    let _ = synth.send_event(MidiEvent::ChannelPressure {
                        channel: *channel,
                        value: *value,
                    });
                }
                EffectType::PolyphonicKeyPressure { key, value } => {
                    let _ = synth.send_event(MidiEvent::PolyphonicKeyPressure {
                        channel: *channel,
                        key: *key,
                        value: *value,
                    });
                }
                EffectType::Chorus(value) => {
                    let _ =
                        synth.set_gen(*channel as usize, GeneratorType::ChorusSend, *value as f32);
                }
                EffectType::Reverb(value) => {
                    let _ =
                        synth.set_gen(*channel as usize, GeneratorType::ReverbSend, *value as f32);
                }
                EffectType::Pan(value) => {
                    let _ = synth.set_gen(*channel as usize, GeneratorType::Pan, *value as f32);
                }
            },
        }
    }
}
