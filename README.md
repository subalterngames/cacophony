# Cacophony

**Cacophony is a minimalist and ergonomic MIDI sequencer.** It's minimalist in that it doesn't have a lot of functionality MIDI sequencers have. It's ergonomic in that there is no mouse input and a very clean interface, allowing you to juggle less inputs and avoid awkward mouse motions.

Cacophony is a proof-of-concept of an unusual way to create digital music.

I made Cacophony because I want to make music in a very particular way that I couldn't find anywhere.

Cacophony's mascot is Casey the Transgender Cacodemon. No official artwork of Casey exists because I don't want to be cursed.

## How to compile

### Linux

*Tested on Ubuntu 20.04 with an x64 processor.*

1. `apt install clang libspeechd-dev`
2. `cargo build --release`

## MacOS

*Tested on MacOS Catalina 10.15.7 Intel CPU and MacOS Ventura 13.2.1 M2 Apple Silicon CPU*

1. `cargo install cargo-bundle`
2. `cargo bundle`

### Windows

1. `cargo build --release`

## Design Principles

The overall goal is that Cacophony should feel as close as possible to improvising on an actual instrument and writing notes on physical paper with a physical pencil. There should be as little as possible between you and writing music.

### 1. Clean interface

Cacophony's ASCII-esque interface is inspired by roguelikes moreso than, say, vim. I like ASCII interfaces because they require the designer to be highly economical about what information is on the screen at any given time.

Every DAW I've ever seen is exceedingly, jaw-droppingly ugly. I'm stating this as a programmer who routinely uses bloated IDEs like Unity3D; Unity may be horrendous, but DAW interfaces are like an order of magnitude worse.

Many DAWs are attempting to simulate actual studio setups or synthesizers. That's probably useful if you want to actually make a career out of making music, but I don't want to make a career out of making music, I just want to make music. So, Cacophony doesn't make any attempt to emulate any "real-world" audio production verbs.

### 2. Screen-reader friendly

Cacophony is *very* screen-reader friendly. To avoid cluttering up the interface with tooltips and labels, nearly everything can be described by the text-to-speech engine. The actualy text-to-speech engine is external to the application. This probably makes the application more suspectible to bitrot but that might just be a fundamental feature of accessible software.

Audio-only tooltips are usually frowned upon because there's no way to know if the user has audio on. However, this is a music-making program so we *do* know that the user has audio on.

### 3. Ergonomic

I don't want to juggle a qwerty keyboard, MIDI keyboard, *and* a mouse. I don't want to click and drag a mouse to do basic tasks like set the gain. I don't want to click through several windows to listen to each instrument.

Cacophony is qwerty-and-MIDI input *only*. Between this an the helpful screen-reader, the best-case scenario for Cacophony is that it reaches a state at which experienced users hardly ever need to look at the screen.

### 4. No live recording

Note input is modeled after MuseScore and doesn't include live recording. I really like this form of input, but I don't need the result to be pristine sheet music that I can print to PDF, hence Cacophony's piano roll rectangles. (Cacophony *can* export to .mid which can then be imported into MuseScore.)

## Limitations

**By design, Cacophony lacks many features common to audio software. If you want those features, I encourage you to use Cacophony in combination with other software.** 

### 1. Lacks many features

See the top of the README. This is by design.

### 2. No VSTs

VSTs are *unbelievably* heterogenous. The big DAWs out there tend to have custom code *per VST*, which is well outside of what I can support.

### 3. Can't import wav data

This might change, but importing wav data would be a huge undertaking. My goal, for now, is to stabilize Cacophony. If there is interested in 

### 4. Quantized SoundFont input

Cacophony only soundfonts, and no other type of input, because they're very easy to find and support. Input is always quantized because this solves a *lot* of other interface problems.

Cacophony *technically* supports .sf3 files but it doesn't work very well.

### 5. No FLAC export

Sorry, but FLAC support in Rust is really bad! Hopefully this will change soon.

## Automatically send events (debug mode)

If run a debug build, you can pass input events in like this: `cargo build -- --events events.txt`

...where the contents of `events.txt` is:

```
NextPanel
AddTrack
EnableSoundFontPanel
SelectFile
```

This can be useful if you're debugging and you need to input some initial events.