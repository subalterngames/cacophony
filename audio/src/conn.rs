use std::sync::Arc;
use std::thread::spawn;

use crate::export::{ExportState, ExportType, Exportable, MultiFileSuffix};
use crate::exporter::Exporter;
use crate::SharedExportState;
use crate::{
    midi_event_queue::MidiEventQueue, types::SharedSample, Command, Player, Program,
    SharedMidiEventQueue, SharedSynth, SharedTimeState, SynthState, TimeState,
};
use common::open_file::Extension;
use common::{MidiTrack, Music, PathsState, State, Time, MAX_VOLUME};
use hashbrown::HashMap;
use oxisynth::{MidiEvent, SoundFont, SoundFontId, Synth};
use parking_lot::Mutex;
use std::fs::File;
use std::path::{Path, PathBuf};

/// Export this many bytes per decay chunk.
const DECAY_CHUNK_SIZE: usize = 2048;
///  Oxisynth usually doesn't zero out its audio. This is essentially an epsilon.
/// This is used to detect if the export is done.
const SILENCE: f32 = 1e-7;

/// A convenient wrapper for a SoundFont.
struct SoundFontBanks {
    id: SoundFontId,
    /// The banks and their presets.
    banks: HashMap<u32, Vec<u8>>,
}

impl SoundFontBanks {
    pub fn new(font: SoundFont, synth: &mut SharedSynth) -> Self {
        let mut banks: HashMap<u32, Vec<u8>> = HashMap::new();
        (0u32..129u32).for_each(|b| {
            let presets: Vec<u8> = (0u8..128)
                .filter(|p| font.preset(b, *p).is_some())
                .collect();
            if !presets.is_empty() {
                banks.insert(b, presets);
            }
        });
        let mut synth = synth.lock();
        let id = synth.add_font(font, true);
        Self { id, banks }
    }
}

/// The connects used by an external function.
pub struct Conn {
    /// The current export state, if any.
    pub export_state: SharedExportState,
    /// The playback framerate.
    pub framerate: f32,
    /// The audio player. This is here so we don't drop it.
    _player: Option<Player>,
    /// The most recent sample.
    pub sample: SharedSample,
    synth: SharedSynth,
    midi_event_queue: SharedMidiEventQueue,
    time_state: SharedTimeState,
    soundfonts: HashMap<PathBuf, SoundFontBanks>,
    pub state: SynthState,
    pub exporter: Exporter,
}

impl Default for Conn {
    fn default() -> Self {
        // Set the synthesizer.
        let mut synth = Synth::default();
        synth.set_gain(MAX_VOLUME as f32);
        let synth = Arc::new(Mutex::new(synth));

        // Create other shared data.
        let time_state = Arc::new(Mutex::new(TimeState::default()));
        let midi_event_queue = Arc::new(Mutex::new(MidiEventQueue::default()));
        let sample = Arc::new(Mutex::new((0.0, 0.0)));

        // Create the player.
        let player_synth = Arc::clone(&synth);
        let player_time_state = Arc::clone(&time_state);
        let player_midi_event_queue = Arc::clone(&midi_event_queue);
        let player_sample = Arc::clone(&sample);
        let player = Player::new(
            player_midi_event_queue,
            player_time_state,
            player_synth,
            player_sample,
        );

        // Get the framerate.
        let framerate = match &player {
            Some(player) => player.framerate as f32,
            None => 0.0,
        };
        Self {
            export_state: Arc::new(Mutex::new(ExportState::NotExporting)),
            _player: player,
            framerate,
            sample,
            synth,
            midi_event_queue,
            time_state,
            soundfonts: HashMap::default(),
            state: SynthState::default(),
            exporter: Exporter::default(),
        }
    }
}

impl Conn {
    /// Do all note-on events created by user input on this app frame.
    pub fn note_ons(&mut self, state: &State, note_ons: &[[u8; 3]]) {
        if let Some(track) = state.music.get_selected_track() {
            let mut synth = self.synth.lock();
            synth.set_sample_rate(self.framerate);
            let gain = track.gain as f32 / MAX_VOLUME as f32;
            for note_on in note_ons.iter() {
                if synth
                    .send_event(MidiEvent::NoteOn {
                        channel: track.channel,
                        key: note_on[1],
                        vel: (note_on[2] as f32 * gain) as u8,
                    })
                    .is_ok()
                {}
            }
        }
    }

    /// Do all note-off events created by user input on this app frame.
    pub fn note_offs(&mut self, state: &State, note_offs: &[u8]) {
        if let Some(track) = state.music.get_selected_track() {
            let mut synth = self.synth.lock();
            synth.set_sample_rate(self.framerate);
            for note_off in note_offs.iter() {
                if synth
                    .send_event(MidiEvent::NoteOff {
                        channel: track.channel,
                        key: *note_off,
                    })
                    .is_ok()
                {}
            }
        }
    }

    pub fn do_commands(&mut self, commands: &[Command]) {
        for command in commands.iter() {
            match command {
                Command::LoadSoundFont { channel, path } => {
                    match &self.soundfonts.get(path) {
                        // We already loaded this font.
                        Some(_) => self.set_program_default(*channel, path),
                        // Load the font.
                        None => match SoundFont::load(&mut File::open(path).unwrap()) {
                            Ok(font) => {
                                let banks = SoundFontBanks::new(font, &mut self.synth);
                                self.soundfonts.insert(path.clone(), banks);
                                // Set the default program.
                                self.set_program_default(*channel, path);
                                // Restore the other programs.
                                let programs = self.state.programs.clone();
                                let mut synth = self.synth.lock();
                                for program in programs.iter().filter(|p| p.0 != channel) {
                                    synth
                                        .program_select(
                                            *program.0,
                                            self.soundfonts[&program.1.path].id,
                                            program.1.bank,
                                            program.1.preset,
                                        )
                                        .unwrap();
                                }
                            }
                            Err(error) => {
                                panic!("Failed to load SoundFont: {:?}", error)
                            }
                        },
                    }
                }
                Command::SetProgram {
                    channel,
                    path,
                    bank_index,
                    preset_index,
                } => {
                    let soundfont = &self.soundfonts[path];
                    let banks = soundfont.banks.keys().copied().collect::<Vec<u32>>();
                    let bank = banks[*bank_index];
                    let preset = soundfont.banks[&bank][*preset_index];
                    let channel = *channel;
                    self.set_program(channel, path, bank, preset, soundfont.id);
                }
                Command::UnsetProgram { channel } => {
                    self.state.programs.remove(channel);
                }
                Command::SetGain { gain } => {
                    let mut synth = self.synth.lock();
                    synth.set_gain(*gain as f32 / MAX_VOLUME as f32);
                    self.state.gain = *gain;
                }
            }
        }
    }

    /// Start to play music if music isn't playing. Stop music if music is playing.
    pub fn set_music(&mut self, state: &State) {
        let music = self.time_state.lock().music.clone();
        if music {
            self.stop_music(&state.music)
        } else {
            self.start_music(state)
        }
    }

    pub fn exporting(&self) -> bool {
        *self.export_state.lock() != ExportState::NotExporting
    }

    /// Schedule MIDI events and start to play music.
    fn start_music(&mut self, state: &State) {
        // Get the start time.
        let start = state
            .time
            .ppq_to_samples(state.time.playback, self.framerate);

        // Set the playback framerate.
        let mut synth = self.synth.lock();
        synth.set_sample_rate(self.framerate);

        // Enqueue note events.
        let mut midi_event_queue = self.midi_event_queue.lock();
        for track in state.music.midi_tracks.iter() {
            let gain = track.gain as f32 / MAX_VOLUME as f32;
            for note in track.get_playback_notes(start) {
                // Note-on event.
                midi_event_queue.enqueue(
                    state.time.ppq_to_samples(note.start, self.framerate),
                    MidiEvent::NoteOn {
                        channel: track.channel,
                        key: note.note,
                        vel: (note.velocity as f32 * gain) as u8,
                    },
                );
                // Note-off event.
                midi_event_queue.enqueue(
                    state.time.ppq_to_samples(note.end, self.framerate),
                    MidiEvent::NoteOff {
                        channel: track.channel,
                        key: note.note,
                    },
                );
            }
        }
        // Sort the events by start time.
        midi_event_queue.sort();

        // Set time itself.
        let mut time_state = self.time_state.lock();
        time_state.music = true;
        time_state.time = Some(start);
    }

    /// Stop ongoing music.
    fn stop_music(&mut self, music: &Music) {
        let mut synth = self.synth.lock();
        for track in music.midi_tracks.iter() {
            if synth
                .send_event(MidiEvent::AllNotesOff {
                    channel: track.channel,
                })
                .is_ok()
            {}
            if synth
                .send_event(MidiEvent::AllSoundOff {
                    channel: track.channel,
                })
                .is_ok()
            {}
        }
        let mut time_state = self.time_state.lock();
        time_state.music = false;
        time_state.time = None;
    }

    /// Set the synthesizer program to a default program.
    fn set_program_default(&mut self, channel: u8, path: &Path) {
        let soundfont = &self.soundfonts[path];
        // Get the bank info.
        let mut banks: Vec<u32> = soundfont.banks.keys().copied().collect();
        banks.sort();
        let bank = banks[0];
        let preset = soundfont.banks[&bank][0];
        // Select the default program.
        let id = self.soundfonts[path].id;
        self.set_program(channel, path, bank, preset, id);
    }

    /// Set the synthesizer program to a program.
    fn set_program(&mut self, channel: u8, path: &Path, bank: u32, preset: u8, id: SoundFontId) {
        let mut synth = self.synth.lock();
        if synth.program_select(channel, id, bank, preset).is_ok() {
            let soundfont = &self.soundfonts[path];
            // Get the bank info.
            let bank_index = soundfont.banks.keys().position(|&b| b == bank).unwrap();
            // Get the preset info.
            let preset_index = soundfont.banks[&bank]
                .iter()
                .position(|&p| p == preset)
                .unwrap();
            let synth = self.synth.lock();
            let preset_name = synth.channel_preset(channel).unwrap().name().to_string();
            let num_banks = soundfont.banks.len();
            let num_presets = soundfont.banks[&bank].len();
            let program = Program {
                path: path.to_path_buf(),
                num_banks,
                bank_index,
                bank,
                num_presets,
                preset_index,
                preset_name,
                preset,
            };
            // Remember the program.
            self.state.programs.insert(channel, program);
        }
    }

    pub fn start_export(&mut self, state: &State, paths_state: &PathsState) {
        let gain = self.state.gain as f32 / MAX_VOLUME as f32;
        let mut exportables = vec![];
        let tracks = state.music.get_playable_tracks();
        self.set_export_framerate();

        // Export each track as a separate file.
        if self.exporter.multi_file {
            for track in tracks {
                let mut events = MidiEventQueue::default();
                let mut t1 = 0;
                self.enqueue_track_events(track, &state.time, &mut events, &mut t1, gain);
                events.sort();
                let suffix = Some(self.get_export_file_suffix(track));
                // Add an exportable.
                exportables.push(Exportable {
                    events: events,
                    total_samples: t1,
                    suffix,
                });
            }
        }
        // Export all tracks combined.
        else {
            for track in tracks {
                let mut events = MidiEventQueue::default();
                let mut t1 = 0;
                self.enqueue_track_events(track, &state.time, &mut events, &mut t1, gain);
                events.sort();
                // Add an exportable.
                exportables.push(Exportable {
                    events: events,
                    total_samples: t1,
                    suffix: None,
                });
            }
        }

        let export_state = Arc::clone(&self.export_state);
        let synth = Arc::clone(&self.synth);
        let exporter = self.exporter.clone();
        let path = paths_state.exports.get_path();
        spawn(move || Self::export(exportables, export_state, synth, exporter, path));
    }

    fn enqueue_track_events(
        &self,
        track: &MidiTrack,
        time: &Time,
        events: &mut MidiEventQueue,
        t1: &mut u64,
        gain: f32,
    ) {
        // Turn off the sound.
        events.enqueue(
            0,
            MidiEvent::AllSoundOff {
                channel: track.channel,
            },
        );
        for note in track.notes.iter() {
            // Note-on.
            events.enqueue(
                time.ppq_to_samples(note.start, self.framerate),
                MidiEvent::NoteOn {
                    channel: track.channel,
                    key: note.note,
                    vel: (note.velocity as f32 * gain) as u8,
                },
            );
            let end = time.ppq_to_samples(note.start, self.framerate);
            // This is the last known event.
            if *t1 < end {
                *t1 = end;
            }
            events.enqueue(
                end,
                MidiEvent::NoteOff {
                    channel: track.channel,
                    key: note.note,
                },
            );
        }
    }

    fn export(
        mut exportables: Vec<Exportable>,
        export_state: SharedExportState,
        synth: SharedSynth,
        exporter: Exporter,
        path: PathBuf,
    ) {
        let mut decay_left = [0.0f32; DECAY_CHUNK_SIZE];
        let mut decay_right = [0.0f32; DECAY_CHUNK_SIZE];
        let extension: Extension = exporter.export_type.get().into();
        for exportable in exportables.iter_mut() {
            let total_samples = exportable.total_samples;
            // Get the audio buffers.
            let mut left = vec![0.0f32; total_samples as usize];
            let mut right = vec![0.0f32; total_samples as usize];
            // Set the initial wav export state.
            Self::set_export_state_wav(exportable, &export_state, 0);
            let mut synth = synth.lock();
            let mut t0 = 0;
            // Process each event.
            while let Some(t) = exportable.events.get_next_time() {
                // Get and send each event at this time.
                for event in exportable.events.dequeue(t).iter() {
                    if synth.send_event(*event).is_ok() {}
                }
                // Write a sample.
                if t0 == t {
                    let sample = synth.read_next();
                    let t = t as usize;
                    left[t] = sample.0;
                    right[t] = sample.1;
                }
                // Write a block of samples.
                else {
                    let t0u = t0 as usize;
                    let t1u = t as usize;
                    synth.write((left[t0u..t1u].as_mut(), right[t0u..t1u].as_mut()));
                }
                // Set the export state.
                Self::set_export_state_wav(exportable, &export_state, t);
                // Set the start time.
                t0 = t;
            }
            // Append decaying silence.
            Self::set_export_state(&export_state, ExportState::AppendingSilence);
            let mut decaying = true;
            while decaying {
                // Write to the decay chunks.
                synth.write((decay_left.as_mut_slice(), decay_right.as_mut_slice()));
                // If the decay chunks are totally silent then we're not decaying anymore.
                decaying = decay_left.iter().any(|s| s.abs() > SILENCE)
                    || decay_right.iter().any(|s| s.abs() > SILENCE);
                // Add the silence.
                if decaying {
                    left.extend_from_slice(&decay_left);
                    right.extend_from_slice(&decay_right);
                }
            }
            // Convert.
            Self::set_export_state(&export_state, ExportState::WritingToDisk);
            let suffix = exportable.suffix.clone().unwrap();
            let path = if exporter.multi_file {
                path.parent()
                    .unwrap()
                    .join(format!(
                        "{}_{}{}",
                        path.file_stem().unwrap().to_str().unwrap(),
                        suffix,
                        extension.to_str(true)
                    ))
                    .to_path_buf()
            } else {
                path.clone()
            };
            let audio = [left, right];
            match &exporter.export_type.get() {
                ExportType::Mid => {
                    panic!("Tried exporting a .mid from the synthesizer")
                }
                // Export to a .wav file.
                ExportType::Wav => {
                    exporter.wav(&path, &audio);
                }
                ExportType::MP3 => {
                    exporter.mp3(&path, &audio);
                }
                ExportType::Ogg => {
                    exporter.ogg(&path, &audio);
                }
            }
            // Done.
            Self::set_export_state(&export_state, ExportState::Done);
        }
        Self::set_export_state(&export_state, ExportState::NotExporting);
    }

    /// Set the exporter's framerate.
    fn set_export_framerate(&mut self) {
        let framerate = self.exporter.framerate.get_f();
        let mut synth = self.synth.lock();
        synth.set_sample_rate(framerate);
    }

    /// Set the number of exported wav samples.
    fn set_export_state_wav(
        exportable: &Exportable,
        export_state: &SharedExportState,
        exported_samples: u64,
    ) {
        let mut export_state = export_state.lock();
        *export_state = ExportState::WritingWav {
            total_samples: exportable.total_samples,
            exported_samples,
        }
    }

    /// Set the export state.
    fn set_export_state(export_state: &SharedExportState, state: ExportState) {
        let mut export_state = export_state.lock();
        *export_state = state;
    }

    fn get_export_file_suffix(&self, track: &MidiTrack) -> String {
        // Get the path for this track.
        match self.exporter.multi_file_suffix.get() {
            MultiFileSuffix::Channel => track.channel.to_string(),
            MultiFileSuffix::Preset => self
                .state
                .programs
                .get(&track.channel)
                .unwrap()
                .preset_name
                .clone(),
            MultiFileSuffix::ChannelAndPreset => format!(
                "{}_{}",
                track.channel,
                self.state.programs.get(&track.channel).unwrap().preset_name
            ),
        }
    }
}
