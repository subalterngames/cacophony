#[derive(Copy, Clone)]
pub enum EditMode {
    /// Edit at a normal pace. What "normal" means is defined by the edit mode.
    Normal,
    /// Edit quickly; a multiple of `Normal`.
    Quick,
    /// Edit precisely. What "precisely" means is defined by the edit mode.
    Precise,
}
