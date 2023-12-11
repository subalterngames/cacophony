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

    /// Increment the `value` of an effect. Add the effect if it doesn't exist.
    /// For `PitchBend`, a delta is applied: `self.pitch_bend_sensitivity`.
    /// For `PolyphonicKeyPressure`, the `value` is incremented.
    fn increment_effect_value(&self, state: &mut State, conn: &Conn, up: bool) -> Option<Snapshot> {
        let s0 = state.clone();
        match Self::get_effect(state) {
            Some(effect) => {
                // Increment by an extra delta.
                if let EffectType::PitchBend(_) = effect.effect {
                    let mut incremented = false;
                    for i in 0..self.pitch_bend_sensitivity {
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
            None => self.add_effect(state, conn),
        }
    }

    /// Increment the `key` of a `PolyphonicKeyPressure` effect.
    fn increment_aftertouch(&self, state: &mut State, conn: &Conn, up: bool) -> Option<Snapshot> {
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
            None => self.add_effect(state, conn),
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
                        // Get a sortable copy of the effects.
                        let mut effects = track.effects.iter().collect::<Vec<&Effect>>();
                        // Sort by time.
                        effects.sort();
                        // Get the last effect.
                        let last = effects.iter().filter(|e| ve.eq(&e.effect)).last();
                        let program = &conn.state.programs[&track.channel];
                        // Get a new effect type.
                        // Try to use the value of the last effect of this type, if it exists.
                        let effect_type = match ve {
                            ValuelessEffectType::Chorus => match &last {
                                Some(effect) => effect.effect,
                                None => EffectType::Chorus(program.chorus as u16),
                            },
                            ValuelessEffectType::Pan => match &last {
                                Some(effect) => effect.effect,
                                None => EffectType::Pan(program.pan as i16),
                            },
                            ValuelessEffectType::Reverb => match &last {
                                Some(effect) => effect.effect,
                                None => EffectType::Reverb(program.reverb as u16),
                            },
                            ValuelessEffectType::PitchBend => match &last {
                                Some(effect) => effect.effect,
                                None => EffectType::PitchBend(0),
                            },
                            ValuelessEffectType::ChannelPressure => match &last {
                                Some(effect) => effect.effect,
                                None => EffectType::ChannelPressure(0),
                            },
                            ValuelessEffectType::PolyphonicKeyPressure => match &last {
                                Some(effect) => effect.effect,
                                None => EffectType::PolyphonicKeyPressure {
                                    key: MIDDLE_C,
                                    value: 0,
                                },
                            },
                        };
                        // Get a new effect.
                        track.effects.push(Effect {
                            time: state.time.cursor,
                            effect: effect_type,
                        });
                        // Deselect all.
                        state.selection.deselect();
                        // Return the snapshot.
                        Some(Snapshot::from_states(s0, state))
                    }
                    None => None,
                }
            }
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
            let mut tts_strings = vec![
                self.tooltips.get_tooltip(
                    "EFFECTS_PANEL_INPUT_TTS_SCROLL",
                    &[InputEvent::PreviousEffect, InputEvent::NextTrack],
                    input,
                    text,
                ),
                TtsString::from(self.tooltips.get_tooltip(
                    "EFFECTS_PANE_INPUT_TTS_VALUE",
                    &[
                        InputEvent::IncrementEffectValue,
                        InputEvent::DecrementEffectValue,
                    ],
                    input,
                    text,
                )),
            ];
            // Add a new effect.
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
        } else if input.happened(&InputEvent::IncrementEffectValue) {
            self.increment_effect_value(state, conn, true)
        } else if input.happened(&InputEvent::DecrementEffectValue) {
            self.increment_effect_value(state, conn, false)
        } else if input.happened(&InputEvent::IncrementAftertouchNote) {
            self.increment_aftertouch(state, conn, true)
        } else if input.happened(&InputEvent::DecrementAftertouchNote) {
            self.increment_aftertouch(state, conn, false)
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
