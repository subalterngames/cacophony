/// A sub-mode of the piano roll panel.
#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub enum PianoRollMode {
    Time,
    View,
    Edit,
    Select,
}

/// A sequential list of piano roll modes.
pub const PIANO_ROLL_MODES: [PianoRollMode; 4] = [PianoRollMode::Time, PianoRollMode::View, PianoRollMode::Edit, PianoRollMode::Select];