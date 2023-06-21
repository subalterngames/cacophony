use crate::note::MIDDLE_C;
use crate::sizes::*;
use crate::{Index, EDIT_MODES, PPQ_U};
use ini::Ini;
use serde::{Deserialize, Serialize};

/// The dimensions of the piano roll viewport.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct View {
    /// The start and end time of the viewport in PPQ.
    pub dt: [u64; 2],
    /// The start and end note of the viewport.
    pub dn: [u8; 2],
    /// The current edit mode.
    pub mode: Index,
    /// If true, we're viewing a single track. If false, we're viewing multiple tracks.
    pub single_track: bool,
}

impl View {
    pub fn new(config: &Ini) -> Self {
        // Get the width of the viewport.
        let viewport_size = get_viewport_size(config);
        let w = viewport_size[0];
        // Get the start time.
        let t0 = 0;
        // Get the end time from the size of the panel and the beats per cell.
        let t1 = t0 + w as u64 * PPQ_U;
        // Get the time delta.
        let dt = [t0, t1];
        // Get the notes delta.
        let h = viewport_size[1] as u8;
        let n0 = MIDDLE_C + h / 2;
        let n1 = n0 - h;
        let dn = [n0, n1];
        let mode = Index::new(0, EDIT_MODES.len());
        Self {
            dt,
            dn,
            mode,
            single_track: true,
        }
    }
}
