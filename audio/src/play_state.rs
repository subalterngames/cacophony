#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum PlayState {
    /// Not playing any audio.
    NotPlaying,
    /// Playing music. There are queued events.
    Playing,
    /// There are no more events. Audio is decaying.
    Decaying,
}
