use crate::decayer::Decayer;
use crate::play_state::PlayState;
use crate::types::SharedSample;
use crate::{SharedMidiEventQueue, SharedPlayState, SharedSynth, SharedTimeState};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::*;

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
        let mut left = vec![0.0; 1];
        let mut right = vec![0.0; 1];
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
                        for (frame, (left, right)) in output
                            .chunks_mut(channels)
                            .zip(decayer.left[0..len].iter().zip(&decayer.right[0..len]))
                        {
                            // Add the sample.
                            // This is almost certainly more performant than the code in the `else` block.
                            if two_channels {
                                frame[0] = *left;
                                frame[1] = *right;
                            }
                            // Add for more than one channel. This is slower.
                            else {
                                let channels = [*left, *right];
                                for (id, sample) in frame.iter_mut().enumerate() {
                                    *sample = channels[id % 2];
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
                    let mut time_state = time_state.lock();
                    let mut midi_event_queue = midi_event_queue.lock();
                    // There are no more events. Fill the buffer and advance time.
                    if midi_event_queue.is_empty() {
                        // Resize the buffers.
                        if len > left.len() {
                            left.resize(len, 0.0);
                            right.resize(len, 0.0);
                        }

                        // Write the samples.
                        let mut synth = synth.lock();
                        synth.write((left[0..len].as_mut(), right[0..len].as_mut()));

                        // Stop time.
                        if time_state.time.is_some() {
                            time_state.music = false;
                            time_state.time = None;
                            let mut play_state = play_state.lock();
                            *play_state = PlayState::Decaying;
                        }
                    }
                    // There are MIDI events.
                    else {
                        // Iterate through the number of samples.
                        for frame in output.chunks_mut(channels) {
                            // We're playing music. Advance to the next events.
                            if let Some(time) = time_state.time {
                                // Dequeue events.
                                let events = midi_event_queue.dequeue(time);
                                // Send the MIDI events to the synth.
                                if !events.is_empty() {
                                    let mut synth = synth.lock();
                                    for event in events {
                                        if synth.send_event(event).is_ok() {}
                                    }
                                }
                                // Advance time by one sample.
                                time_state.time = Some(time + 1)
                            }
                            // Get the next sample.
                            let mut synth = synth.lock();
                            // Get the sample.
                            let (left, right) = synth.read_next();

                            // Add the sample.
                            // This is almost certainly more performant than the code in the `else` block.
                            if two_channels {
                                frame[0] = left;
                                frame[1] = right;
                            }
                            // Add for more than one channel. This is slower.
                            else {
                                let channels = [left, right];
                                for (id, sample) in frame.iter_mut().enumerate() {
                                    *sample = channels[id % 2];
                                }
                            }
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
}
