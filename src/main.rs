#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use audio::connect;
use audio::exporter::Exporter;
use common::config::{load, parse_bool};
use common::sizes::get_window_pixel_size;
use common::{get_bytes, Paths, PathsState, State};
use input::Input;
use io::IO;
use macroquad::prelude::*;
use render::{draw_subtitles, Panels, Renderer};
use text::{Text, TTS};

const CLEAR_COLOR: macroquad::color::Color = macroquad::color::BLACK;

#[macroquad::main(window_conf)]
async fn main() {
    // Get the paths.
    let paths = Paths::default();

    // Load the splash image.
    let splash = load_texture(paths.splash_path.as_os_str().to_str().unwrap())
        .await
        .unwrap();
    // Linux X11 can mess this up the initial window size.
    let splash_width = splash.width();
    let splash_height = splash.height();
    let screen_width = screen_width();
    let screen_height = screen_height();
    // Oops something went wrong. 
    let dest_size = if splash_width != screen_width || splash_height != screen_height {
        Some(Vec2::new(screen_width, screen_height))
    }
    else {
        None
    };
    let draw_texture_params = DrawTextureParams { dest_size, ..Default::default()};
    draw_texture_ex(&splash, 0.0, 0.0, WHITE, draw_texture_params);
    next_frame().await;

    // Load the config file.
    let config = load();

    // Create the text.
    let mut text = Text::new(&config, &paths);

    // Try to load the text-to-speech engine.
    let mut tts = TTS::new(&config);

    // Get the input object.
    let mut input = Input::new(&config);

    // Create the exporter.
    let mut exporter = Exporter::new_shared();

    // Create the audio connection.
    let mut conn = connect(&exporter);

    // Create the state.
    let mut state = State::new(&config);

    // Create the paths state.
    let mut paths_state = PathsState::new(&paths);

    // Get the IO state.
    let mut io = IO::new(&config, &input, &state.input, &mut text);

    // Load the renderer.
    let renderer = Renderer::new(&config);

    // Load the panels.
    let mut panels = Panels::new(&config, &state, &input, &mut text, &renderer);

    // Resize the screen.
    let window_size = get_window_pixel_size(&config);
    request_new_screen_size(window_size[0], window_size[1]);

    // Fullscreen.
    let render_section = config.section(Some("RENDER")).unwrap();
    let fullscreen = parse_bool(render_section, "fullscreen");
    if fullscreen {
        set_fullscreen(fullscreen);
    }

    // Begin.
    let mut done: bool = false;
    while !done {
        // Clear.
        clear_background(CLEAR_COLOR);

        // Draw.
        panels.update(
            &renderer,
            &state,
            &conn,
            &input,
            &text,
            &paths_state,
            &exporter,
        );

        // Draw subtitles.
        draw_subtitles(&renderer, &tts);

        // If we're exporting audio, don't allow input.
        if conn.export_state.is_none() {
            // Update the input state.
            input.update(&state);

            // Modify the state.
            done = io.update(
                &mut state,
                &mut conn,
                &input,
                &mut tts,
                &mut text,
                &mut paths_state,
                &mut exporter,
            );
        }

        if !done {
            // Update time itself.
            conn.update();

            // Update the subtitles.
            tts.update();

            // Late update to do stuff like screen capture.
            panels.late_update(&state, &renderer);

            // Wait.
            next_frame().await;
        }
    }
}

/// Configure the window.
fn window_conf() -> Conf {
    let icon = if cfg!(windows) {
        let icon_bytes = get_bytes("./data/icon");
        let big: [u8; 16384] = icon_bytes[0..16384].try_into().unwrap();
        let medium: [u8; 4096] = icon_bytes[16384..20480].try_into().unwrap();
        let small: [u8; 1024] = icon_bytes[20480..21504].try_into().unwrap();
        Some(miniquad::conf::Icon { big, medium, small })
    } else {
        None
    };
    let window_resizable = cfg!(target_os = "linux");
    Conf {
        window_title: "Cacophony".to_string(),
        window_width: 624,
        window_height: 240,
        high_dpi: true,
        window_resizable,
        icon,
        ..Default::default()
    }
}
