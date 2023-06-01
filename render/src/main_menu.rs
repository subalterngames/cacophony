use crate::panel::*;
use common::hashbrown::HashMap;
use common::music_panel_field::MusicPanelField;
use text::truncate;

/// The music panel.
pub(crate) struct MusicPanel {
    /// The panel background.
    panel: Panel,
    /// The position of each field in grid units.
    field_positions: HashMap<MusicPanelField, [u32; 2]>,
    /// The width of each field in grid units.
    field_width: u32,
    /// The maximum length of the name text.
    max_name_length: usize,
    /// The width of a key-value pair.
    kv_width: usize,
}