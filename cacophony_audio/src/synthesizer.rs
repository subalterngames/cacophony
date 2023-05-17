use crate::{AudioMessage, Command, CommandsMessage, ExportedAudio, Program, SynthState, TimeState};
use crossbeam_channel::{Receiver, Sender};
use oxisynth::{MidiEvent, SoundFont, SoundFontId, Synth};
use std::collections::HashMap;
use std::fs::File;

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

pub(crate) struct Synthesizer {
    /// The synthesizer.
    synth: Synth,
    /// A map of the SoundFonts and their banks. Key = Path.
    soundfonts: HashMap<String, SoundFontBanks>,
    /// A list of queued MIDI events.
    events_queue: Vec<QueuedEvent>,
    /// If true, we're ready to receive more commands.
    ready: bool,
    /// The state of the synthesizer.
    state: SynthState,
}

impl Synthesizer {
    pub(crate) fn start(
        recv_commands: Receiver<CommandsMessage>,
        send_audio: Sender<AudioMessage>,
        send_state: Sender<SynthState>,
        send_exported_audio: Sender<ExportedAudio>,
        send_time: Sender<TimeState>
    ) {
        // Create the synthesizer.
        let mut s = Synthesizer {
            synth: Synth::default(),
            soundfonts: HashMap::new(),
            events_queue: vec![],
            ready: true,
            state: SynthState::default(),
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
                                Command::PlayMusic => s.state.time.music = true,
                                Command::StopMusic => s.state.time.music = false,
                                // Stop all audio.
                                Command::StopAll { channels } => {
                                    channels.iter().for_each(|c| {
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
                                Command::StopAllAt { channels, time } => {
                                    channels.iter().for_each(|c| {
                                        s.events_queue.push(QueuedEvent {
                                            time: *time,
                                            event: MidiEvent::AllNotesOff { channel: *c },
                                        });
                                    })
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
                                // Schedule a note-off.
                                Command::NoteOffAt { channel, key, time } => {
                                    s.events_queue.push(QueuedEvent {
                                        time: *time,
                                        event: MidiEvent::NoteOff {
                                            channel: *channel,
                                            key: *key,
                                        },
                                    });
                                }
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
                                },
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
                                },
                                // Export audio.
                                Command::Export { commands } => {
                                    match send_exported_audio.send(s.export(commands.clone())) {
                                        Ok(_) => (),
                                        Err(error) => panic!("Failed to export audio! {}", error),
                                    }
                                },
                                Command::SetTime { time } => s.state.time.time = Some(*time)
                            }
                        }
                        // Try to send the state.
                        if send_state.send(s.state.clone()).is_ok() { }
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

            // Send audio.
            match send_audio.send(s.synth.read_next()) {
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

    /// Export all audio using the queued up events.
    fn export(&mut self, commands: CommandsMessage) -> ExportedAudio {
        let mut num_note_offs: usize = 0;
        for command in commands.iter() {
            if let Command::NoteOffAt {
                channel: _,
                key: _,
                time: _,
            } = command
            {
                num_note_offs += 1
            }
        }
        // Get the buffer.
        let mut buffer: ExportedAudio = vec![];
        let mut note_offs: usize = 0;
        let mut t = 0;
        // Get samples.
        while note_offs < num_note_offs {
            for command in commands.iter() {
                match command {
                    Command::NoteOnAt {
                        channel,
                        key,
                        velocity,
                        time,
                        duration: _,
                    } => {
                        // Time to play this note.
                        if *time == t {
                            Synthesizer::send_event(
                                MidiEvent::NoteOn {
                                    channel: *channel,
                                    key: *key,
                                    vel: *velocity,
                                },
                                &mut self.synth,
                            );
                        }
                    }
                    Command::NoteOffAt { channel, key, time } => {
                        if *time == t {
                            // Increment the note-off counter.
                            note_offs += 1;
                            // Note-off.
                            Synthesizer::send_event(
                                MidiEvent::NoteOff {
                                    channel: *channel,
                                    key: *key,
                                },
                                &mut self.synth,
                            );
                        }
                    }
                    _ => (),
                }
            }
            // Get a sample.
            let sample = self.synth.read_next();
            buffer.push([sample.0, sample.1]);
            // Increment the time.
            t += 1;
        }
        // Append some silence.
        let silence = [0.0, 0.0];
        (0..44100).for_each(|_| buffer.push(silence));
        buffer
    }

    /// Set the synthesizer program to a program.
    fn set_program(&mut self, channel: u8, path: &String, bank: u32, preset: u8) {
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
    fn set_program_default(&mut self, channel: u8, path: &String) {
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
