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
mod scroll_page;
use focusable_texture::FocusableTexture;
use scroll_page::get_page;
use text::TTS;

/// If subtitles are enabled and Casey is speaking, draw the subtitles.
pub fn draw_subtitles(renderer: &Renderer, tts: &TTS) {
    if let Some(speech) = &tts.speech {
        renderer.subtitle(speech)
    }
}
