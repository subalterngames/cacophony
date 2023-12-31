use common::EffectType;

/// The state of an effect UI field.
#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub(super) struct EffectFieldState {
    /// If true, the user selected at least one effect of this type in the piano roll panel.
    pub selected: bool,
    /// If true, there is an effect of this type at the cursor time.
    pub now: bool,
    /// The associated effect.
    pub effect_type: Option<EffectType>,
}
