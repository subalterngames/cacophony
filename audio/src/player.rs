use crate::decayer::Decayer;
use crate::play_state::PlayState;
use crate::types::SharedSample;
use crate::{SharedMidiEventQueue, SharedPlayState, SharedSynth, SharedTimeState};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::*;
use oxisynth::Synth;

const ERROR_MESSAGE: &str = "Failed to create an audio output stream: ";

/// Try to start an audio stream and play audio.
/// Source: https://github.com/PolyMeilex/OxiSynth/blob/master/examples/real-time/src/main.rs
pub(crate) struct Player {
    /// The audio host. We don't want to drop it.
    _host: Host,
    /// The audio stream. We don't want to drop it.
    _stream: Option<Stream>,
    /// The machine's audio framerate.
    pub framerate: u32,
}

impl Player {
    pub(crate) fn new(
        midi_event_queue: SharedMidiEventQueue,
        time_state: SharedTimeState,
        synth: SharedSynth,
        sample: SharedSample,
        play_state: SharedPlayState,
    ) -> Option<Self> {
        // Get the host.
        let host = default_host();
        // Try to get an output device.
        match host.default_output_device() {
            None => {
                println!("{} Failed to get output device", ERROR_MESSAGE);
                None
            }
            // Try to get config info.
            Some(device) => match device.default_output_config() {
                Err(err) => {
                    println!("{} {}", ERROR_MESSAGE, err);
                    None
                }
                // We have a device and a config!
                Ok(config) => {
                    let framerate = config.sample_rate().0;
                    let stream_config: StreamConfig = config.into();
                    let channels = stream_config.channels as usize;

                    // Try to get a stream.
                    let stream = Player::run(
                        channels,
                        device,
                        stream_config,
                        midi_event_queue,
                        time_state,
                        synth,
                        sample,
                        play_state,
                    );
                    Some(Self {
                        _host: host,
                        _stream: stream,
                        framerate,
                    })
                }
            },
        }
    }

    /// Start running the stream.
    #[allow(clippy::too_many_arguments)]
    fn run(
        channels: usize,
        device: Device,
        stream_config: StreamConfig,
        midi_event_queue: SharedMidiEventQueue,
        time_state: SharedTimeState,
        synth: SharedSynth,
        sample: SharedSample,
        play_state: SharedPlayState,
    ) -> Option<Stream> {
        // Define the error callback.
        let err_callback = |err| println!("Stream error: {}", err);

        let two_channels = channels == 2;
        let mut buffer = vec![0.0; 2];
        let mut sample_buffer = [0.0; 2];
        let mut decayer = Decayer::default();

        // Define the data callback used by cpal. Move `stream_send` into the closure.
        let data_callback = move |output: &mut [f32], _: &OutputCallbackInfo| {
            let ps = *play_state.lock();
            match ps {
                // Assume that there is no audio and do nothing.
                PlayState::NotPlaying => (),
                // Add decay.
                PlayState::Decaying => {
                    // Write the decay block.
                    let len = output.len() / channels;
                    decayer.decay_shared(&synth, len);
                    // Set the decay block.
                    if decayer.decaying {
                        for (out_frame, in_frame) in output
                            .chunks_mut(channels)
                            .zip(decayer.buffer[0..len].chunks_mut(2))
                        {
                            // Add the sample.
                            // This is almost certainly more performant than the code in the `else` block.
                            if two_channels {
                                out_frame.copy_from_slice(in_frame);
                            }
                            // Add for more than one channel. This is slower.
                            else {
                                for (id, sample) in out_frame.iter_mut().enumerate() {
                                    *sample = in_frame[id % 2];
                                }
                            }
                        }
                    }
                    // Done decaying.
                    else {
                        // Fill the output with silence.
                        output.iter_mut().for_each(|o| *o = 0.0);
                        let mut play_state = play_state.lock();
                        *play_state = PlayState::NotPlaying;
                    }
                }
                // Playing music.
                PlayState::Playing => {
                    let len = output.len();
                    // Resize the buffers.
                    if len > buffer.len() {
                        buffer.resize(len, 0.0);
                    }
                    // Get the next sample.
                    let mut synth = synth.lock();
                    let time = Self::get_time(&time_state);
                    match time {
                        // We are playing music.
                        Some(time) => {
                            let mut midi_event_queue = midi_event_queue.lock();
                            // Iterate through the output buffer's frames.
                            let mut begin_decay = false;
                            let buffer_len = len / channels;
                            let mut t = time;
                            for frame in output.chunks_mut(channels) {
                                match midi_event_queue.get_next_time() {
                                    Some(next_time) => {
                                        // There are events on this frame.
                                        if t == next_time {
                                            // Dequeue events.
                                            let events = midi_event_queue.dequeue(t);
                                            // Send the MIDI events to the synth.
                                            if !events.is_empty() {
                                                for event in events {
                                                    if synth.send_event(event).is_ok() {}
                                                }
                                            }
                                        }
                                        // Add the sample.
                                        // This is almost certainly more performant than the code in the `else` block.
                                        if two_channels {
                                            // Get the sample.
                                            synth.write(frame);
                                        }
                                        // Add for more than one channel. This is slower.
                                        else {
                                            synth.write(sample_buffer.as_mut_slice());
                                            for (id, sample) in frame.iter_mut().enumerate() {
                                                *sample = sample_buffer[id % 2];
                                            }
                                        }
                                        // Advance time.
                                        t += 1;
                                    }
                                    // There are no more events.
                                    None => {
                                        Self::stop_time(&time_state);
                                        begin_decay = true;
                                        break;
                                    }
                                }
                            }
                            Self::set_time(&time_state, t);
                            if begin_decay {
                                Self::begin_decay(
                                    buffer[0..buffer_len].as_mut(),
                                    output,
                                    channels,
                                    two_channels,
                                    &play_state,
                                    &mut synth,
                                );
                            }
                        }
                        None => {
                            let buffer_len = len / channels;
                            Self::begin_decay(
                                buffer[0..buffer_len].as_mut(),
                                output,
                                channels,
                                two_channels,
                                &play_state,
                                &mut synth,
                            );
                        }
                    }
                }
            }
            // Share the first sample.
            let mut sample = sample.lock();
            sample.0 = output[0];
            sample.1 = output[1]
        };

        // Build the cpal output stream from the stream config info and the callbacks.
        match device.build_output_stream(&stream_config, data_callback, err_callback) {
            // We have a stream!
            Ok(stream) => match stream.play() {
                Ok(_) => Some(stream),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }

    fn get_time(time_state: &SharedTimeState) -> Option<u64> {
        time_state.lock().time.clone()
    }

    fn set_time(time_state: &SharedTimeState, time: u64) {
        let mut time_state = time_state.lock();
        if let Some(t0) = time_state.time.as_mut() {
            *t0 = time
        }
    }

    fn stop_time(time_state: &SharedTimeState) {
        let mut time_state = time_state.lock();
        time_state.music = false;
        time_state.time = None;
    }

    fn begin_decay(
        buffer: &mut [f32],
        output: &mut [f32],
        channels: usize,
        two_channels: bool,
        play_state: &SharedPlayState,
        synth: &mut Synth,
    ) {
        if two_channels {
            synth.write(output);
        } else {
            // Write decay samples.
            synth.write(buffer.as_mut());
            for (out_frame, in_frame) in output.chunks_mut(channels).zip(buffer.chunks(2)) {
                for (id, sample) in out_frame.iter_mut().enumerate() {
                    *sample = in_frame[id % 2];
                }
            }
        }
        *play_state.lock() = PlayState::Decaying;
    }
}
