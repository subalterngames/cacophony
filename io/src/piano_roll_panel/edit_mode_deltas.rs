use common::config::{parse, parse_fraction};
use common::ini::Ini;
use common::{EditMode, Fraction, InputState};

/// Delta factors and values for edit modes.
pub(super) struct EditModeDeltas {
    /// Multiply the beat by this factor to get the quick time.
    quick_time_factor: u32,
    /// In precise mode, move the view left and right by this beat length.
    precise_time: Fraction,
    /// In normal mode, move the viewport up and down by this many half-steps.
    normal_note: u8,
    /// In quick mode, move the viewport up and down by this many half-steps.
    quick_note: u8,
    /// In precise mode, move the view up and down by this many half-steps.
    precise_note: u8,
    /// In normal mode, edit volume by this delta.
    normal_volume: u8,
    /// In quick mode, edit volume by this delta.
    quick_volume: u8,
    /// In precise mode, edit volume by this delta.
    precise_volume: u8,
}

impl EditModeDeltas {
    pub(super) fn new(config: &Ini) -> Self {
        let section = config.section(Some("PIANO_ROLL")).unwrap();
        let quick_time_factor: u32 = parse(section, "quick_time_factor");
        let precise_time: Fraction = parse_fraction(section, "precise_time");
        let normal_note: u8 = parse(section, "normal_note");
        let quick_note: u8 = parse(section, "quick_note");
        let precise_note: u8 = parse(section, "precise_note");
        let normal_volume: u8 = parse(section, "normal_volume");
        let quick_volume: u8 = parse(section, "quick_volume");
        let precise_volume: u8 = parse(section, "precise_volume");
        Self {
            quick_time_factor,
            precise_time,
            normal_note,
            quick_note,
            precise_note,
            normal_volume,
            quick_volume,
            precise_volume,
        }
    }

    /// Returns the delta for time.
    pub(super) fn get_dt(&self, mode: &EditMode, beats: &Vec<Fraction>, input: &InputState) -> Fraction {
        match mode {
            EditMode::Normal => beats[input.beat.get()],
            EditMode::Quick => beats[input.beat.get()] * self.quick_time_factor,
            EditMode::Precise => self.precise_time,
        }
    }

    /// Returns the delta for notes.
    pub(super) fn get_dn(&self, mode: &EditMode) -> u8 {
        match mode {
            EditMode::Normal => self.normal_note,
            EditMode::Quick => self.quick_note,
            EditMode::Precise => self.precise_note,
        }
    }

    /// Returns the delta for volume.
    pub(super) fn get_dv(&self, mode: &EditMode) -> u8 {
        match mode {
            EditMode::Normal => self.normal_volume,
            EditMode::Quick => self.quick_volume,
            EditMode::Precise => self.precise_volume,
        }
    }
}
