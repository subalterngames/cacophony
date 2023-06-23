use serde::{Deserialize, Serialize};

/// Export metadata.
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct Metadata {
    /// The title of the music.
    pub title: String,
    /// The name of the artist.
    pub artist: Option<String>,
    /// The name of the album.
    pub album: Option<String>,
    /// The track number.
    pub track_number: Option<u16>,
    /// The genre.
    pub genre: Option<String>,
    /// Misc. comments.
    pub comment: Option<String>,
}
