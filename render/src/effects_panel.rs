mod effect_field;
mod effect_field_state;
mod effect_field_values;
use crate::get_effects_panel_width;
use crate::panel::*;
use common::EffectType;
use effect_field::EffectField;
use effect_field_state::EffectFieldState;
use effect_field_values::EffectFieldValues;
use strum::EnumCount;

pub(crate) struct EffectsPanel {
    panel: Panel,
    effect_fields: [EffectField; 6],
}

impl EffectsPanel {
    pub fn new(config: &Ini, renderer: &Renderer, text: &Text) -> Self {
        // Get the panel.
        let size = [
            get_effects_panel_width(text),
            get_piano_roll_panel_size(config)[1] + 2,
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
        let mut effect_fields = vec![];
        for effect_type in EffectType::get_array() {
            effect_fields.push(EffectField::new(x, &mut y, width, &effect_type, text));
        }
        Self {
            panel,
            effect_fields: effect_fields.try_into().unwrap(),
        }
    }

    fn get_effect_index(effect_type: &EffectType) -> usize {
        effect_type.get_ordinal() as usize
    }
}

impl Drawable for EffectsPanel {
    fn update(&self, renderer: &Renderer, state: &State, _: &Conn, _: &Text, _: &PathsState) {
        // Render the panel.
        let focus = self.panel.has_focus(state);
        self.panel.update(focus, renderer);

        // Is there a playable track?
        if let Some(track) = state.music.get_selected_track() {
            // Generate states for each type of field.
            let mut field_states = [EffectFieldState::default(); EffectType::COUNT];
            // Get all effects at the cursor.
            for effect in track
                .effects
                .iter()
                .filter(|e| e.at_time(state.time.cursor))
            {
                // Get the index of the effect.
                let i = Self::get_effect_index(&effect.effect);
                // This effect is happening now.
                field_states[i].now = true;
                // Set the effect type.
                field_states[i].effect_type = Some(effect.effect);
            }
            if focus {
                // Mark fields as selected.
                if let Some((_, effects)) = state.selection.get_selection(&state.music) {
                    for effect in effects {
                        let i = Self::get_effect_index(&effect.effect);
                        if field_states[i].now {
                            field_states[i].selected = true;
                        }
                    }
                }
            }
            // Render.
            let selected_in_panel_index = state.effect_types.index.get();
            for (i, (field_state, ui_field)) in
                field_states.iter().zip(&self.effect_fields).enumerate()
            {
                let (key_color, values_focus) = if focus {
                    // Draw the background of a selected note.
                    if field_state.selected {
                        renderer.rectangle(&ui_field.rect, &ColorKey::NoteSelected);
                        (ColorKey::Separator, false)
                    } else {
                        match field_state.effect_type {
                            Some(_) => (ColorKey::Key, true),
                            None => (ColorKey::Separator, false),
                        }
                    }
                } else {
                    (ColorKey::NoFocus, false)
                };
                // Draw corners.
                if i == selected_in_panel_index {
                    renderer.corners(&ui_field.rect, focus);
                }
                // Draw the label.
                renderer.text(&ui_field.label, &key_color);
                let effect = match field_state.effect_type {
                    Some(effect) => effect,
                    None => EffectType::get_array()[i],
                };
                // Draw the values.
                match &ui_field.values {
                    EffectFieldValues::One(list) => {
                        renderer.list(&effect.get_value_string(), list, [focus, values_focus])
                    }
                    EffectFieldValues::Two(key_lists) => {
                        // Draw the primary value.
                        renderer.key_list(
                            &effect.get_value_string(),
                            &key_lists[0],
                            [focus, values_focus],
                        );
                        if let Some(v) = effect.get_secondary_value() {
                            renderer.key_list(&v, &key_lists[1], [focus, values_focus]);
                        }
                    }
                }
            }
        }
    }
}
