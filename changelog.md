# 0.1.x

## 0.1.3

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