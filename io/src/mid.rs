use audio::SynthState;
use common::*;
use common::export_settings::Mid;
use ghakuf::messages::*;
use ghakuf::writer::*;
use std::path::Path;
use text::Text;

const PULSE: u64 = 1;

/// A MIDI note contains a note and some other useful information.
struct MidiNote {
    /// The `Note`.
    note: Note,
    /// The channel of the note's track.
    channel: u8,
}

impl MidiNote {
    fn new(note: &Note, channel: u8) -> Self {
        Self {
            note: *note,
            channel,
        }
    }
}

/// Convert internal audio commands to a .mid file.
///
/// - `path` Output to this path.
/// - `music` This is what we're saving.
/// - `synth_state` We need this for its present names.
/// - `text` This is is used for metadata.
/// - `export_settings` .mid export settings.
pub(crate) fn to_mid(path: &Path, music: &Music, time: &Time, synth_state: &SynthState, text: &Text, export_settings: &Mid) {
    // Gather all notes.
    let mut notes: Vec<MidiNote> = vec![];
    for track in music.midi_tracks.iter() {
        notes.extend(track.notes.iter().map(|n| MidiNote::new(n, track.channel)));
    }
    // End here if there are no notes.
    if notes.is_empty() {
        return;
    }

    // Set the name of the music.
    let mut messages = vec![Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::TextEvent,
        data: music.name.as_bytes().to_vec(),
    }];
    // Set the instrument names.
    for program in synth_state.programs.values() {
        messages.push(Message::MetaEvent {
            delta_time: 0,
            event: MetaEvent::InstrumentName,
            data: program.preset_name.as_bytes().to_vec(),
        });
    }
    // Set the tempo.
    let tempo = 60000000 / time.bpm.get_u();
    messages.push(Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::SetTempo,
        data: [(tempo >> 16) as u8, (tempo >> 8) as u8, tempo as u8].to_vec(),
    });

    // Sort the notes by start time.
    notes.sort_by(|a, b| a.note.start.cmp(&b.note.start));
    // Get the end time.
    let t1 = notes.iter().map(|n| n.note.end).max().unwrap();

    // Get the beat time of one pulse.
    // This is the current time.
    let mut t = 0;

    // The delta-time since the last event.
    let mut dt = 0;

    // Maybe this should be a for loop.
    while t < t1 {
        // Are there any note-on events?
        for note in notes.iter().filter(|n| n.note.start == t) {
            // Note-on.
            messages.push(Message::MidiEvent {
                delta_time: get_delta_time(&mut dt),
                event: MidiEvent::NoteOn {
                    ch: note.channel,
                    note: note.note.note,
                    velocity: note.note.velocity,
                },
            });
        }
        // Are there any note-off events?
        for note in notes.iter().filter(|n| n.note.end == t) {
            // Note-off.
            messages.push(Message::MidiEvent {
                delta_time: get_delta_time(&mut dt),
                event: MidiEvent::NoteOff {
                    ch: note.channel,
                    note: note.note.note,
                    velocity: note.note.velocity,
                },
            });
        }
        // Increment the time and the delta-time.
        t += PULSE;
        dt += PULSE;
    }
    // Track end.
    messages.push(Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::EndOfTrack,
        data: vec![],
    });
    // Write.
    let mut writer = Writer::new();
    writer.running_status(true);
    for message in &messages {
        writer.push(message);
    }
    if let Err(error) = writer.write(path) {
        panic!("Error writing {:?} {:?}", path, error);
    }
}

/// Converts a PPQ value into a MIDI time delta and resets `ppq` to zero.
fn get_delta_time(ppq: &mut u64) -> u32 {
    // Get the dt.
    let dt = (*ppq as f32 / PPQ_F) as u32;
    // Reset the PPQ value.
    *ppq = 0;
    dt
}
