use crate::IndexedValues;
use serde::{Deserialize, Serialize};

pub type IndexedEditModes = IndexedValues<EditMode, 3>;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Default, Deserialize, Serialize)]
pub enum EditMode {
    /// Edit at a normal pace. What "normal" means is defined by the edit mode.
    #[default]
    Normal,
    /// Edit quickly; a multiple of `Normal`.
    Quick,
    /// Edit precisely. What "precisely" means is defined by the edit mode.
    Precise,
}

impl EditMode {
    pub fn indexed() -> IndexedEditModes {
        IndexedValues::new(0, [EditMode::Normal, EditMode::Quick, EditMode::Precise])
    }
}
