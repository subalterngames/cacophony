use crate::panel::*;
use common::{EffectType, Effect, ValuelessEffectType, MIDDLE_C};
use text::EFFECT_NAME_KEYS;

#[derive(Default)]
pub(crate) struct EffectsPanel {
}

impl EffectsPanel {
    /// Increment or decrement the effect type index.
    fn cycle_effect_type(state: &mut State, up: bool) -> Option<Snapshot> {
        let s0 = state.clone();
        state.effect_types.index.increment(up);
        Some(Snapshot::from_states(s0, state))
    }

    fn increment_effect_value(state: &mut State, conn: &Conn, up: bool) -> Option<Snapshot> {
        let s0 = state.clone();
        let ve = state.effect_types.get();
        match state.music.get_selected_track_mut() {
            Some(track) => match track.effects.iter_mut().filter(|e| e.time == state.time.cursor && ve.eq(&e.effect)).next() {
                Some(effect) => {
                    if (up && effect.effect.increment()) || (!up && effect.effect.decrement()) {
                        Some(Snapshot::from_states(s0, state))
                    }
                    else {
                        None
                    }
                }
                // Add a new effect.
                None => {
                    let program = &conn.state.programs[&track.channel];
                    // Get a new effect type.
                    let effect_type = match ve {
                        ValuelessEffectType::Chorus => EffectType::Chorus(program.chorus as u16),
                        ValuelessEffectType::Pan => EffectType::Pan(program.pan as i16),
                        ValuelessEffectType::Reverb => EffectType::Reverb(program.reverb as u16),
                        ValuelessEffectType::PitchBend => EffectType::PitchBend(0),
                        ValuelessEffectType::ChannelPressure => EffectType::ChannelPressure(0),
                        ValuelessEffectType::PolyphonicKeyPressure => EffectType::PolyphonicKeyPressure { key: MIDDLE_C, value: 0 }
                    };
                    // Get a new effect.
                    track.effects.push(Effect { time: state.time.cursor, effect: effect_type });
                    Some(Snapshot::from_states(s0, state))
                }
            }
            None => None
        }
    }
}

impl Panel for EffectsPanel {
    fn update(
            &mut self,
            state: &mut State,
            conn: &mut Conn,
            input: &Input,
            tts: &mut TTS,
            text: &Text,
            _: &mut PathsState,
        ) -> Option<Snapshot> {
        // Cycle the selected input event.
        if input.happened(&InputEvent::NextEffect) {
            Self::cycle_effect_type(state, true)
        }
        else if input.happened(&InputEvent::PreviousEffect) {
            Self::cycle_effect_type(state, false)
        }
        else if input.happened(&InputEvent::IncrementEffectValue) {
            Self::increment_effect_value(state, conn, true)
        }
        else if input.happened(&InputEvent::DecrementEffectValue) {
            Self::increment_effect_value(state, conn, false)
        }
        else if input.happened(&InputEvent::DeleteEffect) {
            let s0 = state.clone();
            let ve = state.effect_types.get();
            match state.music.get_selected_track_mut() {
                Some(track) => {
                    let has_effect = track.effects.iter().filter(|e| e.time == state.time.cursor && ve.eq(&e.effect)).next().is_some();
                    track.effects.retain(|e| e.time != state.time.cursor || !ve.eq(&e.effect));
                    if has_effect {
                        Some(Snapshot::from_states(s0, state))
                    }
                    else {
                        None
                    }
                }
                None => None
            }
        }
        else {
            None
        }
    }

    fn on_disable_abc123(&mut self, _: &mut State, _: &mut Conn) {}

    fn update_abc123(
        &mut self,
        _: &mut State,
        _: &Input,
        _: &mut Conn,
    ) -> (Option<Snapshot>, bool) {
        (None, false)
    }

    fn allow_alphanumeric_input(&self, _: &State, _: &Conn) -> bool {
        false
    }

    fn allow_play_music(&self) -> bool {
        true
    }
}