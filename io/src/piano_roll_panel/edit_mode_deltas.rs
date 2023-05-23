use common::config::{parse, parse_fraction};
use common::ini::Ini;
use common::{EditMode, Fraction, State, EDIT_MODES};

/// Delta factors and values for edit modes.
pub(super) struct EditModeDeltas {
    /// Multiply the beat by this factor to get the quick time.
    quick_time_factor: u32,
    /// In precise mode, move the view left and right by this beat length.
    precise_time: Fraction,
    /// In quick mode, move the viewport up and down by this many half-steps.
    quick_note: u8,
    /// In precise mode, move the view up and down by this many half-steps.
    precise_note: u8,
}

impl EditModeDeltas {
    pub(super) fn new(config: &Ini) -> Self {
        let section = config.section(Some("EDIT_MODE_DELTAS")).unwrap();
        let quick_time_factor: u32 = parse(section, "quick_time_factor");
        let precise_time: Fraction = parse_fraction(section, "precise_time");
        let quick_note: u8 = parse(section, "quick_note");
        let precise_note: u8 = parse(section, "precise_note");
        Self {
            quick_time_factor,
            precise_time,
            quick_note,
            precise_note,
        }
    }

    /// Returns the delta for moving the viewport left or right.
    pub(super) fn get_dt(&self, state: &State) -> Fraction {
        match EDIT_MODES[state.view.mode.get()] {
            EditMode::Normal => state.input.beat,
            EditMode::Quick => state.input.beat * self.quick_time_factor,
            EditMode::Precise => self.precise_time,
        }
    }

    /// Returns the delta for moving the viewport up or down.
    pub(super) fn get_dn(&self, state: &State) -> u8 {
        match EDIT_MODES[state.view.mode.get()] {
            EditMode::Normal => 1,
            EditMode::Quick => self.quick_note,
            EditMode::Precise => self.precise_note,
        }
    }
}
