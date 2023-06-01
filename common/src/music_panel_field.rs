/// Enum values defining the music panel fields.
#[derive(Eq, PartialEq, Copy, Clone, Hash)]
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
