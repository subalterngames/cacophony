![Cacophony!](doc/images/banner.png)

**Cacophony is a minimalist and ergonomic MIDI sequencer.** It's minimalist in that it doesn't have a lot of functionality MIDI sequencers have. It's ergonomic in that there is no mouse input and a very clean interface, allowing you to juggle less inputs and avoid awkward mouse motions.

![Screenshot of Cacophony](doc/images/screenshot.jpg)

[Buy Cacophony](https://subalterngames.itch.io/cacophony) (or compile it yourself).

[User-end documentation.](https://subalterngames.com/cacophony)

[Discord Server](https://discord.gg/fUapDXgTYj)

## How to compile

### 1. General setup (all platforms)

1. Install Rust (stable)
2. Clone this repo

### 2. Install libraries

Debian 11 and 12:

```bash
sudo apt install clang cmake speech-dispatcher libspeechd-dev pkg-config libssl-dev librust-alsa-sys-dev
```

Ubuntu 20 and 22:

```bash
apt install clang cmake speech-dispatcher libspeechd-dev pkg-config libssl-dev alsa librust-alsa-sys-dev
```

Ubuntu 18:

```bash
sudo apt install clang cmake speech-dispatcher libspeechd-dev pkg-config libssl-dev alsa
```

MacOS (any):

```bash
cargo install cargo-bundle
```

### 3. Compile

`cargo` is a tool that is bundled with most Rust installs. It can compile a library or application via `cargo build` or compile and then immediately run via `cargo run`. In either case, you can add the `--release` flag to optimize for speed. Most of the code snippets below all use `build --release` but you could swap that for `build`, `run`, or `run --release`.

Debian 12:

```bash
cargo build --release --features speech_dispatcher_0_11
```

Debian 11:

```bash
cargo build --release --features speech_dispatcher_0_9
```

Ubuntu 22:

```bash
cargo build --release --features speech_dispatcher_0_11
```

Ubuntu 18 and 20:

```bash
cargo build --release --features speech_dispatcher_0_9
```

MacOS:

```bash
# You can use `cargo build` and `cargo run` on MacOS,
# but they won't create a .app application.
# `cargo bundle` will create a .app but won't run it.
# The `--release` flag is, as always, optional.
cargo bundle --release
```

Windows:

```bash
cargo build --release
```

## How to run without `cargo`

`cargo run` is a convenient shortcut and is *not* required for running Cacophony.

These instructions will refer to the "target directory", which is where the compiled executable actually is. If you added the `--release` flag, the target directory is: `target/release/`. Otherwise, the target directory is: `target/debug/`.

 To run Cacophony as a typical compiled executable:

### Linux

Copy + paste `data/` into the target directory:

```
cacophony/
....target/
........<debug or release>/
............Cacophony
............data/
```

You can move `Cacophony` out of the target directory but it must be in the same folder as `data/`.

To launch the application:

```bash
./Cacophony
```

(Sorry, but double-clicking doesn't work. I don't know why.)

### MacOS

Navigate to the target directory and double-click Cacophony.app

### Windows

Copy + paste `data/` into the target directory:

```
cacophony/
....target/
........<debug or release>/
............Cacophony.exe
............data/
```

You can move `Cacophony.exe` out of the target directory but it must be in the same folder as `data/`.

To launch the application, double-click `Cacophony.exe`

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

## Known success stories

I've been able to run Cacophony on:

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

## Upload

Assuming that you are Esther Alter and you have the relevant credentials on your computer, you can upload the website and create itch.io builds by doing this:

1. `cd py`
2. `py -3 build.py`
