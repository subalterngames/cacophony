use crate::note::MIDDLE_C;
use crate::sizes::*;
use crate::{Index, EDIT_MODES, MAX_NOTE, MIN_NOTE, PPQ_U};
use ini::Ini;
use serde::{Deserialize, Serialize};

/// The maximum zoom time delta in PPQ.
const MAX_ZOOM: u64 = PPQ_U * 5000;

/// The dimensions of the piano roll viewport.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct View {
    /// The start and end time of the viewport in PPQ.
    pub dt: [u64; 2],
    /// The start and end note of the viewport.
    pub dn: [u8; 2],
    /// The current edit mode.
    pub mode: Index,
    /// If true, we're viewing a single track. If false, we're viewing multiple tracks.
    pub single_track: bool,
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
        let mode = Index::new(0, EDIT_MODES.len());
        Self {
            dt,
            dn,
            mode,
            single_track: true,
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
    /// - `zoom` The zoom factor.
    pub fn zoom(&mut self, zoom: f32) {
        self.dt = [
            self.dt[0],
            ((self.dt[1] as f32 * zoom) as u64).clamp(PPQ_U, MAX_ZOOM),
        ];
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
    use ini::Ini;

    const VIEW_T1: u64 = PPQ_U * 133;

    #[test]
    fn view_new() {
        let view = get_new_view();
        assert_eq!(view.dn, [75, 45], "{:?}", view.dn);
        assert_eq!(view.dt, [0, VIEW_T1], "{:?}", view.dt);
        assert_eq!(view.mode.get(), 0, "{}", view.mode.get());
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
        view.zoom(0.75);
        assert_eq!(view.dt, [0, 19152], "{:?}", view.dt);
    }

    fn get_new_view() -> View {
        View::new(&Ini::load_from_file("../data/config.ini").unwrap())
    }
}
