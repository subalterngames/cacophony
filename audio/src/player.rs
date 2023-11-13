use crate::{SharedMidiEventQueue, SharedSynth, SharedTimeState};
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
                    let sample_format = config.sample_format();
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
    fn run(
        channels: usize,
        device: Device,
        stream_config: StreamConfig,
        midi_event_queue: SharedMidiEventQueue,
        time_state: SharedTimeState,
        synth: SharedSynth,
    ) -> Option<Stream> {
        // Define the error callback.
        let err_callback = |err| println!("Stream error: {}", err);

        let two_channels = channels == 2;

        let audio_buffers = [vec![0.0; 1], vec![0.0; 1]];

        // Define the data callback used by cpal. Move `stream_send` into the closure.
        let data_callback = move |output: &mut [f32], _: &OutputCallbackInfo| {
            let mut time_state = time_state.lock();
            let mut midi_event_queue = midi_event_queue.lock();
            // There are no more events. Fill the buffer and advance time.
            if midi_event_queue.is_empty() {
                let len = output.len();

                // Resize the buffers.
                if len > audio_buffers[0].len() {
                    audio_buffers[0].resize(len, 0.0);
                    audio_buffers[1].resize(len, 0.0);
                }

                // Write the samples.
                let mut synth = synth.lock();
                synth.write((&mut audio_buffers[0][0..len], &mut audio_buffers[1][0..len]));

                // Advance time.
                if let Some(time) = time_state.time {
                    time_state.time = Some(time + len as u64);
                }
            } else {
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
