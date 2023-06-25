use audio::connect;
use audio::exporter::Exporter;
use common::config::{load, parse_bool};
use common::macroquad;
use common::macroquad::prelude::*;
use common::sizes::get_window_pixel_size;
use common::{get_bytes, Paths, PathsState, State};
use input::Input;
use io::IO;
use render::{draw_subtitles, Panels, Renderer};
use text::{Text, TTS};

#[macroquad::main(window_conf)]
async fn main() {
    // Get the paths.
    let paths = Paths::default();

    // Load the splash image.
    let splash = load_texture(paths.splash_path.as_os_str().to_str().unwrap())
        .await
        .unwrap();
    draw_texture(splash, 0.0, 0.0, WHITE);
    next_frame().await;

    // Load the config file.
    let config = load();

    // Create the text.
    let text = Text::new(&config, &paths);

    // Try to load the text-to-speech engine.
    let mut tts = TTS::new(&config);

    // Get the input object.
    let mut input = Input::new(&config);

    // Create the audio connection.
    let mut conn = connect();

    // Create the state.
    let mut state = State::new(&config);

    // Create the paths state.
    let mut paths_state = PathsState::new(&paths);

    let mut exporter = Exporter::new();

    // Get the IO state.
    let mut io = IO::new(&config, &input, &state.input, &text);

    // Load the renderer.
    let renderer = Renderer::new(&config);

    // Load the panels.
    let mut panels = Panels::new(&config, &state, &input, &text, &renderer);

    // Resize the screen.
    let window_size = get_window_pixel_size(&config);
    request_new_screen_size(window_size[0], window_size[1]);

    // Fullscreen.
    let render_section = config.section(Some("RENDER")).unwrap();
    let fullscreen = parse_bool(render_section, "fullscreen");
    if fullscreen {
        set_fullscreen(fullscreen);
    }

    let clear_color = macroquad::color::BLACK;

    // Begin.
    let mut done: bool = false;
    while !done {
        // Clear.
        clear_background(clear_color);

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
                &text,
                &mut paths_state,
                &mut exporter,
            );
        }

        if !done {
            // Update time itself.
            conn.update();

            // Late update to do stuff like screen capture.
            panels.late_update(&state, &renderer);

            // Wait.
            next_frame().await;
        }
    }
}

/// Configure the window.
fn window_conf() -> Conf {
    let icon_bytes = get_bytes("./data/icon");
    let big: [u8; 16384] = icon_bytes[0..16384].try_into().unwrap();
    let medium: [u8; 4096] = icon_bytes[16384..20480].try_into().unwrap();
    let small: [u8; 1024] = icon_bytes[20480..21504].try_into().unwrap();
    let icon = Some(miniquad::conf::Icon { big, medium, small });
    Conf {
        window_title: "Cacophony".to_string(),
        window_width: 624,
        window_height: 240,
        window_resizable: false,
        icon,
        ..Default::default()
    }
}
