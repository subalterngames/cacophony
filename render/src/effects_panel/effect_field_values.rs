use crate::field_params::{KeyList, List};

/// The values of an effect field.
/// Some effects have one value, some have two.
#[derive(Debug)]
pub(super) enum EffectFieldValues {
    One(List),
    Two([KeyList; 2]),
}
