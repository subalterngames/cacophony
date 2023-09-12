use audio::SharedExporter;
use common::{MidiTrack, Music, Note, State, U64orF32};
use midly::{MetaMessage, MidiMessage, Smf, TrackEventKind};
use std::fs::read;
use std::path::Path;
use std::str::from_utf8;

pub(crate) fn import(path: &Path, state: &mut State, exporter: &mut SharedExporter) {
    let bytes = read(path).unwrap();
    let smf = Smf::parse(&bytes).unwrap();
    let mut music = Music::default();
    for (i, track_events) in smf.tracks.iter().enumerate() {
        let mut track = MidiTrack::new(i as u8);
        let mut time = 0;
        let mut note_ons = vec![];
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
                        let quarter_note = 2u8.pow(d as u32) / 4;
                        state.time.bpm = U64orF32::from(state.time.bpm.get_u() * quarter_note as u64);
                    }
                    MetaMessage::Text(data) => {
                        if let Ok(text) = from_utf8(data) {
                            let mut exporter = exporter.lock();
                            exporter.metadata.comment = Some(text.to_string())
                        }
                    }
                    _ => {
                        println!("meta message {:?}", track_event)
                    }
                }
                TrackEventKind::Midi { channel, message } => {
                    track.channel = channel.as_int();
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
                        _ => println!("midi message {:?}", track_event)
                    }
                }
            }
        }
        music.midi_tracks.push(track);
    }
    state.music = music;
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::import;
    use common::State;
}