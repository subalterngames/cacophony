use crate::note::MIDDLE_C;
use crate::sizes::*;
use crate::{
    deserialize_fraction, serialize_fraction, Fraction, Index, SerializableFraction, EDIT_MODES,
};
use fraction::{One, Zero};
use ini::Ini;
use serde::{Deserialize, Serialize};

/// The dimensions of the piano roll viewport.
#[derive(Clone)]
pub struct View {
    /// The number of beats per visual cell.
    pub beats_per_cell: Fraction,
    /// The start and end time of the viewport.
    pub dt: [Fraction; 2],
    /// The start and end note of the viewport.
    pub dn: [u8; 2],
    /// The current edit mode.
    pub mode: Index,
}

impl View {
    pub fn new(config: &Ini) -> Self {
        // Get the width of the viewport.
        let viewport_size = get_viewport_size(config);
        let w = viewport_size[0];
        // Get the beats per cell.
        let beats_per_cell = Fraction::one();
        // Get the start time.
        let t0 = Fraction::zero();
        // Get the end time from the size of the panel and the beats per cell.
        let t1 = t0 + Fraction::from(w) * beats_per_cell;
        // Get the time delta.
        let dt = [t0, t1];
        // Get the notes delta.
        let h = viewport_size[1] as u8;
        let n0 = MIDDLE_C + h / 2;
        let n1 = n0 - h;
        let dn = [n0, n1];
        let mode = Index::new(0, EDIT_MODES.len());
        Self {
            beats_per_cell,
            dt,
            dn,
            mode,
        }
    }

    /// Returns a serializable version of the viewport.
    pub(crate) fn serialize(&self) -> SerializableViewport {
        SerializableViewport {
            beats_per_cell: serialize_fraction(&self.beats_per_cell),
            dt: [
                serialize_fraction(&self.dt[0]),
                serialize_fraction(&self.dt[1]),
            ],
            dn: self.dn,
            mode: self.mode,
        }
    }
}

/// A serializable viewport.
#[derive(Serialize, Deserialize)]
pub(crate) struct SerializableViewport {
    /// The number of beats per visual cell.
    pub beats_per_cell: SerializableFraction,
    /// The start and end time of the viewport.
    pub dt: [SerializableFraction; 2],
    /// The start and end note of the viewport.
    pub dn: [u8; 2],
    /// The current edit mode.
    pub(crate) mode: Index,
}

impl SerializableViewport {
    /// Returns a deserialized `Viewport`.
    pub(crate) fn deserialize(&self) -> View {
        View {
            beats_per_cell: deserialize_fraction(&self.beats_per_cell),
            dt: [
                deserialize_fraction(&self.dt[0]),
                deserialize_fraction(&self.dt[1]),
            ],
            dn: self.dn,
            mode: self.mode,
        }
    }
}
