#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use audio::connect;
use audio::exporter::Exporter;
use clap::Parser;
use common::config::{load, parse_bool};
use common::sizes::get_window_pixel_size;
use common::{get_bytes, Paths, PathsState, State, VERSION};
use ini::Ini;
use input::Input;
use io::IO;
use macroquad::prelude::*;
use regex::Regex;
use render::{draw_subtitles, Panels, Renderer};
use std::env::current_dir;
use text::{Text, TTS};
use ureq::get;

const CLEAR_COLOR: macroquad::color::Color = macroquad::color::BLACK;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Open the project from disk
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,
    /// Directory where Cacophony data files reside
    ///
    /// Uses './data' if not set
    #[arg(short, long, value_name = "DIR", env = "CACOPHONY_DATA_DIR", default_value = default_data_folder().into_os_string())]
    data_directory: PathBuf,
    /// Make the window fullscreen
    ///
    /// Uses 'fullscreen' under '[RENDER]' in 'config.ini' if not set
    ///
    /// Applied after displaying the splash-screen
    #[arg(short, long, env = "CACOPHONY_FULLSCREEN")]
    fullscreen: bool,
}

#[macroquad::main(window_conf)]
async fn main() {
    // Parse and load the command line arguments.
    let cli = Cli::parse();

    // Get the paths, initialized in loading the window configuration.
    let paths = Paths::get();

    // Load the splash image.
    let splash = load_texture(&paths.splash_path.as_os_str().to_str().unwrap())
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
    } else {
        None
    };
    let draw_texture_params = DrawTextureParams {
        dest_size,
        ..Default::default()
    };
    draw_texture_ex(&splash, 0.0, 0.0, WHITE, draw_texture_params);
    next_frame().await;

    // Load the config file.
    let config = load();

    // Check if a new version is available.
    let remote_version = get_remote_version(&config);

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
    let mut renderer = Renderer::new(&config);

    // Load the panels.
    let mut panels = Panels::new(
        &config,
        &input,
        &state,
        &mut text,
        &renderer,
        remote_version,
    );

    // Resize the screen.
    let window_size = get_window_pixel_size(&config);
    request_new_screen_size(window_size[0], window_size[1]);

    // Fullscreen.
    let fullscreen = if cli.fullscreen {
        // Use the CLI or env argument first if set
        true
    } else {
        let render_section = config.section(Some("RENDER")).unwrap();

        parse_bool(render_section, "fullscreen")
    };
    if fullscreen {
        set_fullscreen(fullscreen);
    }

    // Open the initial save file if set.
    if let Some(save_path) = cli.file {
        io.load_save(
            &save_path,
            &mut state,
            &mut conn,
            &mut paths_state,
            &mut exporter,
        );
    }

    // Begin.
    let mut done: bool = false;
    while !done {
        // Clear.
        clear_background(CLEAR_COLOR);

        // Draw.
        panels.update(&renderer, &state, &conn, &text, &paths_state, &exporter);

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
            panels.late_update(&state, &conn, &mut renderer);

            // Wait.
            next_frame().await;
        }
    }
}

/// Configure the window.
fn window_conf() -> Conf {
    // Parse and load the command line arguments.
    let cli = Cli::parse();

    // Initialize the paths.
    Paths::init(&cli.data_directory);

    let icon = if cfg!(windows) {
        let icon_bytes = get_bytes(&Paths::get().data_directory.join("icon"));
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
        window_width: 926,
        window_height: 240,
        high_dpi: false,
        window_resizable,
        icon,
        ..Default::default()
    }
}

/// Returns a string of the latest version if an update is available.
fn get_remote_version(config: &Ini) -> Option<String> {
    // Check the config file to decide if we should to an HTTP request.
    if parse_bool(config.section(Some("UPDATE")).unwrap(), "check_for_updates") {
        // HTTP request.
        match get("https://raw.githubusercontent.com/subalterngames/cacophony/main/Cargo.toml")
            .call()
        {
            // We got a request.
            Ok(resp) => match resp.into_string() {
                // We got text.
                Ok(text) => {
                    // Get the version from the Cargo.toml.
                    let regex = Regex::new("version = \"(.*?)\"").unwrap();
                    match regex.captures(&text) {
                        Some(captures) => {
                            // If the remote version is the same as the local version, return None.
                            // Why? Because we only need this to show a UI message.
                            // If the versions are the same, we don't need to show the message.
                            if &captures[1] == VERSION {
                                None
                            } else {
                                Some(captures[1].to_string())
                            }
                        }
                        None => None,
                    }
                }
                Err(_) => None,
            },
            Err(_) => None,
        }
    } else {
        None
    }
}

/// Default directory for looking at the 'data/' folder.
fn default_data_folder() -> PathBuf {
    current_dir().unwrap().join("data")
}
