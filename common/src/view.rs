use crate::config::{parse, parse_fraction};
use crate::note::MIDDLE_C;
use crate::sizes::*;
use crate::{EditMode, Index, IndexedEditModes, U64orF32, MAX_NOTE, MIN_NOTE, PPQ_U};
use hashbrown::HashMap;
use ini::Ini;
use serde::{Deserialize, Serialize};

/// The minimum zoom time delta in PPQ.
const MIN_ZOOM: u64 = PPQ_U * 2;
const MAX_ZOOM: u64 = PPQ_U * 10000;

/// The dimensions of the piano roll viewport.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct View {
    /// The start and end time of the viewport in PPQ.
    pub dt: [u64; 2],
    /// The start and end note of the viewport.
    pub dn: [u8; 2],
    /// The current edit mode.
    pub mode: IndexedEditModes,
    /// If true, we're viewing a single track. If false, we're viewing multiple tracks.
    pub single_track: bool,
    /// The zoom time deltas.
    zoom_levels: Vec<u64>,
    /// The index of the current zoom level.
    zoom_index: Index<usize>,
    /// Zoom increments per edit mode.
    zoom_increments: HashMap<EditMode, usize>,
    /// The default zoom index.
    initial_zoom_index: usize,
}

impl View {
    pub fn new(config: &Ini) -> Self {
        // Get the width of the viewport.
        let viewport_size = get_viewport_size(config);
        let w = viewport_size[0];
        // Get the start time.
        let t0 = 0;
        // Get the end time from the size of the panel and the beats per cell.
        let t1 = t0 + w as u64 * PPQ_U;
        // Get the time delta.
        let dt = [t0, t1];
        // Get the notes delta.
        let h = viewport_size[1] as u8;
        let n0 = MIDDLE_C + h / 2;
        let n1 = n0 - h;
        let dn = [n0, n1];
        let mode = EditMode::indexed();
        let section = config.section(Some("PIANO_ROLL")).unwrap();
        let mut zoom_increment = parse_fraction(section, "zoom_increment");
        // Invert the fraction to prevent an infinite loop.
        if zoom_increment.numerator > zoom_increment.denominator {
            zoom_increment.invert();
        }
        let mut zoom_dt = U64orF32::from(t1 - t0);
        let mut zoom_levels = vec![];
        // Zoom in.
        loop {
            zoom_dt = zoom_dt * zoom_increment;
            if zoom_dt.get_u() <= MIN_ZOOM {
                break;
            }
            zoom_levels.insert(0, zoom_dt);
        }
        // Current zoom.
        zoom_dt = U64orF32::from(t1 - t0);
        let zoom_index = zoom_levels.len();
        zoom_levels.push(zoom_dt);
        // Zoom out.
        loop {
            zoom_dt = zoom_dt / zoom_increment;
            if zoom_dt.get_u() >= MAX_ZOOM {
                break;
            }
            zoom_levels.push(zoom_dt);
        }
        let zoom_levels: Vec<u64> = zoom_levels.iter().map(|z| z.get_u()).collect();
        let initial_zoom_index = zoom_index;
        let zoom_index = Index::new(zoom_index, zoom_levels.len());
        let normal_zoom = parse(section, "normal_zoom");
        let quick_zoom = parse(section, "quick_zoom");
        let precise_zoom = parse(section, "precise_zoom");
        let mut zoom_increments = HashMap::new();
        zoom_increments.insert(EditMode::Normal, normal_zoom);
        zoom_increments.insert(EditMode::Quick, quick_zoom);
        zoom_increments.insert(EditMode::Precise, precise_zoom);
        Self {
            dt,
            dn,
            mode,
            single_track: true,
            zoom_levels,
            zoom_index,
            zoom_increments,
            initial_zoom_index,
        }
    }

    /// Returns the time delta from t1 to t0 in PPQ.
    pub fn get_dt(&self) -> u64 {
        self.dt[1] - self.dt[0]
    }

    /// Set the start time of the viewport.
    ///
    /// - `delta` The time delta.
    /// - `add` If true, add. If false, subtract.
    pub fn set_start_time_by(&mut self, dt: u64, add: bool) {
        let delta = self.get_dt();
        self.dt = if add {
            [self.dt[0] + dt, self.dt[1] + dt]
        } else {
            match self.dt[0].checked_sub(dt) {
                Some(t0) => [t0, t0 + delta],
                None => [0, delta],
            }
        };
    }

    /// Move `self.dn` up or down.
    ///
    /// - `dn` Move `self.dn` by this value.
    /// - `add` If true, move up. If false, move down.
    pub fn set_top_note_by(&mut self, dn: u8, add: bool) {
        self.dn = if add {
            // Don't go past n=1.
            if self.dn[0] + dn <= MAX_NOTE {
                [self.dn[0] + dn, self.dn[1] + dn]
            }
            // Snap to n=1.
            else {
                [MAX_NOTE, MAX_NOTE - self.get_dn()]
            }
        } else {
            // Don't go past n=0.
            if self.dn[1] - dn >= MIN_NOTE {
                [self.dn[0] - dn, self.dn[1] - dn]
            }
            // Snap to n=0.
            else {
                [MIN_NOTE + self.get_dn(), MIN_NOTE]
            }
        }
    }

    /// Zoom in or out. `self.dt[0]` doesn't change.
    ///
    /// - `up` If true, zoom in. If false, zoom out.
    pub fn zoom(&mut self, up: bool) {
        // Get the number of increment steps.
        let increments = self.zoom_increments[&self.mode.get()];
        // Use modulus division to get to the next valid index.
        // For example, if `increments == 2` then we need to get to an index relative to `self.initial_zoom_index` that is a multiple of 2.
        let zi = self.zoom_index.get();
        let m = (if zi > self.initial_zoom_index {
            zi - self.initial_zoom_index
        } else {
            self.initial_zoom_index - zi
        }) % increments;
        for _ in 0..m {
            if !self.zoom_index.increment_no_loop(up) {
                break;
            }
        }
        // Increment the zoom index.
        for _ in 0..increments {
            if !self.zoom_index.increment_no_loop(!up) {
                break;
            }
        }
        // Get the time delta.
        let dt = self.zoom_levels[self.zoom_index.get()];
        self.dt = [self.dt[0], dt];
    }

    /// Returns the note delta.
    fn get_dn(&self) -> u8 {
        self.dn[0] - self.dn[1]
    }
}

#[cfg(test)]
mod tests {
    use crate::time::PPQ_U;
    use crate::view::View;
    use crate::EditMode;
    use ini::Ini;

    const VIEW_T1: u64 = PPQ_U * 133;

    #[test]
    fn view_new() {
        let view = get_new_view();
        assert_eq!(view.dn, [75, 45], "{:?}", view.dn);
        assert_eq!(view.dt, [0, VIEW_T1], "{:?}", view.dt);
        assert_eq!(view.mode.index.get(), 0, "{}", view.mode.index.get());
        assert_eq!(view.single_track, true, "{}", view.single_track);
    }

    #[test]
    fn view_dt() {
        let mut view = get_new_view();
        view.set_start_time_by(PPQ_U, false);
        assert_eq!(view.dt, [0, VIEW_T1], "{:?}", view.dt);
        let dt = PPQ_U / 2;
        view.set_start_time_by(dt, true);
        assert_eq!(view.dt, [dt, VIEW_T1 + dt], "{:?}", view.dt);
        view.set_top_note_by(4, true);
        assert_eq!(view.dn, [79, 49], "{:?}", view.dn);
        view.set_top_note_by(4, false);
        assert_eq!(view.dn, [75, 45], "{:?}", view.dn);
        view.dt = [0, VIEW_T1];
    }

    #[test]
    fn view_dz() {
        let mut view = get_new_view();
        // Test modes.
        view.mode.index.set(0);
        assert_eq!(view.mode.get(), EditMode::Normal);
        view.mode.index.set(1);
        assert_eq!(view.mode.get(), EditMode::Quick);
        view.mode.index.set(2);
        assert_eq!(view.mode.get(), EditMode::Precise);
        // Zoom in precisely.
        view.zoom(true);
        // Reset to the default zoom.
        view.mode.index.increment(true);
        assert_eq!(view.mode.get(), EditMode::Normal);
        view.zoom(false);
        assert_eq!(view.get_dt(), VIEW_T1);
    }

    fn get_new_view() -> View {
        View::new(&Ini::load_from_file("../data/config.ini").unwrap())
    }
}
