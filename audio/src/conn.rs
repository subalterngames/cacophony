use crate::decayer::Decayer;
use crate::export::{ExportState, ExportType, Exportable, MultiFileSuffix};
use crate::exporter::Exporter;
use crate::play_state::PlayState;
use crate::soundfont_banks::SoundFontBanks;
use crate::types::SharedPlayState;
use crate::SharedExportState;
use crate::{
    event_queue::EventQueue, types::SharedSample, Command, Player, Program, SharedEventQueue,
    SharedSynth, SynthState,
};
use common::effect::Effect;
use common::open_file::Extension;
use common::{MidiTrack, Music, Note, PathsState, State, MAX_VOLUME};
use hashbrown::HashMap;
use oxisynth::{MidiEvent, SoundFont, SoundFontId, Synth};
use parking_lot::Mutex;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread::spawn;

/// The connects used by an external function.
pub struct Conn {
    /// The current export state, if any.
    pub export_state: SharedExportState,
    /// The playback framerate.
    pub framerate: f32,
    /// The audio player. This is here so we don't drop it.
    _player: Option<Player>,
    /// The most recent sample.
    /// `render::MainMenu` uses this to for its power bars.
    pub sample: SharedSample,
    /// A shared Oxisynth synthesizer.
    /// The `Conn` uses this to send MIDI events and export.
    /// The `Player` uses this to write samples to the output buffer.
    synth: SharedSynth,
    /// A queue of scheduled events.
    /// The `Conn` can add to this.
    /// The `Player` can read this and remove events.
    event_queue: SharedEventQueue,
    /// A HashMap of loaded SoundFonts. Key = The path to a .sf2 file.
    soundfonts: HashMap<PathBuf, SoundFontBanks>,
    /// Metadata for all SoundFont programs.
    pub state: SynthState,
    /// Export settings.
    pub exporter: Exporter,
    /// A flag that `Player` uses to decide how to write samples to the output buffer.
    pub play_state: SharedPlayState,
}

impl Default for Conn {
    fn default() -> Self {
        // Set the synthesizer.
        let mut synth = Synth::default();
        synth.set_gain(1.0);
        let synth = Arc::new(Mutex::new(synth));

        // Create other shared data.
        let event_queue = Arc::new(Mutex::new(EventQueue::default()));
        let sample = Arc::new(Mutex::new((0.0, 0.0)));
        let play_state = Arc::new(Mutex::new(PlayState::NotPlaying));

        // Create the player.
        let player_synth = Arc::clone(&synth);
        let player_event_queue = Arc::clone(&event_queue);
        let player_sample = Arc::clone(&sample);
        let player_play_state = Arc::clone(&play_state);
        let player = Player::new(
            player_event_queue,
            player_synth,
            player_sample,
            player_play_state,
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
            event_queue,
            soundfonts: HashMap::default(),
            state: SynthState::default(),
            exporter: Exporter::default(),
            play_state,
        }
    }
}

impl Conn {
    /// Do all note-on events created by user input on this app frame.
    pub fn note_ons(&mut self, state: &State, note_ons: &[[u8; 3]]) {
        if let Some(track) = state.music.get_selected_track() {
            if !note_ons.is_empty() {
                let mut synth = self.synth.lock();
                let gain = track.gain as f32 / MAX_VOLUME as f32;
                for note_on in note_ons.iter() {
                    let _ = synth.send_event(MidiEvent::NoteOn {
                        channel: track.channel,
                        key: note_on[1],
                        vel: (note_on[2] as f32 * gain) as u8,
                    });
                }
                // Play audio.
                let mut play_state = self.play_state.lock();
                *play_state = PlayState::Decaying;
            }
        }
    }

    /// Do all note-off events created by user input on this app frame.
    pub fn note_offs(&mut self, state: &State, note_offs: &[u8]) {
        if let Some(track) = state.music.get_selected_track() {
            if !note_offs.is_empty() {
                let mut synth = self.synth.lock();
                for note_off in note_offs.iter() {
                    let _ = synth.send_event(MidiEvent::NoteOff {
                        channel: track.channel,
                        key: *note_off,
                    });
                }
            }
        }
    }

    /// Execute a slice of commands sent from `io`.
    pub fn do_commands(&mut self, commands: &[Command]) {
        for command in commands.iter() {
            match command {
                Command::LoadSoundFont { channel, path } => {
                    match &self.soundfonts.get(path) {
                        // We already loaded this font.
                        Some(_) => {
                            self.set_program_default(*channel, path);
                        }
                        // Load the font.
                        None => match SoundFont::load(&mut File::open(path).unwrap()) {
                            Ok(font) => {
                                let banks = SoundFontBanks::new(font, &mut self.synth);
                                self.soundfonts.insert(path.clone(), banks);
                                // Set the default program.
                                self.set_program_default(*channel, path);
                                // Restore the other programs.
                                let programs = self.state.programs.clone();
                                for program in programs.iter().filter(|p| p.0 != channel) {
                                    if self.soundfonts.contains_key(&program.1.path) {
                                        let mut synth = self.synth.lock();
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
        let play_state = *self.play_state.lock();
        match play_state {
            PlayState::NotPlaying => self.start_music(state),
            _ => self.stop_music(&state.music),
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
        drop(synth);

        // Enqueue note events.
        let mut event_queue = self.event_queue.lock();
        let mut end_time = 0;
        for track in state.music.get_playable_tracks().iter() {
            let notes = track.get_playback_notes(state.time.playback);
            let effects = track
                .effects
                .iter()
                .filter(|e| e.time >= state.time.playback)
                .copied()
                .collect::<Vec<Effect>>();
            event_queue.enqueue(
                track.channel,
                &self.state.programs[&track.channel],
                &notes,
                &effects,
                &state.time,
                self.framerate,
                &mut end_time,
            );
        }
        // Sort the events by start time.
        event_queue.sort();
        drop(event_queue);

        // Play music.
        let mut play_state = self.play_state.lock();
        *play_state = PlayState::Playing(start);
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
                && synth
                    .send_event(MidiEvent::AllSoundOff {
                        channel: track.channel,
                    })
                    .is_ok()
            {}
        }
        drop(synth);
        // Let the audio decay.
        let mut play_state = self.play_state.lock();
        *play_state = PlayState::Decaying;
    }

    /// Set the synthesizer program to a default program.
    fn set_program_default(&mut self, channel: u8, path: &Path) {
        let mut synth = self.synth.lock();
        // Create a new program.
        let soundfont = &self.soundfonts[path];
        let program = Program::new(channel, &synth, path, soundfont);
        // Set the program.
        let _ = synth
            .program_select(channel, soundfont.id, program.bank, program.preset)
            .is_ok();
        // Store the program.
        self.state.programs.insert(channel, program);
    }

    /// Set the synthesizer program to a program.
    fn set_program(&mut self, channel: u8, path: &Path, bank: u32, preset: u8, id: SoundFontId) {
        let mut synth = self.synth.lock();
        let _ = synth.program_select(channel, id, bank, preset).is_ok();
        let soundfont = &self.soundfonts[path];
        let program = self.state.programs.get_mut(&channel).unwrap();
        // Set the program to a new SoundFont.
        if program.path != path {
            *program = Program::new(channel, &synth, path, soundfont);
        }
        // Set the bank.
        program.bank_index = soundfont.banks.keys().position(|&b| b == bank).unwrap();
        program.bank = bank;
        // Set the preset.
        program.preset = preset;
        program.preset_name = synth.channel_preset(channel).unwrap().name().to_string();
        program.preset_index = soundfont.banks[&bank]
            .iter()
            .position(|&p| p == preset)
            .unwrap();
    }

    pub fn start_export(&mut self, state: &State, paths_state: &PathsState) {
        let mut exportables = vec![];
        let tracks = state.music.get_playable_tracks();
        self.set_export_framerate();

        // Export each track as a separate file.
        if self.exporter.multi_file {
            for track in tracks {
                let mut events = EventQueue::default();
                let mut t1 = 0;
                let notes = Self::get_exportable_notes(track);
                events.enqueue(
                    track.channel,
                    &self.state.programs[&track.channel],
                    &notes,
                    &track.effects,
                    &state.time,
                    self.framerate,
                    &mut t1,
                );
                events.sort();
                let suffix = Some(self.get_export_file_suffix(track));
                // Add an exportable.
                exportables.push(Exportable {
                    events,
                    total_samples: t1,
                    suffix,
                });
            }
        }
        // Export all tracks combined.
        else {
            let mut t1 = 0;
            let mut events = EventQueue::default();
            for track in tracks {
                let notes = Self::get_exportable_notes(track);
                events.enqueue(
                    track.channel,
                    &self.state.programs[&track.channel],
                    &notes,
                    &track.effects,
                    &state.time,
                    self.framerate,
                    &mut t1,
                );
            }
            events.sort();
            // Add an exportable.
            exportables.push(Exportable {
                events,
                total_samples: t1,
                suffix: None,
            });
        }

        let export_state = Arc::clone(&self.export_state);
        let synth = Arc::clone(&self.synth);
        let exporter = self.exporter.clone();
        let path = paths_state.exports.get_path();
        let player_framerate = self.framerate;
        spawn(move || {
            Self::export(
                exportables,
                export_state,
                synth,
                exporter,
                path,
                player_framerate,
            )
        });
    }

    fn export(
        mut exportables: Vec<Exportable>,
        export_state: SharedExportState,
        synth: SharedSynth,
        exporter: Exporter,
        path: PathBuf,
        player_framerate: f32,
    ) {
        let mut decayer = Decayer::default();
        let extension: Extension = exporter.export_type.get().into();
        for exportable in exportables.iter_mut() {
            let total_samples = exportable.total_samples;
            // Get the audio buffers.
            let mut left = vec![0.0f32; total_samples as usize];
            let mut right = vec![0.0f32; total_samples as usize];
            // Set the initial wav export state.
            Self::set_export_state_wav(exportable, &export_state, 0);
            let mut synth = synth.lock();
            for t in 0..total_samples {
                // Get and send each event at this time.
                for event in exportable.events.dequeue(t).iter() {
                    event.occur(&mut synth);
                }
                // Set the export state.
                Self::set_export_state_wav(exportable, &export_state, t);
                let t = t as usize;
                (left[t], right[t]) = synth.read_next();
            }
            // Append decaying silence.
            Self::set_export_state(&export_state, ExportState::AppendingDecay);
            decayer.decaying = true;
            while decayer.decaying {
                decayer.decay_two_channels(&mut left, &mut right, &mut synth);
            }
            // Convert.
            Self::set_export_state(&export_state, ExportState::WritingToDisk);
            let filename = path.file_stem().unwrap().to_str().unwrap();
            let extension = extension.to_str(true);
            let path = match &exportable.suffix {
                Some(suffix) => path
                    .parent()
                    .unwrap()
                    .join(format!("{}_{}{}", filename, suffix, extension))
                    .to_path_buf(),
                None => path
                    .parent()
                    .unwrap()
                    .join(format!("{}{}", filename, extension))
                    .to_path_buf(),
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
                ExportType::Flac => exporter.flac(&path, &audio),
            }
            // Done.
            Self::set_export_state(&export_state, ExportState::Done);
        }
        Self::set_export_state(&export_state, ExportState::NotExporting);
        synth.lock().set_sample_rate(player_framerate);
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

    fn get_exportable_notes(track: &MidiTrack) -> Vec<Note> {
        let gain = track.get_gain_f();
        let mut notes = vec![];
        for note in track.notes.iter() {
            let mut n1 = *note;
            n1.velocity = (n1.velocity as f32 * gain) as u8;
            notes.push(n1);
        }
        notes
    }
}
