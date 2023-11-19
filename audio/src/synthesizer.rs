use crate::exporter::*;
use crate::{
    AudioBuffer, AudioMessage, Command, CommandsMessage, ExportState, Program, SharedExporter,
    SynthState, TimeState,
};
use crossbeam_channel::{Receiver, Sender};
use hashbrown::HashMap;
use oxisynth::{MidiEvent, SoundFont, SoundFontId, Synth};
use std::fs::File;
use std::path::PathBuf;

/// Export this many bytes per decay chunk.
const DECAY_CHUNK_SIZE: usize = 2048;
///  Oxisynth usually doesn't zero out its audio. This is essentially an epsilon.
/// This is used to detect if the export is done.
const SILENCE: [f32; 2] = [-1e-7, 1e-7];

/// A convenient wrapper for a SoundFont.
struct SoundFontBanks {
    id: SoundFontId,
    /// The banks and their presets.
    banks: HashMap<u32, Vec<u8>>,
}

impl SoundFontBanks {
    pub fn new(font: SoundFont, synth: &mut Synth) -> Self {
        let mut banks: HashMap<u32, Vec<u8>> = HashMap::new();
        (0u32..129u32).for_each(|b| {
            let presets: Vec<u8> = (0u8..128)
                .filter(|p| font.preset(b, *p).is_some())
                .collect();
            if !presets.is_empty() {
                banks.insert(b, presets);
            }
        });

        let id = synth.add_font(font, true);
        Self { id, banks }
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
    /// The export file path.
    export_path: Option<PathBuf>,
    /// The exporter.
    exporter: SharedExporter,
    /// The buffer that the exporter writes to.
    export_buffer: AudioBuffer,
    /// If true, we need to send the export state.
    send_export_state: bool,
}

impl Synthesizer {
    /// Start the synthesizer loop.
    ///
    /// - `recv_commands` Receive commands from the conn.
    /// - `send_audio` Send audio samples to the player.
    /// - `send_state` Send a state to the conn.
    /// - `send_export` Send audio samples to an exporter.
    /// - `send_time` Send the time to the conn.
    /// - `send_sample` Send an audio sample to the conn.
    /// - `exporter` The shared exporter.
    pub(crate) fn start(
        recv_commands: Receiver<CommandsMessage>,
        send_audio: Sender<AudioMessage>,
        send_state: Sender<SynthState>,
        send_export: Sender<Option<ExportState>>,
        send_time: Sender<TimeState>,
        send_sample: Sender<AudioMessage>,
        exporter: SharedExporter,
    ) {
        // Create the synthesizer.
        let mut s = Synthesizer {
            synth: Synth::default(),
            soundfonts: HashMap::new(),
            events_queue: vec![],
            ready: true,
            state: SynthState::default(),
            export_path: None,
            export_state: None,
            exporter,
            send_export_state: false,
            export_buffer: [vec![], vec![]],
        };
        s.synth.set_gain(127.0);
        loop {
            if s.ready {
                // Try to receive commands.
                match recv_commands.try_recv() {
                    Err(_) => (),
                    Ok(commands) => {
                        s.ready = false;
                        for command in commands.iter() {
                            match command {
                                Command::SetFramerate { framerate } => {
                                    s.synth.set_sample_rate(*framerate as f32)
                                }
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
                                    });
                                    s.sort_queue();
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
                                    s.sort_queue();
                                }
                                // Schedule a note-on and a note-off.
                                Command::NoteOnAt {
                                    channel,
                                    key,
                                    velocity,
                                    start,
                                    end,
                                } => {
                                    let channel = *channel;
                                    let key = *key;
                                    s.events_queue.push(QueuedEvent {
                                        time: *start,
                                        event: MidiEvent::NoteOn {
                                            channel,
                                            key,
                                            vel: *velocity,
                                        },
                                    });
                                    s.events_queue.push(QueuedEvent {
                                        time: *end,
                                        event: MidiEvent::NoteOff { channel, key },
                                    });
                                    s.sort_queue();
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
                                Command::LoadSoundFont { channel, path } => match &s
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
                                            // Restore the other programs.
                                            let programs = s.state.programs.clone();
                                            for program in
                                                programs.iter().filter(|p| p.0 != channel)
                                            {
                                                s.synth
                                                    .program_select(
                                                        *program.0,
                                                        s.soundfonts[&program.1.path].id,
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
                                },
                                Command::SetGain { gain } => {
                                    s.synth.set_gain(*gain as f32 / 127.0);
                                    s.state.gain = *gain;
                                }
                                // Start to export audio.
                                Command::Export { path, state } => {
                                    s.export_path = Some(path.clone());
                                    s.export_state = Some(*state);
                                    // Clear the buffers.
                                    s.export_buffer[0].clear();
                                    s.export_buffer[1].clear();
                                }
                                // Send the export state.
                                Command::SendExportState => s.send_export_state = true,
                                // Append silence to the end of exported audio.
                                Command::AppendSilences { paths } => {
                                    let exporter = s.exporter.lock();
                                    exporter.append_silences(paths);
                                }
                            }
                        }
                        // Try to send the state.
                        if send_state.send(s.state.clone()).is_ok() {}
                    }
                }
            }

            if let Some(time) = s.state.time.time {
                // Execute any commands that are at t0 = t.
                if !s.events_queue.is_empty() && s.events_queue[0].time == time {
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
                    // Remove the events.
                    s.events_queue.retain(|e| e.time != time);
                }
            }

            // Either export audio or play the file.
            match &mut s.export_state {
                Some(export_state) => {
                    // Are we done exporting?
                    if export_state.exported >= export_state.samples {
                        let mut decaying = false;
                        for _ in 0..DECAY_CHUNK_SIZE {
                            // Read a sample.
                            let sample = s.synth.read_next();
                            // Write the sample.
                            s.export_buffer[0].push(sample.0);
                            s.export_buffer[1].push(sample.1);
                            // There is still sound.
                            if sample.0 < SILENCE[0]
                                || sample.0 > SILENCE[1]
                                || sample.1 < SILENCE[0]
                                || sample.1 > SILENCE[1]
                            {
                                decaying = true;
                            }
                        }
                        // We're done!
                        if !decaying {
                            let exporter = s.exporter.lock();
                            match exporter.export_type.get() {
                                ExportType::Mid => {
                                    panic!("Tried exporting a .mid from the synthesizer")
                                }
                                // Export to a .wav file.
                                ExportType::Wav => {
                                    exporter.wav(s.export_path.as_ref().unwrap(), &s.export_buffer);
                                }
                                ExportType::MP3 => {
                                    exporter.mp3(s.export_path.as_ref().unwrap(), &s.export_buffer);
                                }
                                ExportType::Ogg => {
                                    exporter.ogg(s.export_path.as_ref().unwrap(), &s.export_buffer)
                                }
                                ExportType::Flac => {
                                    exporter.flac(s.export_path.as_ref().unwrap(), &s.export_buffer)
                                }
                            }
                            // Stop exporting.
                            s.export_state = None;
                            s.export_buffer[0].clear();
                            s.export_buffer[1].clear();
                        }
                    } else if let Some(time) = s.state.time.time.as_mut() {
                        // There are no more events or the next event is right now. Export 1 sample.
                        if s.events_queue.is_empty() || s.events_queue[0].time == *time {
                            *time += 1;
                            s.export_sample();
                        }
                        // Export up to the next event.
                        else {
                            let dt = s.events_queue[0].time - *time;
                            let dtu = dt as usize;
                            let mut left = vec![0.0; dtu];
                            let mut right = vec![0.0; dtu];
                            s.synth.write((left.as_mut_slice(), right.as_mut_slice()));
                            s.export_buffer[0].append(&mut left);
                            s.export_buffer[1].append(&mut right);
                            *time += dt;
                            // Increment the number of exported samples.
                            s.export_state.as_mut().unwrap().exported += dt;
                        }
                    } else {
                        s.export_sample();
                    }
                    // We're ready for a new message.
                    s.ready = true;
                }
                // Play.
                None => {
                    // Get the sample.
                    let sample = s.synth.read_next();
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
                    // Send the sample.
                    let _ = send_sample.try_send(sample);
                    // Send the time state.
                    let _ = send_time.try_send(s.state.time);
                }
            }
            // Stop exporting.
            if s.export_state.is_none() && s.export_path.is_some() {
                s.export_path = None;
            }
            // Send the export state.
            if s.send_export_state && send_export.send(s.export_state).is_ok() {
                s.send_export_state = false;
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
            preset,
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
        // Select the default program.
        let id = self.soundfonts[path].id;
        self.synth
            .program_select(channel, id, bank, preset)
            .unwrap();
        self.set_program(channel, path, bank, preset);
    }

    /// Sort the events queue by time.
    fn sort_queue(&mut self) {
        self.events_queue.sort_by(|a, b| a.time.cmp(&b.time))
    }

    /// Push one sample to the export buffer.
    fn export_sample(&mut self) {
        let sample = self.synth.read_next();
        // Write the sample.
        self.export_buffer[0].push(sample.0);
        self.export_buffer[1].push(sample.1);
        // Increment the number of exported samples.
        self.export_state.as_mut().unwrap().exported += 1;
    }
}
