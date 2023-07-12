# Cacophony

**The programmer's DAW.**

## 1. Purpose

DAWs are bad and ugly and the workflow is poorly engineered and physically awkward. Ask yourself: Do you *really* want to click your mouse to turn thousands of knob.jpg's? **NO YOU DON'T.**  Do you want to look at a thousand nested windows with small font sizes?  **NO YOU DON'T.** Do you want to learn a new workflow? **NO YOU DON'T.** 

This is what you *do* want to do: **YOU WANT TO MAKE MUSIC.**

Back in the day, you'd learn an instrument, maybe the piano, and then you'd spend hours or minutes trying out new tunes until you found something interesting and then you'd write it down. The word "workflow" didn't exist yet. The purposes of Cacophony is to offer a DAW that feels as close to that experience as possible.

## 2. Design Goals

### Simplified tech chain

The audio ecosystem is god-awful. There's a bazillion variants on "sound font", something called VST that don't have a standardized framework, and lots of other input-output options too. You aren't interested in any of that because, see above, you want to make music.

Cacophony provides wrapper scripts for DAW file types, plugins, libraries, etc. 

Advantages to Cacophony wrapper scripts:

- Standardized, agnostic input-output. These scripts have functions with primitive data type parameters, and they have a function that returns a waveform. That's it. You know what to expect, and you know that the underlying architecture will play nice with everything else in your project.
- Less interface clutter. None of these scripts allow for an external graphical interface while still exposing the same functionality.

Disadvantages to Cacophony wrapper scripts:

- Probably there is some loss in input precision because not everything can fit this paradigm.
- If the wrapper script doesn't exist yet, someone has to write it.

### Works out of the box

Cacophony isn't really designed to give you maximum customization options because, *again, say it with me*, you want to make music, not write UI scripts. So, you get a good-enough interface and some ability to change colors, fonts, language, etc. and not much more. It's fine. Don't worry about it.

## 3. UI

Cacophony has a quasi-text-only interface (technically not ASCII; Cacophony uses UTF-8). The interface is clean, (relatively) sparse, and, whenever possible, won't open sub-menus or sub-windows. There are also crisply-drawn rectangular borders and filled rectangles (for notes). Everything is monospace and on a grid (there are never multiple fonts or font sizes). Rendering is crisp with a programmer IDE color scheme (probably OneDark by default).

Input is keyboard/MIDI controller. No mouse input.

Cacophony has a notion of groups of UI fields. There is a text-to-speech option that can tell you what's on the panel.

Between the minimal interface and the text-to-speech and the MIDI controls, the goal is to minimize how often you look at the screen and maximize how much time your playing an instrument.

## 4. API Spec

### `WaveformGenerator`

`WaveformGenerator` is an abstract class. Subclasses may have an arbitrary number of fields. Cacaphony cares about only three functions:

1. The constructor, which never has arguments.
2. `get_waveform(**kwargs) -> bytes` Returns a bytestring of a waveform.
3. `get_type() -> WaveformGeneratorType` This specifies the type of generator. This is used internally to decide the user control scheme, UI presentation, etc.

### `WaveformGeneratorType`

### Quasi-primitive data types

- `Uint` Unsigned integer
- `UFloat` Unsigned float
- `Zero1` A float clamped to 0-1
- `Zero127` An integer clamped to 0-127
- `Zero255` An integer clamped to 0-255
- `Options` A list of strings or enum values and an index.

## 5. The Name

Why Cacophony? Doesn't that literally mean *bad sound*? Well, guess what:

1. It's funny.
2. You're all using stuff named "Ableton" and "Pro Tools" and "Reaper" as if that evokes any emotion at all.


## 6. Mascot

Cacophony's mascot is Casey the Transgender Cacodemon. No official artwork of Casey exists because I don't want to be cursed.

## How to compile

### Linux

*Tested on Ubuntu 20.04 with an x64 processor.*

1. `apt install clang libclang1 libspeechd-dev`
2. `cargo build --release`

## MacOS

1. `cargo build --release`

### Windows

1. `cargo build --release`