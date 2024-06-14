# 0.2.x

## 0.2.5

- Fixed: Crash if a Channel Pressure or Program Change message is sent from a MIDI controller.

## 0.2.4

- Fixed: If note is played via a qwerty key press, and then an octave is changed via a qwerty key press, there won't be a note-off event.
- Fixed: Cacophony can't found files (saves, soundfonts, etc.) if the file extension contains uppercase characters.
- Fixed: ChildPaths sometimes doesn't set the correct directory when moving up a directory.
- Fixed: It's possible to add notes while the music is playing.
- Fixed: When you save a new file, the panel will list children in the save directory's parent folder.
- (Backend) Fixed clippy warnings for Rust 1.78
- (Backend) The GitHub workflow for building Cacophony now uses the latest version of Rust.
- (Backend) Added tests for ChildPaths.

## 0.2.3

- Optimized text and rectangle rendering, which reduces CPU usage by around 5%.

## 0.2.2

- Fixed: There was an input bug where the play/start key (spacebar) was sometimes unresponsive for the first few presses. This is because audio was still decaying from a previous play, meaning that technically the previous play was still ongoing.
- Fixed: When a new file is created or when a new save file loaded, the app didn't reset correctly.
- Fixed: If you try to play music and there are no tracks or no playable notes, the app starts playing music and then immediately stops.
- Fixed: If you're playing music and then load a save file, the save file can't play music because the synthesizer still has MIDI events from the previous music.

## 0.2.1

- I replaced the default qwerty bindings for note input with a more "standard" layout. This information is stored in config.ini, so if you want the update, make sure to delete Documents/cacophony/config.ini if it exists (Cacophony will use the default data/config.ini instead).
- The background of the export settings panel was the same color as the text, so that it just looked like a weird gray rectangle. I fixed it.

## 0.2.0

Cacophony uses a lot of CPU resources even when it's idle. It shouldn't do that! I reduced Cacophony's CPU usage by around 50%; the exact percentage varies depending on the CPU and the OS. This update is the first big CPU optimization, and probably the most significant.

These are the optimizations:

- In `audio`, `Synthesizer` (which no longer exists) ran a very fast infinite loop to send samples to `Player`, and the loop needlessly consumed CPU resources. I replaced this loop with a bunch of `Arc<Mutex<T>>` values that are shared between `Conn` and `Player`. As a result of removing `Synthesizer` I had to reorganize all of the exporter code. There are a lot of small changes that I'm not going to list here because let's be real, no one reads verbose changelogs, but the most noticeable change is that `Exporter` is a field in `Conn` and is no longer shared (there is no `Arc<Mutex<Exporter>>` anywhere in the code). This change affects a *lot* of the codebase, but it's mostly just refactoring with zero functional differences.
- In `input`, `Input` checked for key downs and presses very inefficently. I only had to change two lines of code to make it much faster. This optimizes CPU usage by roughly 10%.
- A tiny optimization to drawing panel backgrounds. This required refactoring throughout `render`. I think that there are more tiny optimizations that could be made in `render` that cumulatively might make more of a difference.

This update doesn't have any new features or bug fixes. In the future, I'm going to reserve major releases (0.x.0) for big new features, but I had to rewrite so much of Cacophony's code, and the results are such a big improvement, that I'm making this a major release anyway.

# 0.1.x

## 0.1.4

- Fixed: Crash when setting the input beat to less than 1/8

## 0.1.3

- Added .flac exporting
- Fixed: Crash when attempting to add silence to the end of a non-wav multi-track export. I don't remember why I wanted to add silence in the first place, so I've removed it.
- Fixed: `--events [PATH]` argument doesn't work.

## 0.1.2

- Added environment variables and command line arguments:
  - Add a save file path to open it at launch, for example: `./cacophony ~/Documents/cacophony/saves/my_music.cac`
  - `CACOPHONY_DATA_DIR` or `--data_directory` to set the data directory
  - `CACOPHONY_FULLCREEN` or `--fullscreen` to enable fullscreen. 
- (Backend) Renamed feature flags `ubuntu_18_20` and `ubuntu_22` to `speech_dispatcher_0_9` and `speech_dispatcher_0_11`, respectively
- (Backend) `Paths` is now handled as a `OnceLock`
- Added missing compliation requirements in README. Added compilation instructions for Debian 11 and 12.
- Added this changelog