# TODO

## Big picture

- [ ] Test on Linux
- [ ] Test on OSX
- [ ] Help links
- [x] Multi-track view
- [x] ~~Effects~~
- [x] ~~Text ref~~
- [ ] Rewrite README
- [x] Export MIDI
- [x] Export separate tracks
- [x] Export ogg
- [x] Export mp3
- [x] ~~Export flac~~
- [x] Export chunks
- [ ] More MIDI bindings
- [ ] Test all TTS
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
- [ ] TTS is not helpful!!

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
- [ ] Audio crackling with some soundfonts??
- [x] Audio playing on music name input, save file input
- [x] Open file name doesn't change if you're saving or exporting
- [ ] Play only works in piano roll panel
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
- [ ] We should save relative paths only
- [x] Wrong soundfont if there is more than one!
- [ ] Playing note highlighting is very off
- [ ] View numbers are wrong in multitrack
- [ ] Multitrack cuts off one early
- [x] Export panel doesn't list mp3s
- [x] Incorrect default mp3 quality
- [x] Crash when setting artist
- [x] Export settings don't save
- [x] Export seems to only include some notes sometimes
- [ ] Exporter can crash due to not enough samples
- [x] Multi track wav doesn't append silence

## Misc.

- [x] Qwerty note play
- [x] Splash version
- [ ] We need to see the select time range and maybe selection should be by time range.
- [ ] View should move while playing
- [x] Playing line

## Piano Roll Panel

- [x] Can we play notes?
- [x] Qwerty note input
- [x] Cursor to beat
- [x] Playback to beat
- [x] Cursor to Playback
- [x] Playback to cursor
- [x] Up/down tracks in multi-track
- [ ] Qwerty note input TTS

## Tracks view

- [x] Extend to the bottom of the screen
- [x] Better colors
- [x] Note sizes in the config file
- [x] Arrows to the tracks
- [x] Disable view up/down

## Export panel

- [x] Make it
- [x] Use Popup