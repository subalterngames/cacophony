![Cacophony!](doc/images/banner.png)

**Cacophony is a minimalist and ergonomic MIDI sequencer.** It's minimalist in that it doesn't have a lot of functionality MIDI sequencers have. It's ergonomic in that there is no mouse input and a very clean interface, allowing you to juggle less inputs and avoid awkward mouse motions. **Cacophony is a proof-of-concept of a different way of making music.**

![Screenshot of Cacophony](doc/images/screenshot.jpg)

[Buy Cacophony](https://subalterngames.itch.io/cacophony) (or compile it yourself).

[User-end documentation.](https://subalterngames.com/cacophony.html)

## How to compile

You can compile Cacophony for Linux, MacOS, or Windows. Below is a list of operating systems I've tested:

Linux:

- Ubuntu 18.04 x64 with X11
- Ubuntu 20.04 x64 with X11

MacOS:

- Catalina 10.15.7 x64
- Ventura 13.2.1 Apple Silicon

Windows:

- Windows 10 x64

### All platforms

1. Install Rust
2. Clone this repo

### Linux

1. `apt install clang libspeechd-dev`
2. `cargo build --release`

### MacOS

1. `cargo install cargo-bundle`
2. `cargo bundle`

### Windows

1. `cargo build --release`

## Debugging

If you `cargo build` instead of `cargo build --release`

### Documentation

- [Setup](doc/setup.md)
- [Run](doc/run.md)
- [Design manifesto](doc/manifesto.md)
- [Roadmap](doc/roadmap.md)
- [Limitations](doc/limitations.md)
- [Debug and test](doc/debug_and_test.md)