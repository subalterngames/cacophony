use crate::{deserialize_fraction, serialize_fraction, Fraction, Index, SerializableFraction};
use serde::{Deserialize, Serialize};

/// The dimensions of the piano roll viewport.
#[derive(Clone)]
pub struct View {
    /// The start and end time of the viewport.
    pub dt: [Fraction; 2],
    /// The start and end note of the viewport.
    pub dn: [u8; 2],
    /// The current edit mode.
    pub mode: Index,
}

impl View {
    /// Returns a serializable version of the viewport.
    pub(crate) fn serialize(&self) -> SerializableViewport {
        SerializableViewport {
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
            dt: [
                deserialize_fraction(&self.dt[0]),
                deserialize_fraction(&self.dt[1]),
            ],
            dn: self.dn,
            mode: self.mode,
        }
    }
}
