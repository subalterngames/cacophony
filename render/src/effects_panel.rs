use std::ops::Index;
use crate::get_effects_panel_width;
use crate::panel::*;
use common::Effect;
use common::IndexedValues;
use common::ValuelessEffectType;

const VALUE_WIDTH: u32 = 5;

#[derive(Debug)]
struct EffectField {
    label: Label,
    value: EffectFieldValue,
    rect: Rectangle,
}

impl EffectField {
    fn new(x: u32, y: &mut u32, width: u32, effect_types: &mut IndexedValues<ValuelessEffectType, 6>, text: &Text) -> Self {
        let e = effect_types.get();
        // Get the title label by getting the effect name.
        let label = Label::new([x, *y], text.get_valueless_effect_name(&e).to_string());
        // Get the value field.
        let (value, dy) = match e {
            ValuelessEffectType::PitchBend => {
                (EffectFieldValue::Two([KeyList::new(
                    text.get("EFFECTS_PANEL_POLYPHONIC_KEY_PITCH_BEND_VALUE"),
                    [x, *y + 1],
                    width,
                    VALUE_WIDTH,
                ),
                KeyList::new(
                    text.get("EFFECTS_PANEL_POLYPHONIC_KEY_PITCH_BEND_DURATION"),
                    [x, *y + 2],
                    width,
                    VALUE_WIDTH,
                ),]), 4)
            }
            ValuelessEffectType::PolyphonicKeyPressure => {
                (EffectFieldValue::Two([KeyList::new(
                    text.get("EFFECTS_PANEL_POLYPHONIC_KEY_PRESSURE_NOTE"),
                    [x, *y + 1],
                    width,
                    VALUE_WIDTH,
                ),
                KeyList::new(
                    text.get("EFFECTS_PANEL_POLYPHONIC_KEY_PRESSURE_VALUE"),
                    [x, *y + 2],
                    width,
                    VALUE_WIDTH,
                ),]), 4)
            }
            _ => (EffectFieldValue::One(List::new([x, *y + 1], VALUE_WIDTH)), 3)
        };
        // Get the background rectangle.
        let rect = Rectangle::new([x, *y], [width, dy]);
        // Increment the y value.
        *y += dy;
        // Increment so we can set the next effect.
        effect_types.index.increment(true);
        Self { label, value, rect }
    }
}

/// The values of an effect field.
/// Some effects have one value, some have two.
#[derive(Debug)]
enum EffectFieldValue {
    One(List),
    Two([KeyList; 2])
}

pub(crate) struct EffectsPanel {
    panel: Panel,
    effects: [EffectField; 6],
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
        let effects: [EffectField; 6] = effects.try_into().unwrap();
        Self {
            panel,
            effects,
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
                // Get the selected effects.
                let mut selected = [false; EFFECT_TYPES.len()];
                if let Some((_, effects)) = state.selection.get_selection(&state.music) {
                    for effect in effects {
                        for (i, e) in EFFECT_TYPES.iter().enumerate() {
                            if e.eq(&effect.effect) {
                                selected[i] = true;
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
