use crate::{deserialize_fraction, serialize_fraction, Fraction, SerializableFraction};
use serde::{Deserialize, Serialize};

/// The dimensions of the piano roll viewport.
#[derive(Clone)]
pub struct Viewport {
    /// The start and end time of the viewport.
    pub dt: [Fraction; 2],
    /// The start and end note of the viewport.
    pub dn: [u8; 2],
}

impl Viewport {
    /// Returns a serializable version of the viewport.
    pub(crate) fn serialize(&self) -> SerializableViewport {
        SerializableViewport {
            dt: [
                serialize_fraction(&self.dt[0]),
                serialize_fraction(&self.dt[1]),
            ],
            dn: self.dn,
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
}

impl SerializableViewport {
    /// Returns a deserialized `Viewport`.
    pub(crate) fn deserialize(&self) -> Viewport {
        Viewport {
            dt: [
                deserialize_fraction(&self.dt[0]),
                deserialize_fraction(&self.dt[1]),
            ],
            dn: self.dn,
        }
    }
}
