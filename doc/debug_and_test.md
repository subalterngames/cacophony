# Debug and Test

To test, just `cargo test --all`.

Sometimes when debugging, it's useful to create the same initial setup every time. To do this, you can pass input events in like this: `cargo run -- --events events.txt`

...where the contents of `events.txt` is something like:

```
NextPanel
AddTrack
EnableSoundFontPanel
SelectFile
```

See `config.ini` for a full list of input events.