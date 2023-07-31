# TODO

## Big picture

- [ ] Setup and MIDI setup docs
- [ ] Discord
- [ ] License
- [ ] itch page
- [ ] Test on Linux
- [x] Test on OSX
- [ ] Help links
- [x] Multi-track view
- [x] ~~Effects~~
- [x] ~~Text ref~~
- [x] Rewrite README
- [x] Export MIDI
- [x] Export separate tracks
- [x] Export ogg
- [x] Export mp3
- [x] ~~Export flac~~
- [x] Export chunks
- [x] More MIDI bindings
- [x] Test all TTS
  - [x] Main
  - [x] Music
  - [x] Tracks
  - [x] Open file - SF
  - [x] Open file - Save
  - [x] Open file - Load
  - [x] Export Settings
  - [x] Piano Roll
- [x] Metadata: name, artist, album, year, genre, track number, comments
- [x] Export settings
  - [x] MIDI: Copyright
  - [x] ~~wav: framerate; 16, 24, 32~~
  - [x] ~~flac: ???~~
  - [x] mp3: framerate, quality, variable speed, channel mode, bit rate mode
  - [x] ogg: framerate; quality (0-10)
  - [x] MIDI: copyright
  - [x] Separate tracks: auto-name
- [x] Asterisk for changes
- [ ] Quit unsaved prompt
- [x] Note times should be measured in PPQ
- [x] Export buffer
- [x] TTS is not helpful!!
- [x] Internal documentation (see below)
- [ ] More tests (see below)
- [ ] Merge and clobber

## Internal documentation

- [x] audio
- [x] common
- [x] input
- [x] io
- [x] render
- [x] text
- [x] main

## Tests

- [x] ~~config~~
- [ ] exporter
- [x] fraction
- [ ] input_state
- [ ] file_or_directory
- [ ] file_and_directory
- [ ] child_paths
- [x] ~~font~~
- [x] index
- [x] indexed_values
- [x] ~~paths_state~~
- [ ] select_mode
- [ ] sizes
- [ ] midi_binding
- [ ] qwerty_binding
- [x] ~~abc123~~
- [x] ~~boolean_corners~~
- [x] boolean
- [ ] key_input
- [ ] key_list_corners
- [x] key_list
- [x] key_width
- [x] list
- [ ] viewable_notes
- [x] ~~tts_string~~
- [ ] value_map

## Linux Bugs

- [x] Can't resize window in Linux
- [ ] Can't double-click Linux executable
- [x] Can't connect to MIDI input on Linux
- [ ] Note names aren't aligned correctly in high DPI Linux
- [x] Textures are too big in high DPI Linux
- [x] Splash window is too small in high DPI Linux

## Bugs

- [x] Open file panel background screen capture
- [x] During alphanumeric input, all other input should be locked except TTS.
- [x] Don't show input backgrounds if alphanumeric input is disabled.
- [x] Channel overflow
- [x] Don't hide notes beyond dn in multi-track view
- [x] Soundfont paths if they don't exist
- [x] Splash is off-center
- [x] Music panel height to align everything better
- [x] Can't add spaces or underscores to alphanumeric input
- [x] ~~Audio crackling with some soundfonts??~~
- [x] Audio playing on music name input, save file input
- [x] Open file name doesn't change if you're saving or exporting
- [x] Play only works in piano roll panel
- [x] Can't delete a track
- [x] Bad zoom
- [x] Play notes in open-file-panel file name
- [x] Don't use id3 tags for wav files.
- [x] Can't change export settings after the first time you export
- [x] Bugs when saving for the first or second time??
- [x] View range is wrong if moved
- [x] View dt rounds incorrectly on reload
- [x] Copy+paste crash
- [x] Wrong gain on reload
- [x] Crash when adding a track if notes are selected
- [x] ~~We should save relative paths only~~
- [x] Wrong soundfont if there is more than one!
- [x] Playing note highlighting is very off
- [x] ~~View numbers are wrong in multitrack~~
- [x] Multitrack cuts off one early
- [x] Export panel doesn't list mp3s
- [x] Incorrect default mp3 quality
- [x] Crash when setting artist
- [x] Export settings don't save
- [x] Export seems to only include some notes sometimes
- [x] ~~Exporter can crash due to not enough samples~~
- [x] Multi track wav doesn't append silence
- [x] Incorrect sound font if you load them while we have notes
- [x] View range can get messed up bqy multitrack?
- [x] Playing line after exporting
- [x] Changing a soundfont doesn't change the preset name
- [x] Invalid bank change resets the soundfont
- [x] Opening a soundfont can crash
- [x] Illegal filename characters allowed?
- [x] ~~Can't access D:/~~
- [x] Viewable notes volume cutoff

## Misc.

- [x] Qwerty note play
- [x] Splash version
- [x] We need to see the select time range and maybe selection should be by time range.
- [x] Playing line
- [x] A faster export is probably possible

## Piano Roll Panel

- [x] Can we play notes?
- [x] Qwerty note input
- [x] Cursor to beat
- [x] Playback to beat
- [x] Cursor to Playback
- [x] Playback to cursor
- [x] Up/down tracks in multi-track
- [x] Qwerty note input TTS
- [ ] Multi-track zoom
- [ ] View should move while playing

## Tracks view

- [x] Extend to the bottom of the screen
- [x] Better colors
- [x] Note sizes in the config file
- [x] Arrows to the tracks
- [x] Disable view up/down

## Export panel

- [x] Make it
- [x] Use Popup

## Open File

- [x] Don't show full path of files
- [x] Show relative paths
- [x] Arrows

## MIDI

- [x] Automatically choose input devices
