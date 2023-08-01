//! This crate handles all rendering.
//!
//! The `Panels` struct reads the current state of the program and draws on the window using the `Renderer` struct.
//!
//! This crate does not *modify* any aspects of the state of the program. For that, see the `io` crate.
//!
//! The sizes of the panels are derived from functions in `common::sizes`.
//! It's in `common` because `State` needs some of that information (for example, to define the initial piano roll viewport).
//!
//! Unless otherwise specified, positions and sizes are set in *grid units* rather than pixels.
//!
//! `ColorKey` is used to define colors. To change the colors, change the config file.]
//!
//! Everything in `field_params/` is used to draw parameters and values.

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
mod page;
mod page_position;
use audio::Conn;
use common::State;
pub(crate) use page::Page;
pub(crate) use page_position::PagePosition;
mod quit_panel;

pub(crate) const TRACK_HEIGHT_SOUNDFONT: u32 = 4;
pub(crate) const TRACK_HEIGHT_NO_SOUNDFONT: u32 = 1;

/// If subtitles are enabled and Casey is speaking, draw the subtitles.
pub fn draw_subtitles(renderer: &Renderer, tts: &TTS) {
    if let Some(subtitles) = tts.get_subtitles() {
        renderer.subtitle(subtitles)
    }
}

/// Get the height of each track. This is shared by the tracks panel and the multi-track piano roll panel.
pub(crate) fn get_track_heights(state: &State, conn: &Conn) -> Vec<u32> {
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
