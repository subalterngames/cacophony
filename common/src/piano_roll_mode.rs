use serde::{Deserialize, Serialize};

/// A sub-mode of the piano roll panel.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize, Serialize, Hash)]
pub enum PianoRollMode {
    Time,
    View,
    Edit,
    Select,
}
