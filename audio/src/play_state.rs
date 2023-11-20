#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PlayState {
    /// Not playing any audio.
    NotPlaying,
    /// Playing music. There are queued events. Value: The elapsed time in samples.
    Playing(u64),
    /// There are no more events. Audio is decaying.
    Decaying,
}
