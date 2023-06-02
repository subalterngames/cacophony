use crate::AudioMessage;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::*;
use crossbeam_channel::Receiver;

const ERROR_MESSAGE: &str = "Failed to create an audio output stream: ";

/// Try to start an audio stream and play audio.
/// Source: https://github.com/PolyMeilex/OxiSynth/blob/master/examples/real-time/src/main.rs
pub(crate) struct Player {
    _host: Host,
    _stream: Option<Stream>,
}

impl Player {
    pub(crate) fn new(recv: Receiver<AudioMessage>) -> Option<Self> {
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
                    let stream_config: StreamConfig = config.into();
                    let channels = stream_config.channels as usize;

                    // Try to get a stream.
                    let stream = match sample_format {
                        SampleFormat::F32 => {
                            Player::run::<f32>(recv, channels, device, stream_config)
                        }
                        SampleFormat::I16 => {
                            Player::run::<i16>(recv, channels, device, stream_config)
                        }
                        SampleFormat::U16 => {
                            Player::run::<u16>(recv, channels, device, stream_config)
                        }
                    };
                    Some(Self {
                        _host: host,
                        _stream: stream,
                    })
                }
            },
        }
    }

    /// Start running the stream.
    fn run<T>(
        recv: Receiver<AudioMessage>,
        channels: usize,
        device: Device,
        stream_config: StreamConfig,
    ) -> Option<Stream>
    where
        T: Sample,
    {
        // Define the error callback.
        let err_callback = |err| println!("Stream error: {}", err);

        // Move `recv` into a closure.
        let next_sample = move || recv.recv();

        // Define the data callback used by cpal. Move `stream_send` into the closure.
        let data_callback = move |output: &mut [T], _: &OutputCallbackInfo| {
            for frame in output.chunks_mut(channels) {
                // Try to receive a new sample.
                if let Ok((l, r)) = next_sample() {
                    let channels = [Sample::from::<f32>(&l), Sample::from::<f32>(&r)];
                    for (id, sample) in frame.iter_mut().enumerate() {
                        *sample = channels[id % 2];
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
