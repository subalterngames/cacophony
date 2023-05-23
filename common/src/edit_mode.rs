use crate::Index;

#[derive(Eq, PartialEq)]
pub enum EditMode {
    /// Edit at a normal pace. What "normal" means is defined by the edit mode.
    Normal,
    /// Edit quickly; a multiple of `Normal`.
    Quick,
    /// Edit precisely. What "precisely" means is defined by the edit mode.
    Precise,
}

/// An ordered array of the edit modes.
pub const EDIT_MODES: [EditMode; 3] = [EditMode::Normal, EditMode::Quick, EditMode::Precise];

/// Returns an indexer for an edit mode.
pub fn get_index() -> Index {
    Index::new(0, EDIT_MODES.len())
}
