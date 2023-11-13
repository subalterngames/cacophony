use std::sync::Arc;

use crate::{
    midi_event_queue::MidiEventQueue, types::SharedSample, Command, ExportState,
    Player, SharedMidiEventQueue, SharedSynth, SharedTimeState, SynthState, TimeState, Program
};
use common::{State, MAX_VOLUME};
use crossbeam_channel::Receiver;
use hashbrown::HashMap;
use oxisynth::{MidiEvent, SoundFont, SoundFontId, Synth};
use parking_lot::Mutex;
use std::path::{Path, PathBuf};
use std::fs::File;

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
    pub export_state: Option<ExportState>,
    /// The playback framerate.
    framerate: f32,
    /// The audio player. This is here so we don't drop it.
    _player: Option<Player>,
    /// The most recent sample.
    pub sample: SharedSample,
    synth: SharedSynth,
    midi_event_queue: SharedMidiEventQueue,
    time_state: SharedTimeState,
    soundfonts: HashMap<PathBuf, SoundFontBanks>,
    pub state: SynthState,
}

impl Conn {
    pub(crate) fn new(
        recv_export: Receiver<Option<ExportState>>,
    ) -> Self {
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
            export_state: None,
            _player: player,
            framerate,
            sample,
            synth,
            midi_event_queue,
            time_state,
            soundfonts: HashMap::default(),
            state: SynthState::default()
        }
    }

    /// Do all note-on and note-off events created by user input on this app frame.
    pub fn do_note_events(&mut self, state: &State, note_ons: &[[u8; 3]], note_offs: &[u8]) {
        if let Some(track) = state.music.get_selected_track() {
            let mut synth = self.synth.lock();
            for note_on in note_ons.iter() {
                if synth
                    .send_event(MidiEvent::NoteOn {
                        channel: track.channel,
                        key: note_on[1],
                        vel: note_on[2],
                    })
                    .is_ok()
                {}
            }
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
                Command::SetProgram { channel, path, bank_index, preset_index } => {
                    let soundfont = &self.soundfonts[path];
                    let mut banks = soundfont.banks.keys().copied().collect::<Vec<u32>>();
                    let bank = banks[*bank_index];
                    let preset = soundfont.banks[&bank][*preset_index];
                    let channel = *channel;
                    let mut synth = self.synth.lock();
                    if synth.program_select(channel, soundfont.id, bank, preset).is_ok()
                    {
                        self.set_program(channel, path, bank, preset);
                    }
                }
                Command::UnsetProgram { channel } => {
                    self.state.programs.remove(channel);
                }
                Command::SetGain { gain } => {
                    let mut synth = self.synth.lock();
                    synth.set_gain(*gain as f32 / 127.0);
                    self.state.gain = *gain;
                }
            }
        }
    }

    /// Schedule MIDI events and start to play music.
    pub fn schedule_music(&mut self, state: &State) {
        // Get the start time.
        let start = state
            .time
            .ppq_to_samples(state.time.playback, self.framerate);

        // Set the playback framerate.
        let mut synth = self.synth.lock();
        synth.set_sample_rate(self.framerate);

        // Enqueue note events.
        let mut midi_event_queue = self.midi_event_queue.lock();
        for track in state.music.midi_tracks {
            for note in track.get_playback_notes(start) {
                // Note-on event.
                midi_event_queue.enqueue(
                    state.time.ppq_to_samples(note.start, self.framerate),
                    MidiEvent::NoteOn {
                        channel: track.channel,
                        key: note.note,
                        vel: note.velocity,
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
        let mut synth = self.synth.lock();
        synth.program_select(channel, id, bank, preset).unwrap();
        self.set_program(channel, path, bank, preset);
    }

    /// Set the synthesizer program to a program.
    fn set_program(&mut self, channel: u8, path: &Path, bank: u32, preset: u8) {
        let soundfont = &self.soundfonts[path];
        // Get the bank info.
        let bank_index = soundfont.banks.keys().position(|&b| b == bank).unwrap();
        // Get the preset info.
        let preset_index = soundfont.banks[&bank].iter().position(|&p| p == preset).unwrap();
        let mut synth = self.synth.lock();
        let preset_name = synth
            .channel_preset(channel)
            .unwrap()
            .name()
            .to_string();
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
