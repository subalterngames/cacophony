use crate::panel::*;
use crate::{get_track_heights, Page, TRACK_HEIGHT_NO_SOUNDFONT, TRACK_HEIGHT_SOUNDFONT};
use text::{get_file_name, truncate};

const MUTE_OFFSET: u32 = 6;

/// The list of tracks.
pub struct TracksPanel {
    /// The panel.
    panel: Panel,
    /// The size of a track with a SoundFont.
    track_size_sf: [u32; 2],
    /// The size of a track with no SoundFont.
    track_size_no_sf: [u32; 2],
    /// The bank key string.
    bank_key: String,
    /// The gain key string.
    gain_key: String,
    /// The mute string.
    mute_text: String,
    /// The solo string.
    solo_text: String,
    /// The maximum height of a page of tracks.
    page_height: u32,
    /// The width of each field.
    field_width: u32,
}

impl TracksPanel {
    pub fn new(config: &Ini, renderer: &Renderer, text: &Text) -> Self {
        // Get the panel.
        let width = get_tracks_panel_width(config);
        let grid_size = get_window_grid_size(config);
        let height = grid_size[1] - MUSIC_PANEL_HEIGHT;
        let x = MUSIC_PANEL_POSITION[0];
        let y = MUSIC_PANEL_POSITION[1] + MUSIC_PANEL_HEIGHT;
        let panel_position = [x, y];
        let panel = Panel::new(
            PanelType::Tracks,
            panel_position,
            [width, height],
            renderer,
            text,
        );
        // Get the sizes.
        let track_width = width - 2;
        let track_size_sf = [track_width, TRACK_HEIGHT_SOUNDFONT];
        let track_size_no_sf = [track_width, TRACK_HEIGHT_NO_SOUNDFONT];
        let field_width = width - 4;
        let bank_key = text.get("TRACKS_PANEL_BANK");
        let gain_key = text.get("TRACKS_PANEL_GAIN");
        let mute_text = text.get("TRACKS_PANEL_MUTE");
        let solo_text = text.get("TRACKS_PANEL_SOLO");
        let page_height = height - 2;
        // Return.
        Self {
            panel,
            track_size_sf,
            track_size_no_sf,
            bank_key,
            gain_key,
            mute_text,
            solo_text,
            page_height,
            field_width,
        }
    }
}

impl Drawable for TracksPanel {
    fn update(&self, renderer: &Renderer, state: &State, conn: &Conn, text: &Text, _: &PathsState) {
        // Get the focus,
        let focus = self.panel.has_focus(state);
        // Draw the panel.
        self.panel.update(focus, renderer);

        // Get a list of track element heights.
        let track_page = Page::new(
            &state.music.selected,
            &get_track_heights(state, conn),
            self.page_height,
        )
        .visible;
        // Get the color of the separator.
        let separator_color = if focus {
            ColorKey::Separator
        } else {
            ColorKey::NoFocus
        };

        // Draw the tracks.
        let x = self.panel.background.grid_rect.position[0] + 1;
        let mut y = self.panel.background.grid_rect.position[1] + 1;
        for i in track_page {
            let track = &state.music.midi_tracks[i];
            let channel = track.channel;
            let mut track_focus = false;
            // There is a selected track.
            if let Some(selected) = state.music.selected {
                // *This* is the selected track.
                if selected == i {
                    // Get the size of the track.
                    let track_size = match conn.state.programs.get(&channel) {
                        Some(_) => self.track_size_sf,
                        None => self.track_size_no_sf,
                    };
                    let rect = RectanglePixel::new_from_u([x, y], track_size, renderer);
                    // Draw corners.
                    renderer.corners(&rect, focus);
                    // This widget has focus.
                    track_focus = true;
                }
            }
            // Draw the track.
            match conn.state.programs.get(&channel) {
                // No program. No SoundFont.
                None => {
                    let label = Label::new(
                        [x + 1, y],
                        text.get_with_values("TRACKS_PANEL_TRACK_TITLE", &[&channel.to_string()]),
                        renderer,
                    );
                    renderer.text(&label, &Renderer::get_key_color(focus));
                    y += 1;
                }
                // There is a program. Draw the properties.
                Some(program) => {
                    let f = [focus, track_focus];
                    let list = List::new([x, y], self.field_width - 1, renderer);
                    // Draw the preset.
                    renderer.list(&program.preset_name, &list, f);
                    y += 1;
                    // Draw the bank.
                    let bank = KeyList::new(
                        self.bank_key.clone(),
                        [x + 1, y],
                        self.field_width,
                        3,
                        renderer,
                    );
                    renderer.key_list(&program.bank.to_string(), &bank, f);
                    y += 1;
                    // Draw the gain.
                    let gain = KeyList::new(
                        self.gain_key.clone(),
                        [x + 1, y],
                        self.field_width,
                        3,
                        renderer,
                    );
                    renderer.key_list(&track.gain.to_string(), &gain, f);
                    // Mute.
                    if track.mute {
                        let mute_position = [x + self.field_width - MUTE_OFFSET, y];
                        let label = Label::new(mute_position, self.mute_text.clone(), renderer);
                        renderer.text(
                            &label,
                            &Renderer::get_boolean_color(track_focus && focus, track.mute),
                        );
                    }
                    // Solo.
                    if track.solo {
                        let solo_position = [
                            x + self.field_width
                                - MUTE_OFFSET
                                - self.solo_text.chars().count() as u32
                                - 1,
                            y,
                        ];
                        let label = Label::new(solo_position, self.solo_text.clone(), renderer);
                        renderer.text(
                            &label,
                            &Renderer::get_boolean_color(track_focus && focus, track.solo),
                        );
                    }
                    y += 1;
                    // Draw the file.
                    let file_text = truncate(
                        get_file_name(&program.path),
                        self.field_width as usize,
                        false,
                    );
                    let file_color = match (focus, track_focus) {
                        (true, true) => ColorKey::Arrow,
                        (true, false) => ColorKey::Key,
                        _ => ColorKey::NoFocus,
                    };
                    let file_label = LabelRef::new([x + 1, y], file_text, renderer);
                    renderer.text_ref(&file_label, &file_color);
                    y += 1;
                    // Draw a line separator.
                    renderer.horizontal_line_grid(
                        x,
                        x + self.field_width,
                        [0.0, 0.0],
                        y,
                        0.5,
                        &separator_color,
                    );
                    y += 1;
                }
            }
        }
    }
}
