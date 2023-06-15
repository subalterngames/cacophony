use crate::note::MIDDLE_C;
use crate::sizes::*;
use crate::{
    deserialize_fraction, serialize_fraction, Fraction, Index, SerializableFraction, EDIT_MODES,
};
use fraction::Zero;
use ini::Ini;
use serde::{Deserialize, Serialize};

/// The dimensions of the piano roll viewport.
#[derive(Clone, Debug)]
pub struct View {
    /// The start and end time of the viewport.
    pub dt: [Fraction; 2],
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
        // Get the beats per cell.
        // Get the start time.
        let t0 = Fraction::zero();
        // Get the end time from the size of the panel and the beats per cell.
        let t1 = t0 + Fraction::from(w);
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

    /// Returns a serializable version of the viewport.
    pub(crate) fn serialize(&self) -> SerializableViewport {
        SerializableViewport {
            dt: [
                serialize_fraction(&self.dt[0]),
                serialize_fraction(&self.dt[1]),
            ],
            dn: self.dn,
            mode: self.mode,
            single_track: self.single_track,
        }
    }
}

/// A serializable viewport.
#[derive(Serialize, Deserialize)]
pub(crate) struct SerializableViewport {
    /// The start and end time of the viewport.
    pub(crate) dt: [SerializableFraction; 2],
    /// The start and end note of the viewport.
    pub(crate) dn: [u8; 2],
    /// The current edit mode.
    pub(crate) mode: Index,
    /// If true, we're viewing a single track. If false, we're viewing multiple tracks.
    pub(crate) single_track: bool,
}

impl SerializableViewport {
    /// Returns a deserialized `Viewport`.
    pub(crate) fn deserialize(&self) -> View {
        View {
            dt: [
                deserialize_fraction(&self.dt[0]),
                deserialize_fraction(&self.dt[1]),
            ],
            dn: self.dn,
            mode: self.mode,
            single_track: self.single_track,
        }
    }
}
