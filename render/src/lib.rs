//! This crate handles all rendering.
//!
//! The `Panels` struct reads the current state of the program and draws on the window using the `Renderer` struct.
//!
//! This crate does not *modify* any aspects of the state of the program. For that, see the `io` crate.
//!
//! The sizes of the panels are derived from functions in `common::sizes`.
//! It's in `common` because `State` needs some of that information (for example, to define the initial piano roll viewport).
//!
//! Unless otherwise specified, Positions and sizes are set in *grid units* rather than pixels.
//!
//! `ColorKey` is used to define colors. To change the colors, change the config file.

mod color_key;
mod drawable;
mod export_panel;
mod export_settings_panel;
mod field_params;
mod focusable_texture;
mod main_menu;
mod music_panel;
mod panel;
mod panels;
mod renderer;
mod tracks_panel;
use color_key::ColorKey;
use common::ini::Ini;
pub use panels::Panels;
pub use renderer::Renderer;
mod open_file_panel;
mod piano_roll_panel;
use focusable_texture::FocusableTexture;
use text::TTS;
mod popup;
mod types;
pub(crate) use popup::Popup;
use types::*;

pub(crate) const TRACK_HEIGHT_SOUNDFONT: u32 = 4;
pub(crate) const TRACK_HEIGHT_NO_SOUNDFONT: u32 = 1;

/// If subtitles are enabled and Casey is speaking, draw the subtitles.
pub fn draw_subtitles(renderer: &Renderer, tts: &TTS) {
    if let Some(speech) = &tts.speech {
        renderer.subtitle(speech)
    }
}

pub(crate) fn get_track_heights(state: &common::State, conn: &audio::Conn) -> Vec<u32> {
    // Get a list of track element heights.
    let mut elements = vec![];
    for track in state.music.midi_tracks.iter() {
        elements.push(match conn.state.programs.get(&track.channel) {
            Some(_) => TRACK_HEIGHT_SOUNDFONT + 1,
            None => TRACK_HEIGHT_NO_SOUNDFONT,
        });
    }
    elements
}

/// Converts a list of elements into a viewable page.
///
/// - `selected` The index of the current selection.
/// - `elements` A list of elements. Each value is the *height* of the element in grid units.
/// - `height` The height of the viewable area.
pub(crate) fn get_page(selected: &Option<usize>, elements: &[u32], height: u32) -> Vec<usize> {
    // Generate a page of tracks.
    let mut track_page: Vec<usize> = vec![];
    let mut page_h = 0;
    let mut this_page = false;
    for (i, element) in elements.iter().enumerate() {
        // There is room for this track. Add it.
        if page_h + *element <= height {
            track_page.push(i);
            // Increment.
            page_h += *element;
        } else {
            // It's this page. Stop here.
            if this_page {
                break;
            }
            // New page.
            track_page.clear();
            track_page.push(i);
            page_h = *element;
        }
        // This is the page!
        if let Some(selected) = selected {
            if *selected == i {
                this_page = true;
            }
        }
    }
    // We couldn't find the any selected track.
    if !this_page {
        track_page.clear();
    }
    track_page
}
