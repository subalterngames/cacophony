use serde::{Deserialize, Serialize};

/// The default title of the music.
pub const DEFAULT_TITLE: &str = "My Music";

/// Export metadata.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Metadata {
    /// The title of the music.
    pub title: String,
    /// The name of the artist.
    pub artist: Option<String>,
    /// The name of the album.
    pub album: Option<String>,
    /// The track number.
    pub track_number: Option<u32>,
    /// The genre.
    pub genre: Option<String>,
    /// Misc. comments.
    pub comment: Option<String>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            title: DEFAULT_TITLE.to_string(),
            artist: None,
            album: None,
            track_number: None,
            genre: None,
            comment: None,
        }
    }
}
