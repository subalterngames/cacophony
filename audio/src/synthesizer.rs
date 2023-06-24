use crate::{AudioMessage, Command, CommandsMessage, ExportState, Program, SynthState, TimeState};
use common::chrono::prelude::*;
use common::export_settings::*;
use common::hashbrown::HashMap;
use crossbeam_channel::{Receiver, Sender};
use hound::*;
use id3::*;
use mp3lame_encoder::*;
use oxisynth::{MidiEvent, SoundFont, SoundFontId, Synth};
use std::fs::File;
use std::path::PathBuf;

const MP3_BIT_RATES: [Bitrate; 16] = [
    Bitrate::Kbps8,
    Bitrate::Kbps16,
    Bitrate::Kbps24,
    Bitrate::Kbps32,
    Bitrate::Kbps40,
    Bitrate::Kbps48,
    Bitrate::Kbps64,
    Bitrate::Kbps80,
    Bitrate::Kbps96,
    Bitrate::Kbps112,
    Bitrate::Kbps128,
    Bitrate::Kbps160,
    Bitrate::Kbps192,
    Bitrate::Kbps224,
    Bitrate::Kbps256,
    Bitrate::Kbps320,
];
const MP3_QUALITIES: [Quality; 10] = [
    Quality::Best,
    Quality::SecondBest,
    Quality::NearBest,
    Quality::VeryNice,
    Quality::Nice,
    Quality::Good,
    Quality::Decent,
    Quality::Ok,
    Quality::SecondWorst,
    Quality::Worst,
];
/// Conversion factor for f32 to i16.
const F32_TO_I16: f32 = 32767.5;
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
    /// The export file path.
    export_path: Option<PathBuf>,
    /// The export settings.
    export_settings: ExportSettings,
    /// The buffer that the exporter writes to.
    export_buffer: [Vec<f32>; 2],
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
    pub(crate) fn start(
        recv_commands: Receiver<CommandsMessage>,
        send_audio: Sender<AudioMessage>,
        send_state: Sender<SynthState>,
        send_export: Sender<Option<ExportState>>,
        send_time: Sender<TimeState>,
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
            export_settings: ExportSettings::new(),
            send_export_state: false,
            export_buffer: [vec![], vec![]],
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
                                Command::Export { path, state } => {
                                    s.export_path = Some(path.clone());
                                    s.export_state = Some(*state);
                                    // Clear the buffers.
                                    s.export_buffer[0].clear();
                                    s.export_buffer[1].clear();
                                }
                                // Send the export state.
                                Command::SendExportState => s.send_export_state = true,
                                // Set the MP3 export settings.
                                Command::SetMP3 { mp3 } => s.export_settings.mp3 = *mp3,
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
                            match EXPORT_TYPES[s.export_settings.export_type.get()] {
                                ExportType::Mid => {
                                    panic!("Tried exporting a .mid from the synthesizer")
                                }
                                // Export to a .wav file.
                                ExportType::Wav => {
                                    s.write_wav();
                                    let tag = s.get_tag();
                                    let path = s.export_path.as_ref().unwrap();
                                    if let Err(error) = tag.write_to_wav_path(path, Version::Id3v24)
                                    {
                                        panic!("Error writing ID3 tag to {:?}: {}", path, error);
                                    }
                                }
                                ExportType::MP3 => {
                                    // Create the encoder.
                                    let mut mp3_encoder =
                                        Builder::new().expect("Create LAME builder");
                                    mp3_encoder.set_num_channels(2).expect("set channels");
                                    mp3_encoder
                                        .set_sample_rate(s.export_settings.framerate.get_u() as u32)
                                        .expect("set sample rate");
                                    mp3_encoder
                                        .set_brate(
                                            MP3_BIT_RATES[s.export_settings.mp3.bit_rate.get()],
                                        )
                                        .expect("set bitrate");
                                    mp3_encoder
                                        .set_quality(
                                            MP3_QUALITIES[s.export_settings.mp3.quality.get()],
                                        )
                                        .expect("set quality");
                                    // Build the encoder.
                                    let mut mp3_encoder =
                                        mp3_encoder.build().expect("To initialize LAME encoder");
                                    // Get the input.
                                    let input = DualPcm {
                                        left: &s.export_buffer[0],
                                        right: &s.export_buffer[1],
                                    };
                                    // Get the output buffer.
                                    let mut mp3_out_buffer = Vec::new();
                                    mp3_out_buffer.reserve(max_required_buffer_size(
                                        s.export_buffer[0].len(),
                                    ));
                                    // Get the size.
                                    let encoded_size = mp3_encoder
                                        .encode(input, mp3_out_buffer.spare_capacity_mut())
                                        .expect("To encode");
                                    unsafe {
                                        mp3_out_buffer.set_len(
                                            mp3_out_buffer.len().wrapping_add(encoded_size),
                                        );
                                    }
                                    let encoded_size = mp3_encoder
                                        .flush::<FlushNoGap>(mp3_out_buffer.spare_capacity_mut())
                                        .expect("to flush");
                                    unsafe {
                                        mp3_out_buffer.set_len(
                                            mp3_out_buffer.len().wrapping_add(encoded_size),
                                        );
                                    }
                                    let tag = s.get_tag();
                                    let path = s.export_path.as_ref().unwrap();
                                    if let Err(error) = tag.write_to_path(path, Version::Id3v24) {
                                        panic!("Error writing ID3 tag to {:?}: {}", path, error);
                                    }
                                }
                            }
                            // Stop exporting.
                            s.export_state = None;
                            s.export_buffer[0].clear();
                            s.export_buffer[1].clear();
                        }
                    } else {
                        // Read a sample.
                        let sample = s.synth.read_next();
                        // Write the sample.
                        s.export_buffer[0].push(sample.0);
                        s.export_buffer[1].push(sample.1);
                        // Increment the number of exported samples.
                        export_state.exported += 1;
                        // Increment time.
                        if let Some(time) = s.state.time.time.as_mut() {
                            *time += 1;
                        }
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
                    // Send the time state.
                    if send_time.try_send(s.state.time).is_ok() {}
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

    fn write_wav(&self) {
        // Get the path.
        let path = self.export_path.as_ref().unwrap().to_str().unwrap();
        // Get the spec.
        let spec = WavSpec {
            channels: 2,
            sample_rate: self.export_settings.framerate.get_u() as u32,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };
        // Write.
        let mut writer = WavWriter::create(path, spec).unwrap();
        let mut i16_writer = writer.get_i16_writer(self.export_buffer[0].len() as u32 * 2);
        for (l, r) in self.export_buffer[0]
            .iter()
            .zip(self.export_buffer[1].iter())
        {
            i16_writer.write_sample(to_i16(l));
            i16_writer.write_sample(to_i16(r));
        }
        i16_writer.flush().unwrap();
        writer.finalize().unwrap();
    }

    fn get_tag(&self) -> Tag {
        let time = Local::now();
        let mut tag = Tag::new();
        tag.set_year(time.year());
        tag.set_title(&self.export_settings.metadata.title);
        if let Some(artist) = &self.export_settings.metadata.artist {
            tag.set_artist(artist);
        }
        if let Some(album) = &self.export_settings.metadata.album {
            tag.set_album(album);
        }
        if let Some(genre) = &self.export_settings.metadata.genre {
            tag.set_genre(genre);
        }
        if let Some(comment) = &self.export_settings.metadata.comment {
            tag.set_genre(comment);
        }
        if let Some(track_number) = &self.export_settings.metadata.track_number {
            tag.set_track(*track_number);
        }
        tag
    }
}

/// Converts an f32 sample to an i16 sample.
fn to_i16(sample: &f32) -> i16 {
    (sample * F32_TO_I16).floor() as i16
}
