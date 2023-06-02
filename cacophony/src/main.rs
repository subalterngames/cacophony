use audio::connect;
use common::config::{load, parse_bool};
use common::macroquad;
use common::macroquad::prelude::*;
use common::sizes::get_window_pixel_size;
use common::{Paths, State};
use input::Input;
use io::IO;
use render::{Panels, Renderer};
use text::{Text, TTS};

#[macroquad::main(window_conf)]
async fn main() {
    // Get the paths.
    let paths = Paths::new();

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

    // Get the IO state.
    let mut io = IO::new(&config, &input, &state.input, &text);

    // Load the renderer.
    let renderer = Renderer::new(&config, &text);

    // Load the panels.
    let panels = Panels::new(&config, &text);

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
            &io.open_file_panel.open_file,
        );

        // Receive input. Possible say something or do an audio operation. Modify the state.
        done = io.update(&mut state, &mut conn, &mut input, &mut tts, &text, &paths);

        // Wait.
        next_frame().await;
    }
}

/// Configure the window.
fn window_conf() -> Conf {
    Conf {
        window_title: "Cacophony".to_string(),
        window_width: 372,
        window_height: 144,
        window_resizable: false,
        ..Default::default()
    }
}
