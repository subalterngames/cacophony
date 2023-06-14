use crate::{AudioMessage, Command, CommandsMessage, ExportState, Program, SynthState, TimeState};
use crossbeam_channel::{Receiver, Sender};
use hashbrown::HashMap;
use oxisynth::{MidiEvent, SoundFont, SoundFontId, Synth};
use riff_wave::*;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

const F32_TO_I16: f32 = 32767.5;

/// A convenient wrapper for a SoundFont.
struct SoundFontBanks {
    id: SoundFontId,
    /// The banks and their presets.
    banks: HashMap<u32, Vec<u8>>,
}

impl SoundFontBanks {
    pub fn new(font: SoundFont, synth: &mut Synth) -> Self {
        let mut banks: HashMap<u32, Vec<u8>> = HashMap::new();
        (0u32..128u32).for_each(|b| {
            let presets: Vec<u8> = (0u8..128)
                .filter(|p| font.preset(b, *p).is_some())
                .collect();
            if !presets.is_empty() {
                banks.insert(b, presets);
            }
        });
        Self {
            id: synth.add_font(font, true),
            banks,
        }
    }
}

/// A queued MIDI event.
struct QueuedEvent {
    /// The event time.
    time: u64,
    /// The event.
    event: MidiEvent,
}

/// Synthesize audio.
///
/// - A list of `Command` can be received from the `Conn`. If received, the `Synthesizer` executes the commands and sends a `SynthState` to the `Conn`.
/// - Per frame, the `Synthesizer` reads audio from its synthesizer and tries to send a sample to the `Player` and a `TimeState` to the `Conn`.
pub(crate) struct Synthesizer {
    /// The synthesizer.
    synth: Synth,
    /// A map of the SoundFonts and their banks. Key = Path.
    soundfonts: HashMap<PathBuf, SoundFontBanks>,
    /// A list of queued MIDI events.
    events_queue: Vec<QueuedEvent>,
    /// If true, we're ready to receive more commands.
    ready: bool,
    /// The state of the synthesizer.
    state: SynthState,
    /// The export state.
    export_state: Option<ExportState>,
    /// The export file writer.
    exporter: Option<WaveWriter<File>>,
}

impl Synthesizer {
    /// Start the synthesizer loop.
    ///
    /// - `recv_commands` Receive commands from the conn.
    /// - `send_audio` Send audio samples to the player.
    /// - `send_state` Send a state to the conn.
    /// - `send_export` Send audio samples to an exporter.
    /// - `send_time` Send the time to the conn.
    pub(crate) fn start(
        recv_commands: Receiver<CommandsMessage>,
        send_audio: Sender<AudioMessage>,
        send_state: Sender<SynthState>,
        send_export: Sender<ExportState>,
        send_time: Sender<TimeState>,
    ) {
        // Create the synthesizer.
        let mut s = Synthesizer {
            synth: Synth::default(),
            soundfonts: HashMap::new(),
            events_queue: vec![],
            ready: true,
            state: SynthState::default(),
            exporter: None,
            export_state: None,
        };
        loop {
            if s.ready {
                // Try to receive commands.
                match recv_commands.try_recv() {
                    Err(_) => (),
                    Ok(commands) => {
                        s.ready = false;
                        for command in commands.iter() {
                            match command {
                                Command::PlayMusic { time } => {
                                    s.events_queue.clear();
                                    s.state.time.music = true;
                                    s.state.time.time = Some(*time);
                                }
                                // Stop all notes.
                                Command::StopMusic => {
                                    s.state.programs.keys().for_each(|c| {
                                        Synthesizer::send_event(
                                            MidiEvent::AllNotesOff { channel: *c },
                                            &mut s.synth,
                                        )
                                    });
                                    // Clear the queue of commands.
                                    s.events_queue.clear();
                                    // Stop the time.
                                    s.state.time.music = false;
                                    s.state.time.time = None;
                                }
                                // Schedule a stop-all event.
                                Command::StopMusicAt { time } => {
                                    s.state.programs.keys().for_each(|c| {
                                        s.events_queue.push(QueuedEvent {
                                            time: *time,
                                            event: MidiEvent::AllNotesOff { channel: *c },
                                        });
                                    })
                                }
                                // Turn off all sound.
                                Command::SoundOff => {
                                    s.state.programs.keys().for_each(|c| {
                                        Synthesizer::send_event(
                                            MidiEvent::AllSoundOff { channel: *c },
                                            &mut s.synth,
                                        )
                                    });
                                }
                                // Note-on ASAP. Schedule a note-off as well.
                                Command::NoteOn {
                                    channel,
                                    key,
                                    velocity,
                                    duration,
                                } => {
                                    let ch = *channel;
                                    let k = *key;
                                    Synthesizer::send_event(
                                        MidiEvent::NoteOn {
                                            channel: ch,
                                            key: k,
                                            vel: *velocity,
                                        },
                                        &mut s.synth,
                                    );
                                    s.state.time.time = Some(0);
                                    // Queue a note-off event.
                                    s.events_queue.push(QueuedEvent {
                                        time: s.state.time() + duration,
                                        event: MidiEvent::NoteOff {
                                            channel: ch,
                                            key: k,
                                        },
                                    });
                                }
                                // Schedule a note-on and a note-off.
                                Command::NoteOnAt {
                                    channel,
                                    key,
                                    velocity,
                                    time,
                                    duration,
                                } => {
                                    let channel = *channel;
                                    let key = *key;
                                    s.events_queue.push(QueuedEvent {
                                        time: *time,
                                        event: MidiEvent::NoteOn {
                                            channel,
                                            key,
                                            vel: *velocity,
                                        },
                                    });
                                    s.events_queue.push(QueuedEvent {
                                        time: *time + duration,
                                        event: MidiEvent::NoteOff { channel, key },
                                    });
                                }
                                // Note-off ASAP.
                                Command::NoteOff { channel, key } => Synthesizer::send_event(
                                    MidiEvent::NoteOff {
                                        channel: *channel,
                                        key: *key,
                                    },
                                    &mut s.synth,
                                ),
                                // Program select.
                                Command::SetProgram {
                                    channel,
                                    path,
                                    bank_index,
                                    preset_index,
                                } => {
                                    let sf = &s.soundfonts[path];
                                    let mut banks = sf.banks.keys().copied().collect::<Vec<u32>>();
                                    banks.sort();
                                    let bank = banks[*bank_index];
                                    let preset = sf.banks[&bank][*preset_index];
                                    let channel = *channel;
                                    if s.synth.program_select(channel, sf.id, bank, preset).is_ok()
                                    {
                                        s.set_program(channel, path, bank, preset);
                                    }
                                }
                                // Unset the program for this track.
                                Command::UnsetProgram { channel } => {
                                    s.state.programs.remove(channel);
                                }
                                // Load SoundFont.
                                Command::LoadSoundFont { channel, path } => match s
                                    .soundfonts
                                    .get(path)
                                {
                                    // We already loaded this font.
                                    Some(_) => s.set_program_default(*channel, path),
                                    // Load the font.
                                    None => match SoundFont::load(&mut File::open(path).unwrap()) {
                                        Ok(font) => {
                                            let banks = SoundFontBanks::new(font, &mut s.synth);
                                            s.soundfonts.insert(path.clone(), banks);
                                            // Set the default program.
                                            s.set_program_default(*channel, path);
                                        }
                                        Err(error) => {
                                            panic!("Failed to load SoundFont: {:?}", error)
                                        }
                                    },
                                },
                                Command::SetGain { gain } => {
                                    s.synth.set_gain(*gain as f32 / 127.0);
                                    s.state.gain = *gain;
                                }
                                // Start to export audio.
                                Command::Export { path, state } => match path.to_str() {
                                    Some(path) => match OpenOptions::new()
                                        .write(true)
                                        .append(false)
                                        .truncate(true)
                                        .create(true)
                                        .open(path)
                                    {
                                        Ok(file) => {
                                            // Create a new writer.
                                            let writer =
                                                WaveWriter::new(2, 44100, 16, file).unwrap();
                                            // Remember the export state.
                                            s.exporter = Some(writer);
                                            s.export_state = Some(*state);
                                        }
                                        Err(error) => {
                                            panic!("Error opening the file to export: {:?}", error)
                                        }
                                    },
                                    None => {
                                        panic!("Error converting export path to string: {:?}", path)
                                    }
                                },
                            }
                        }
                        // Try to send the state.
                        if send_state.send(s.state.clone()).is_ok() {}
                    }
                }
            }

            if let Some(time) = s.state.time.time {
                // Execute any commands that are at t0 = t.
                s.events_queue
                    .iter()
                    .filter(|e| e.time == time)
                    .for_each(|e| {
                        // Stop time.
                        if let MidiEvent::AllNotesOff { channel: _ } = e.event {
                            s.state.time.time = None;
                            s.state.time.music = false;
                        }
                        // Send.
                        Synthesizer::send_event(
                            Synthesizer::copy_midi_event(&e.event),
                            &mut s.synth,
                        )
                    });
                // Remove those commands.
                s.events_queue.retain(|e| e.time != time);
            }

            // Get the sample.
            let sample = s.synth.read_next();

            // Either export audio or play the file.
            match (&mut s.exporter, &mut s.export_state) {
                (Some(writer), Some(export_state)) => {
                    // Export.
                    if let Err(error) = writer.write_sample_i16(to_i16(sample.0)) {
                        panic!("Error exporting example: {}", error)
                    }
                    if let Err(error) = writer.write_sample_i16(to_i16(sample.1)) {
                        panic!("Error exporting example: {}", error)
                    }
                    // Increment the number of exported samples.
                    export_state.exported += 1;
                    // Send the export state.
                    if send_export.send(*export_state).is_ok() {}
                    // Are we done exporting?
                    if export_state.exported >= export_state.samples {
                        // Sync the header.
                        writer.sync_header().unwrap();
                        // Open the file.
                        s.export_state = None;
                        s.exporter = None;
                    }
                    // Increment time.
                    else if let Some(time) = s.state.time.time.as_mut() {
                        *time += 1;
                    }
                }
                // Set to None.
                (Some(_), None) => s.exporter = None,
                (None, Some(_)) => s.export_state = None,
                // Play.
                (None, None) => {
                    match send_audio.send(sample) {
                        // We sent a message. Increment the time.
                        Ok(_) => {
                            // Increment time.
                            if let Some(time) = s.state.time.time.as_mut() {
                                *time += 1;
                            }
                            // We're ready for a new message.
                            s.ready = true;
                        }
                        // Wait.
                        Err(_) => continue,
                    }
                    // Send the time state.
                    if send_time.try_send(s.state.time).is_ok() {}
                }
            }
        }
    }

    /// Send a MidiEvent to the Synth. We don't care if it succeeds or not.
    fn send_event(event: MidiEvent, synth: &mut Synth) {
        if synth.send_event(event).is_ok() {}
    }

    /// Copy a MIDI event. It's very dumb that we have to do it this way but... ok fine.
    fn copy_midi_event(event: &MidiEvent) -> MidiEvent {
        match event {
            MidiEvent::NoteOn { channel, key, vel } => MidiEvent::NoteOn {
                channel: *channel,
                key: *key,
                vel: *vel,
            },
            MidiEvent::NoteOff { channel, key } => MidiEvent::NoteOff {
                channel: *channel,
                key: *key,
            },
            MidiEvent::ControlChange {
                channel,
                ctrl,
                value,
            } => MidiEvent::ControlChange {
                channel: *channel,
                ctrl: *ctrl,
                value: *value,
            },
            MidiEvent::AllNotesOff { channel } => MidiEvent::AllNotesOff { channel: *channel },
            MidiEvent::AllSoundOff { channel } => MidiEvent::AllSoundOff { channel: *channel },
            MidiEvent::PitchBend { channel, value } => MidiEvent::PitchBend {
                channel: *channel,
                value: *value,
            },
            MidiEvent::ProgramChange {
                channel,
                program_id,
            } => MidiEvent::ProgramChange {
                channel: *channel,
                program_id: *program_id,
            },
            MidiEvent::ChannelPressure { channel, value } => MidiEvent::ChannelPressure {
                channel: *channel,
                value: *value,
            },
            MidiEvent::PolyphonicKeyPressure {
                channel,
                key,
                value,
            } => MidiEvent::PolyphonicKeyPressure {
                channel: *channel,
                key: *key,
                value: *value,
            },
            MidiEvent::SystemReset => MidiEvent::SystemReset,
        }
    }

    /// Set the synthesizer program to a program.
    fn set_program(&mut self, channel: u8, path: &PathBuf, bank: u32, preset: u8) {
        let sf_banks = &self.soundfonts[path].banks;
        // Get the bank info.
        let mut banks: Vec<u32> = sf_banks.keys().copied().collect();
        banks.sort();
        let bank_index = banks.iter().position(|&b| b == bank).unwrap();
        let bank: u32 = banks[bank_index];
        // Get the preset info.
        let presets = sf_banks[&bank].clone();
        let preset_index = presets.iter().position(|&p| p == preset).unwrap();
        let preset_name = self
            .synth
            .channel_preset(channel)
            .unwrap()
            .name()
            .to_string();
        let num_banks = banks.len();
        let num_presets = presets.len();
        let program = Program {
            path: path.clone(),
            num_banks,
            bank_index,
            bank,
            num_presets,
            preset_index,
            preset_name,
        };
        // Remember the program.
        self.state.programs.insert(channel, program);
    }

    /// Set the synthesizer program to a default program.
    fn set_program_default(&mut self, channel: u8, path: &PathBuf) {
        let sf_banks = &self.soundfonts[path].banks;
        // Get the bank info.
        let mut banks: Vec<u32> = sf_banks.keys().copied().collect();
        banks.sort();
        let bank = banks[0];
        let preset = sf_banks[&bank][0];
        self.set_program(channel, path, bank, preset);
        // Select the default program.
        let id = self.soundfonts[path].id;
        self.synth
            .program_select(channel, id, bank, preset)
            .unwrap();
    }
}

/// Converts an f32 sample to an i16 sample.
fn to_i16(sample: f32) -> i16 {
    (sample * F32_TO_I16).floor() as i16
}
