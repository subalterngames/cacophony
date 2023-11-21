# 0.2.x

## 0.2.0

Reduced idle CPU usage by around 50% and CPU usage while playing music by around 16%. I think I sped up export time too but I haven't properly benchmarked it. 

*(Backend notes)* The problem was that the `Synthesizer` struct (which no longer exists) ran a very fast infinite loop to send samples to `Player`. I replaced this loop with a bunch of `Arc<Mutex<T>>` values that are shared between `Conn` and `Player`. As a result of removing `Synthesizer` I had to reorganize all of the exporter code. There are a lot of small changes but the most noticeable one is that `Exporter` is no longer shared (there is no `Arc<Mutex<Exporter>>` anywhere in the code) and can instead by called like this: `conn.exporter`. A lot of code through Cacophony referenced an `Arc<Mutex<Exporter>>`, so there's a bit of refactoring pretty much everywhere in the codebase.

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