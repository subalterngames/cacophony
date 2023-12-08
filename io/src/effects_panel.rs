use crate::panel::*;
use common::config::parse;
use common::{Effect, EffectType, ValuelessEffectType, MAX_NOTE, MIDDLE_C, MIN_NOTE};
use ini::Ini;
use text::EFFECT_NAME_KEYS;

/// Add, remove, or adjust effects.
pub(crate) struct EffectsPanel {
    /// The sensitivity of pitch bend input.
    pitch_bend_sensitivity: usize,
    /// Text tooltips.
    tooltips: Tooltips,
}

impl EffectsPanel {
    pub fn new(config: &Ini) -> Self {
        let section = config.section(Some("EFFECTS")).unwrap();
        let pitch_bend_sensitivity = parse(section, "pitch_bend_sensitivity");
        Self {
            pitch_bend_sensitivity,
            tooltips: Tooltips::default(),
        }
    }

    /// Increment or decrement the effect type index.
    fn cycle_effect_type(state: &mut State, up: bool) -> Option<Snapshot> {
        let s0 = state.clone();
        state.effect_types.index.increment(up);
        Some(Snapshot::from_states(s0, state))
    }

    /// Increment the `value` of an effect.
    /// For `PitchBend`, a delta is applied: `self.pitch_bend_sensitivity`.
    /// For `PolyphonicKeyPressure`, the `value` is incremented.
    fn increment_effect_value(state: &mut State, up: bool) -> Option<Snapshot> {
        let s0 = state.clone();
        match Self::get_effect(state) {
            Some(effect) => {
                // Increment by an extra delta.
                if let EffectType::PitchBend(_) = effect.effect {
                    let mut incremented = false;
                    for i in 0..10 {
                        if !effect.effect.increment(up) {
                            if i > 0 {
                                incremented = true;
                            }
                            break;
                        }
                    }
                    if incremented {
                        Some(Snapshot::from_states(s0, state))
                    } else {
                        None
                    }
                }
                // Increment by one.
                else {
                    if effect.effect.increment(up) {
                        Some(Snapshot::from_states(s0, state))
                    } else {
                        None
                    }
                }
            }
            None => None,
        }
    }

    /// Increment the `key` of a `PolyphonicKeyPressure` effect.
    fn increment_aftertouch(state: &mut State, up: bool) -> Option<Snapshot> {
        let s0 = state.clone();
        match Self::get_effect(state) {
            Some(effect) => {
                if let EffectType::PolyphonicKeyPressure { key, value } = effect.effect {
                    if up {
                        if key < MAX_NOTE {
                            effect.effect = EffectType::PolyphonicKeyPressure {
                                key: key + 1,
                                value,
                            };
                            Some(Snapshot::from_states(s0, state))
                        } else {
                            None
                        }
                    } else {
                        if key > MIN_NOTE {
                            effect.effect = EffectType::PolyphonicKeyPressure {
                                key: key - 1,
                                value,
                            };
                            Some(Snapshot::from_states(s0, state))
                        } else {
                            None
                        }
                    }
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// Add a new effect.
    fn add_effect(&self, state: &mut State, conn: &Conn) -> Option<Snapshot> {
        let ve = state.effect_types.get();
        match Self::get_effect(state) {
            // There is already an effect.
            Some(_) => None,
            // Try to add an effect.
            None => {
                let s0 = state.clone();
                match state.music.get_selected_track_mut() {
                    Some(track) => {
                        let program = &conn.state.programs[&track.channel];
                        // Get a new effect type.
                        let effect_type = match ve {
                            ValuelessEffectType::Chorus => {
                                EffectType::Chorus(program.chorus as u16)
                            }
                            ValuelessEffectType::Pan => EffectType::Pan(program.pan as i16),
                            ValuelessEffectType::Reverb => {
                                EffectType::Reverb(program.reverb as u16)
                            }
                            ValuelessEffectType::PitchBend => EffectType::PitchBend(0),
                            ValuelessEffectType::ChannelPressure => EffectType::ChannelPressure(0),
                            ValuelessEffectType::PolyphonicKeyPressure => {
                                EffectType::PolyphonicKeyPressure {
                                    key: MIDDLE_C,
                                    value: 0,
                                }
                            }
                        };
                        // Get a new effect.
                        track.effects.push(Effect {
                            time: state.time.cursor,
                            effect: effect_type,
                        });
                        Some(Snapshot::from_states(s0, state))
                    }
                    None => None,
                }
            }
        }
    }

    /// Remove an existing effect.
    fn remove_effect(state: &mut State) -> Option<Snapshot> {
        let s0 = state.clone();
        match Self::get_effect_copy(state) {
            Some(effect) => {
                state
                    .music
                    .get_selected_track_mut()
                    .unwrap()
                    .effects
                    .retain(|e| *e != effect);
                Some(Snapshot::from_states(s0, state))
            }
            None => None,
        }
    }

    /// Set the time of an existing effect.
    fn set_time(state: &mut State, up: bool) -> Option<Snapshot> {
        let s0 = state.clone();
        let beat = state.input.beat.get_u();
        match Self::get_effect(state) {
            Some(effect) => {
                // Increase the time.
                if up {
                    effect.time += beat;
                    Some(Snapshot::from_states(s0, state))
                } else {
                    match effect.time.checked_sub(beat) {
                        Some(time) => {
                            effect.time = time;
                            Some(Snapshot::from_states(s0, state))
                        }
                        None => None,
                    }
                }
            }
            None => None,
        }
    }

    /// Get a copied of the selected effect.
    fn get_effect_copy(state: &State) -> Option<Effect> {
        let ve = state.effect_types.get();
        match state.music.get_selected_track() {
            Some(track) => track
                .effects
                .iter()
                .filter(|e| e.time == state.time.cursor && ve.eq(&e.effect))
                .copied()
                .next(),
            None => None,
        }
    }

    /// Get the selected effect.
    fn get_effect(state: &mut State) -> Option<&mut Effect> {
        let ve = state.effect_types.get();
        match state.music.get_selected_track_mut() {
            Some(track) => track
                .effects
                .iter_mut()
                .filter(|e| e.time == state.time.cursor && ve.eq(&e.effect))
                .next(),
            None => None,
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
        if input.happened(&InputEvent::InputTTS) {
            let mut tts_strings = vec![self.tooltips.get_tooltip(
                "EFFECTS_PANEL_INPUT_TTS_SCROLL",
                &[InputEvent::PreviousEffect, InputEvent::NextTrack],
                input,
                text,
            )];
            // Add a new effect.
            if Self::get_effect(state).is_some() {
                tts_strings.push(TtsString::from(self.tooltips.get_tooltip_with_values(
                    "EFFECTS_PANEL_INPUT_TTS_ADD",
                    &[InputEvent::AddTrack],
                    &[text.get_ref(EFFECT_NAME_KEYS[state.effect_types.index.get()])],
                    input,
                    text,
                )))
            }
            // Adjust an effect.
            else {
                tts_strings.push(TtsString::from(self.tooltips.get_tooltip(
                    "EFFECTS_PANE_INPUT_TTS_VALUE",
                    &[
                        InputEvent::IncrementEffectValue,
                        InputEvent::DecrementEffectValue,
                    ],
                    input,
                    text,
                )));
                tts_strings.push(TtsString::from(self.tooltips.get_tooltip(
                    "EFFECTS_PANEL_INPUT_TTS_TIME",
                    &[InputEvent::EffectTimeLeft, InputEvent::EffectTimeRight],
                    input,
                    text,
                )));
                if let ValuelessEffectType::PolyphonicKeyPressure = state.effect_types.get() {
                    tts_strings.push(TtsString::from(self.tooltips.get_tooltip(
                        "EFFECTS_PANEL_INPUT_TTS_AFTERTOUCH",
                        &[
                            InputEvent::IncrementAftertouchNote,
                            InputEvent::DecrementAftertouchNote,
                        ],
                        input,
                        text,
                    )));
                }
                tts_strings.push(TtsString::from(self.tooltips.get_tooltip_with_values(
                    "EFFECTS_PANEL_INPUT_TTS_REMOVE",
                    &[InputEvent::RemoveEffect],
                    &[text.get_ref(EFFECT_NAME_KEYS[state.effect_types.index.get()])],
                    input,
                    text,
                )));
            }
            tts.enqueue(tts_strings);
            None
        } else if input.happened(&InputEvent::StatusTTS) {
            let effect_name = text.get_ref(EFFECT_NAME_KEYS[state.effect_types.index.get()]);
            let mut tts_strings = vec![];
            // The name of the selected effect.
            match Self::get_effect(state) {
                Some(effect) => {
                    let value = match effect.effect {
                        EffectType::Chorus(value)
                        | EffectType::Reverb(value)
                        | EffectType::PitchBend(value) => value.to_string(),
                        EffectType::Pan(value) => value.to_string(),
                        EffectType::ChannelPressure(value)
                        | EffectType::PolyphonicKeyPressure { key: _, value } => value.to_string(),
                    };
                    tts_strings.push(TtsString::from(text.get_with_values(
                        "EFFECTS_PANEL_STATUS_TTS_EFFECT_NAME",
                        &[effect_name, &value],
                    )));
                    if let EffectType::PolyphonicKeyPressure { key, value: _ } = effect.effect {
                        tts_strings.push(TtsString::from(text.get_with_values(
                            "EFFECTS_PANEL_STATUS_TTS_AFTERTOUCH_KEY",
                            &[&key.to_string()],
                        )));
                    }
                }
                None => {
                    tts_strings.push(TtsString::from(
                        text.get_with_values("EFFECTS_PANEL_STATUS_TTS_NO_EFFECT", &[effect_name]),
                    ));
                }
            }
            tts.enqueue(tts_strings);
            None
        }
        // Cycle the selected input event.
        else if input.happened(&InputEvent::NextEffect) {
            Self::cycle_effect_type(state, true)
        } else if input.happened(&InputEvent::PreviousEffect) {
            Self::cycle_effect_type(state, false)
        } else if input.happened(&InputEvent::EffectTimeRight) {
            Self::set_time(state, true)
        } else if input.happened(&InputEvent::EffectTimeLeft) {
            Self::set_time(state, false)
        } else if input.happened(&InputEvent::IncrementEffectValue) {
            Self::increment_effect_value(state, true)
        } else if input.happened(&InputEvent::DecrementEffectValue) {
            Self::increment_effect_value(state, false)
        } else if input.happened(&InputEvent::IncrementAftertouchNote) {
            Self::increment_aftertouch(state, true)
        } else if input.happened(&InputEvent::DecrementAftertouchNote) {
            Self::increment_aftertouch(state, false)
        } else if input.happened(&InputEvent::AddEffect) {
            Self::add_effect(&self, state, conn)
        } else if input.happened(&InputEvent::RemoveEffect) {
            Self::remove_effect(state)
        } else {
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
