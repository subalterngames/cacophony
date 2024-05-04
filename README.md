![Cacophony!](doc/images/banner.png)

**Cacophony is a minimalist and ergonomic MIDI sequencer.** It's minimalist in that it doesn't have a lot of functionality MIDI sequencers have. It's ergonomic in that there is no mouse input and a very clean interface, allowing you to juggle less inputs and avoid awkward mouse motions.

![Screenshot of Cacophony](doc/images/screenshot.jpg)

[Buy Cacophony](https://subalterngames.itch.io/cacophony) (or compile it yourself).

[User-end documentation.](https://subalterngames.com/cacophony)

[Discord Server](https://discord.gg/fUapDXgTYj)

## How to compile

I compile Cacophony with Rust 1.74.0 for Linux, MacOS, or Windows. Below is a list of operating systems I've tested:

Linux:

- Ubuntu 18.04 i386 with X11
- Ubuntu 18.04 x64 with X11
- Ubuntu 20.04 x64 with X11
- Ubuntu 22.04 x64 with X11

MacOS:

- Catalina 10.15.7 x64
- Ventura 13.2.1 Apple Silicon

Windows:

- Windows 10 x64

### All platforms

1. Install Rust (stable)
2. Clone this repo

### Linux

#### Debian 11

1. `apt install clang cmake speech-dispatcher libspeechd-dev pkg-config libssl-dev librust-alsa-sys-dev`
2. `cargo build --release --features speech_dispatcher_0_9`

#### Debian 12

1. `apt install clang cmake speech-dispatcher libspeechd-dev pkg-config libssl-dev librust-alsa-sys-dev`
2. `cargo build --release --features speech_dispatcher_0_11`

#### Ubuntu 18

1. `apt install clang cmake speech-dispatcher libspeechd-dev pkg-config libssl-dev alsa`
2. `cargo build --release --features speech_dispatcher_0_9`

#### Ubuntu 20

1. `apt install clang cmake speech-dispatcher libspeechd-dev pkg-config libssl-dev alsa librust-alsa-sys-dev`
2. `cargo build --release --features speech_dispatcher_0_9`

#### Ubuntu 22

1. `apt install clang cmake speech-dispatcher libspeechd-dev pkg-config libssl-dev alsa librust-alsa-sys-dev`
2. `cargo build --release --features speech_dispatcher_0_11`

### MacOS

1. `cargo install cargo-bundle`
2. `cargo bundle --release`

### Windows

1. `cargo build --release`

### OpenBSD

1. Install and configure jack
1. `cargo build --release --features jack

## Tests

To test, just `cargo test --all`.

Sometimes when debugging, it's useful to create the same initial setup every time. To do this, you can pass input events in like this: `cargo run -- --events events.txt`

...where the contents of `events.txt` is something like:

```
NextPanel
AddTrack
EnableSoundFontPanel
SelectFile
```

## How to run

You can run Cacophony like any other application or you can use Rust's `cargo run` to compile and execute.

### Linux

There are two ways to run Cacophony:

1. Copy + paste `data/` into the output directory (`target/release/`). Open a terminal in `release/` and run `./Cacophony` .
2. Instead of `cargo build --release`, run `cargo run --release` Include the `--features` listed above, for example `cargo build --release --features speech_dispatcher_0_11` on Ubuntu 22

### MacOS

There are two ways to run Cacophony:

1. After compiling, double-click `Cacophony.app` (located in `./target/release/`)
2. `cargo run --release` This will compile and launch the application but it won't create a .app

### Windows

There are two ways to run Cacophony:

1. Copy + paste `data/` into the output directory (`target/release/`) and double-click `Cacophony.exe` (located in `release/`)
2. Instead of `cargo build --release`, run `cargo run --release`

## Upload

Assuming that you are Esther Alter and you have the relevant credentials on your computer, you can upload the website and create itch.io builds by doing this:

1. `cd py`
2. `py -3 build.py`
