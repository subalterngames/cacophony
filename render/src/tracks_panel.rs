use crate::get_page;
use crate::panel::*;
use text::{get_file_name, truncate};

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
    pub fn new(config: &Ini, text: &Text) -> Self {
        // Get the panel.
        let width = get_tracks_panel_width(config);
        let grid_size = get_window_grid_size(config);
        let height = grid_size[1] - MUSIC_PANEL_HEIGHT;
        let x = MUSIC_PANEL_POSITION[0];
        let y = MUSIC_PANEL_POSITION[1] + MUSIC_PANEL_HEIGHT;
        let panel = Panel::new(PanelType::Tracks, [x, y], [width, height], text);
        // Get the sizes.
        let track_width = width - 2;
        let track_size_sf = [track_width, 4];
        let track_size_no_sf = [track_width, 1];
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
    fn update(
        &self,
        renderer: &Renderer,
        state: &State,
        conn: &Conn,
        _: &Input,
        text: &Text,
        _: &OpenFile,
    ) {
        // Get the focus,
        let focus = self.panel.has_focus(state);
        // Draw the panel.
        self.panel.update(focus, renderer);

        // Get a list of track element heights.
        let mut elements = vec![];
        for track in state.music.midi_tracks.iter() {
            elements.push(match conn.state.programs.get(&track.channel) {
                Some(_) => self.track_size_sf[1],
                None => self.track_size_no_sf[1],
            });
        }
        let track_page = get_page(&state.music.selected, &elements, self.page_height);
        // Get the color of the separator.
        let separator_color = if focus {
            ColorKey::Separator
        } else {
            ColorKey::NoFocus
        };

        // Draw the tracks.
        let x = self.panel.position[0] + 1;
        let mut y = self.panel.position[1] + 1;
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
                    // Draw corners.
                    renderer.corners([x, y], track_size, focus);
                    // This widget has focus.
                    track_focus = true;
                }
            }
            y += 1;
            // Draw the track.
            match conn.state.programs.get(&channel) {
                // No program. No SoundFont.
                None => {
                    let label = Label {
                        text: text
                            .get_with_values("TRACKS_PANEL_TRACK_TITLE", &[&channel.to_string()]),
                        position: [x, y],
                    };
                    renderer.text(&label, &Renderer::get_key_color(focus));
                    y += 1;
                }
                // There is a program. Draw the properties.
                Some(program) => {
                    let f = [focus, track_focus];
                    let list = List::new([x, y], self.field_width);
                    // Draw the preset.
                    renderer.list(&program.preset_name, &list, f);
                    y += 1;
                    // Draw the bank.
                    let bank = KeyList::new(&self.bank_key, [x, y], self.field_width, 3);
                    renderer.key_list(&program.bank.to_string(), &bank, f);
                    y += 1;
                    // Draw the gain.
                    let gain = KeyList::new(&self.gain_key, [x, y], self.field_width, 3);
                    renderer.key_list(&track.gain.to_string(), &gain, f);
                    // Mute.
                    if track.mute {
                        let mute_position = [x + self.field_width - 1, y];
                        let label = Label {
                            text: self.mute_text.clone(),
                            position: mute_position,
                        };
                        renderer.text(
                            &label,
                            &Renderer::get_boolean_color(track_focus && focus, track.mute),
                        );
                    }
                    // Solo.
                    if track.solo {
                        let solo_position = [
                            x + self.field_width - 1 - self.solo_text.chars().count() as u32 - 1,
                            y,
                        ];
                        let label = Label {
                            text: self.solo_text.clone(),
                            position: solo_position,
                        };
                        renderer.text(
                            &label,
                            &Renderer::get_boolean_color(track_focus && focus, track.mute),
                        );
                    }
                    y += 1;
                    // Draw the file.
                    let file_text = truncate(
                        &get_file_name(&program.path),
                        self.field_width as usize,
                        true,
                    );
                    let file_color = match (focus, track_focus) {
                        (true, true) => ColorKey::Arrow,
                        (true, false) => ColorKey::Key,
                        _ => ColorKey::NoFocus,
                    };
                    let file_label = Label {
                        text: file_text,
                        position: [x, y],
                    };
                    renderer.text(&file_label, &file_color);
                    y += 1;
                    // Draw a line separator.
                    renderer.horizontal_line(
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
