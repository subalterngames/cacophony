/// Describes the state of audio playback.
#[derive(Copy, Clone)]
pub struct TimeState {
    /// The current playback time in samples.
    pub time: Option<u64>,
    /// If true, we're playing music, as opposed to random user input.
    pub music: bool,
}

impl Default for TimeState {
    fn default() -> Self {
        Self {
            time: None,
            music: false,
        }
    }
}
