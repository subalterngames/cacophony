use super::effect_field_values::EffectFieldValues;
use crate::field_params::{KeyList, Label, List, Rectangle};
use common::EffectType;
use text::Text;

const VALUE_WIDTH: u32 = 5;

/// Render data for an effect type.
#[derive(Debug)]
pub(super) struct EffectField {
    /// The label of the effect's name.
    pub label: Label,
    /// The render data for the values of the effect's fields.
    pub values: EffectFieldValues,
    /// The rectangle of the effect's UI widget, spanning the label and the values.
    pub rect: Rectangle,
}

impl EffectField {
    pub(super) fn new(
        x: u32,
        y: &mut u32,
        width: u32,
        effect_type: &EffectType,
        text: &Text,
    ) -> Self {
        // Get the title label by getting the effect name.
        let label = Label::new([x, *y], text.get_effect_type_name(effect_type).to_string());
        // Get the value field.
        let (values, dy) = match effect_type {
            EffectType::PitchBend {
                value: _,
                duration: _,
            } => (
                EffectFieldValues::Two([
                    KeyList::new(
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
                    ),
                ]),
                4,
            ),
            EffectType::PolyphonicKeyPressure { key: _, value: _ } => (
                EffectFieldValues::Two([
                    KeyList::new(
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
                    ),
                ]),
                4,
            ),
            _ => (
                EffectFieldValues::One(List::new([x, *y + 1], VALUE_WIDTH)),
                3,
            ),
        };
        // Get the background rectangle.
        let rect = Rectangle::new([x, *y], [width, dy]);
        // Increment the y value.
        *y += dy;
        Self {
            label,
            values,
            rect,
        }
    }
}
