mod effect_field;
mod effect_field_values;
mod effect_field_state;
use crate::get_effects_panel_width;
use crate::panel::*;
use common::EffectType;
use effect_field::EffectField;
use effect_field_values::EffectFieldValues;
use effect_field_state::EffectFieldState;


pub(crate) struct EffectsPanel {
    panel: Panel,
    effect_fields: [EffectField; 6],
    effect_types: [EffectType; 6]
}

impl EffectsPanel {
    pub fn new(config: &Ini, state: &State, renderer: &Renderer, text: &Text) -> Self {
        // Get the panel.
        let size = [
            get_effects_panel_width(text),
            get_piano_roll_panel_size(config)[1]
                - PIANO_ROLL_PANEL_TOP_BAR_HEIGHT
                - PIANO_ROLL_PANEL_VOLUME_HEIGHT,
        ];
        let position = [
            get_window_grid_size(config)[0] - size[0],
            MAIN_MENU_HEIGHT + PIANO_ROLL_PANEL_TOP_BAR_HEIGHT,
        ];
        let panel = Panel::new(PanelType::Effects, position, size, renderer, text);
        // Get the labels.
        let x = position[0] + 1;
        let mut y = position[1] + 1;
        let width = size[0] - 2;
        let mut effect_types = state.effect_types.clone();
        let mut effects = vec![];
        effect_types.index.set(0);
        for i in 0..effect_types.index.get_length() {
            effects.push(EffectField::new(x, &mut y, width, &mut effect_types, text));
        }
        let effect_fields: [EffectField; 6] = effects.try_into().unwrap();
        Self {
            panel,
            effect_fields,
            effect_types: EffectType::get_array()
        }
    }
}

impl Drawable for EffectsPanel {
    fn update(
        &self,
        renderer: &Renderer,
        state: &State,
        _: &Conn,
        _: &Text,
        _: &PathsState,
    ) {
        // Render the panel.
        let focus = self.panel.has_focus(state);
        self.panel.update(focus, renderer);
        
        // Is there a playable track?
        match state.music.get_selected_track() {
            Some(track) => {
                // Generate states for each type of field.
                let mut field_states = [EffectFieldState::default(); EffectType::len()];
                // Mark fields as selected and/or current.
                if let Some((_, effects)) = state.selection.get_selection(&state.music) {
                    for effect in effects {
                        for (i, e) in self.effect_types.iter().enumerate() {
                            // This type of effect is selected.
                            if e.valueless_eq(&effect.effect) {
                                field_states[i].selected = true;
                            }
                            // This type of effect is at the playback cursor.
                            match effect.effect {
                                // Check whether the effect is in range.
                                EffectType::PitchBend { value: _, duration } {
                                    if effect.time == state.time.cursor || (effect.time < state.time.cursor && effect.time + duration >= state.time.cursor)
                                }
                            }
                        }
                    }
                }
                // Get the effects at this time.
                let current_effects = track.effects.iter().filter(|e| e.time == state.time.cursor).map(|e| ValuelessEffectType::from(e.effect)).collect::<Vec<ValuelessEffectType>>();
                for ((field, selected), effect) in self.effects.iter().zip(selected).zip(current_effects) {

                }
            }
            None => {
                for e in self.effects.iter() {
                    renderer.text(&e.label, &ColorKey::NoFocus);
                }
            }
        }
    }
}
