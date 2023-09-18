use audio::{Command, Conn, SharedExporter};
use common::{MidiTrack, Music, Note, State, U64orF32, Paths};
use midly::{MetaMessage, MidiMessage, Smf, TrackEventKind, Timing};
use std::fs::read;
use std::path::Path;
use std::str::from_utf8;

pub(crate) fn import(path: &Path, state: &mut State, conn: &mut Conn, exporter: &mut SharedExporter) {
    let bytes = read(path).unwrap();
    let smf = Smf::parse(&bytes).unwrap();
    let timing = match smf.header.timing {
        Timing::Metrical(v) => v.as_int() as f32,
        Timing::Timecode(fps, t) => fps.as_f32() / t as f32
    };
    let mut music = Music::default();
    let paths = Paths::default();
    for (i, track_events) in smf.tracks.iter().enumerate() {
        // Create a new track.
        let c = i as u8;
        let mut track = MidiTrack::new(c);
        // Load the default SoundFont.
        conn.send(vec![Command::LoadSoundFont { channel: c, path: paths.default_soundfont_path.clone() }]);
        let mut time = 0;
        // A list of note-on events that need corresponding note-off messages.
        let mut note_ons = vec![];

        // Iterate through this track's events.
        for track_event in track_events {
            time += track_event.delta.as_int() as u64;
            match track_event.kind {
                TrackEventKind::Escape(_) | TrackEventKind::SysEx(_) => (),
                TrackEventKind::Meta(message) => match message {
                    MetaMessage::Copyright(data) => {
                        if let Ok(copyright) = from_utf8(data) {
                            let mut exporter = exporter.lock();
                            exporter.copyright = true;
                            exporter.metadata.artist = Some(copyright.to_string());
                        }
                    }
                    MetaMessage::Tempo(data) => {
                        state.time.bpm = U64orF32::from(60000000 / data.as_int() as u64);
                    }
                    MetaMessage::TimeSignature(n, d, c, b) => {
                        // This SHOULD always be correct. If not, there might be an error with how n is used.
                        let q = (n as f32 / 2f32.powf(d as f32)) * (timing / (c * b) as f32);
                        state.time.bpm = U64orF32::from(state.time.bpm.get_f() * q);
                    }
                    MetaMessage::Text(data) => {
                        if let Ok(text) = from_utf8(data) {
                            let mut exporter = exporter.lock();
                            exporter.metadata.comment = Some(text.to_string())
                        }
                    }
                    _ => ()
                }
                TrackEventKind::Midi { channel: _, message } => {
                    match message {
                        MidiMessage::NoteOn { key, vel } => {
                            note_ons.push((key, vel, time));
                        }
                        MidiMessage::NoteOff { key, vel } => {
                            let (index, note_on) = note_ons.iter().enumerate().filter(|(_, n)| n.0 == key).next().unwrap();
                            // Add a note.
                            track.notes.push(Note { note: note_on.0.as_int(), velocity: u8::max(vel.as_int(), note_on.1.as_int()), start: note_on.2, end: time });
                            // Remove the note-on event.
                            note_ons.remove(index);
                        }
                        // Set the preset.
                        MidiMessage::ProgramChange { program } => {
                            conn.send(vec![Command::SetProgram { channel: track.channel, path: paths.default_soundfont_path.clone(), bank_index: conn.state.programs.get(&track.channel).unwrap().bank_index, preset_index: program.as_int() as usize}]);
                        }
                        _ => ()
                    }
                }
            }
        }
        music.midi_tracks.push(track);
    }
    // Remove empty tracks.
    music.midi_tracks.retain(|t| !t.notes.is_empty());
    // Select the first track.
    if !music.midi_tracks.is_empty() {
        music.selected = Some(0);
    }
    state.music = music;
}