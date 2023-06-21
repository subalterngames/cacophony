use serde::{Deserialize, Serialize};

/// A value that is expressed as a u64 or an f32.
#[derive(Debug, PartialEq, Copy, Clone, Deserialize, Serialize)]
pub struct U64orF32 {
    pub u: u64,
    pub f: f32,
}

impl From<u64> for U64orF32 {
    fn from(value: u64) -> Self {
        Self {
            u: value,
            f: value as f32,
        }
    }
}

impl From<f32> for U64orF32 {
    fn from(value: f32) -> Self {
        Self {
            u: value as u64,
            f: value,
        }
    }
}
