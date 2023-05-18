/// Describes the state of audio playback.
#[derive(Copy, Clone, Default)]
pub struct TimeState {
    /// The current playback time in samples.
    pub time: Option<u64>,
    /// If true, we're playing music, as opposed to random user input.
    pub music: bool,
}