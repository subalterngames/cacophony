use std::ops::Index;

use crate::get_effects_panel_width;
use crate::panel::*;
use common::Effect;
use common::ValuelessEffectType;
use text::EFFECT_NAME_KEYS;

const VALUE_WIDTH: u32 = 5;
const EFFECT_TYPES: [ValuelessEffectType; 6] = [
    ValuelessEffectType::Chorus,
    ValuelessEffectType::Reverb,
    ValuelessEffectType::Pan,
    ValuelessEffectType::PitchBend,
    ValuelessEffectType::ChannelPressure,
    ValuelessEffectType::PolyphonicKeyPressure,
];

#[derive(Debug)]
struct EffectField {
    label: Label,
    value: List,
    rect: Rectangle,
}

impl EffectField {
    fn new(x: u32, y: &mut u32, width: u32, i: usize, text: &Text) -> Self {
        let label = Label::new([x, *y], text.get(EFFECT_NAME_KEYS[i]));
        let value = List::new([x, *y + 1], VALUE_WIDTH);
        let rect = Rectangle::new([x, *y], [width, 2]);
        *y += 2;
        Self { label, value, rect }
    }
}

pub(crate) struct EffectsPanel {
    panel: Panel,
    effects: [EffectField; 5],
    aftertouch_label: Label,
    aftertouch_rect: Rectangle,
    aftertouch_values: [KeyList; 2],
}

impl EffectsPanel {
    pub fn new(config: &Ini, renderer: &Renderer, text: &Text) -> Self {
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
        let effects: [EffectField; 5] = (0..5)
            .map(|i| EffectField::new(x, &mut y, width, i, text))
            .collect::<Vec<EffectField>>()
            .try_into()
            .unwrap();
        let aftertouch_label = Label::new([x, y], text.get(EFFECT_NAME_KEYS[5]));
        let aftertouch_rect = Rectangle::new([x, y], [width, 2]);
        y += 1;
        let aftertouch_values = [
            KeyList::new(
                text.get("EFFECTS_PANEL_POLYPHONIC_KEY_PRESSURE_NOTE"),
                [x, y],
                width,
                VALUE_WIDTH,
            ),
            KeyList::new(
                text.get("EFFECTS_PANEL_POLYPHONIC_KEY_PRESSURE_VALUE"),
                [x, y + 1],
                width,
                VALUE_WIDTH,
            ),
        ];
        Self {
            panel,
            effects,
            aftertouch_label,
            aftertouch_rect,
            aftertouch_values,
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
