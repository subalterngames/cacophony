/// Enum values defining the music panel fields.
pub enum MusicPanelField {
    Name,
    BPM,
    Gain,
}

pub(crate) const MUSIC_PANEL_FIELDS: [MusicPanelField; 3] = [
    MusicPanelField::Name,
    MusicPanelField::BPM,
    MusicPanelField::Gain,
];
