use common::config::{parse, parse_float, parse_fraction};
use common::ini::Ini;
use common::{EditMode, InputState, PPQ_F};
use common::fraction::*;

/// Delta factors and values for edit modes.
pub(super) struct EditModeDeltas {
    /// Multiply the beat by this factor to get the quick time.
    quick_time_factor: u64,
    /// In precise mode, move the view left and right by this PPQ value.
    precise_time: u64,
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
    /// In normal mode, zoom in by this factor.
    normal_zoom: Fraction,
    /// In quick mode, zoom in by this factor.
    quick_zoom: Fraction,
    /// In precise mode, zoom in by this factor.
    precise_zoom: Fraction,
}

impl EditModeDeltas {
    pub(super) fn new(config: &Ini) -> Self {
        let section = config.section(Some("PIANO_ROLL")).unwrap();
        let quick_time_factor: u64 = parse(section, "quick_time_factor");
        let precise_time: u64 = (parse_float(section, "precise_time") * PPQ_F) as u64;
        let normal_note: u8 = parse(section, "normal_note");
        let quick_note: u8 = parse(section, "quick_note");
        let precise_note: u8 = parse(section, "precise_note");
        let normal_volume: u8 = parse(section, "normal_volume");
        let quick_volume: u8 = parse(section, "quick_volume");
        let precise_volume: u8 = parse(section, "precise_volume");
        let normal_zoom = parse_fraction(section, "normal_zoom");
        let quick_zoom = normal_zoom * parse_fraction(section, "quick_zoom");
        let precise_zoom = normal_zoom / parse_fraction(section, "precise_zoom");
        Self {
            quick_time_factor,
            precise_time,
            normal_note,
            quick_note,
            precise_note,
            normal_volume,
            quick_volume,
            precise_volume,
            normal_zoom,
            quick_zoom,
            precise_zoom,
        }
    }

    /// Returns the delta for time.
    pub(super) fn get_dt(&self, mode: &EditMode, input: &InputState) -> u64 {
        match mode {
            EditMode::Normal => input.beat.get_u(),
            EditMode::Quick => input.beat.get_u() * self.quick_time_factor,
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

    /// Returns the zoom delta.
    pub(super) fn get_dz(&self, mode: &EditMode) -> Fraction {
        match mode {
            EditMode::Normal => self.normal_zoom,
            EditMode::Quick => self.quick_zoom,
            EditMode::Precise => self.precise_zoom,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EditModeDeltas;
    use common::ini::Ini;
    use common::PPQ_U;

    #[test]
    fn edit_mode_deltas() {
        let e = EditModeDeltas::new(&Ini::load_from_file("../data/config.ini").unwrap());
        assert_eq!(e.quick_time_factor, 2, "{}", e.quick_time_factor);
        assert_eq!(e.precise_time, PPQ_U / 32, "{}", e.precise_time);
        assert_eq!(e.normal_note, 1, "{}", e.normal_note);
        assert_eq!(e.quick_note, 11, "{}", e.quick_note);
        assert_eq!(e.precise_note, 1, "{}", e.precise_note);
        assert_eq!(e.normal_volume, 1, "{}", e.normal_volume);
        assert_eq!(e.quick_volume, 2, "{}", e.quick_volume);
        assert_eq!(e.precise_volume, 1, "{}", e.precise_volume);
    }
}
