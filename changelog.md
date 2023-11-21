# 0.2.x

## 0.2.0

Cacophony uses a lot of CPU resources even when it's idle. It shouldn't do that! I reduced Cacophony's idle CPU usage by around 20-50% and by 15-50% while playing music; the exact percentage varies depending on the CPU and the OS. This update is the first big CPU optimization, and probably the most significant; I think all other subsequent optimizations will chip away at the problem.

This update doesn't have any new features or bug fixes. In the future, I'm going to reserve major releases (0.x.0) for big new features, but I had to rewrite so much of Cacophony's code, and the results are such a big improvement, that I'm making this a major release anyway.

*(Backend notes)* The problem was that the `Synthesizer` struct (which no longer exists) ran a very fast infinite loop to send samples to `Player`, and the loop needlessly consumed CPU resources. I replaced this loop with a bunch of `Arc<Mutex<T>>` values that are shared between `Conn` and `Player`. As a result of removing `Synthesizer` I had to reorganize all of the exporter code. There are a lot of small changes that I'm not going to list here because let's be real, no one reads verbose changelogs, but the most noticeable change is that `Exporter` is a field in `Conn` and is no longer shared (there is no `Arc<Mutex<Exporter>>` anywhere in the code). This change affects a *lot* of the codebase, but it's mostly just refactoring with zero functional differences. 

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