use crate::export::{ExportSetting, ExportType, Metadata, MultiFileSuffix};
use crate::{AudioBuffer, SynthState};
use chrono::Datelike;
use chrono::Local;
use common::IndexedValues;
use common::{Index, Music, Time, U64orF32, DEFAULT_FRAMERATE, PPQ_F, PPQ_U};
use flacenc::bitsink::ByteSink;
use flacenc::component::BitRepr;
use flacenc::config::Encoder as FlacEncoder;
use flacenc::encode_with_fixed_block_size;
use flacenc::source::MemSource;
use hound::{SampleFormat, WavSpec, WavWriter};
use id3::{Tag, TagLike, Version};
use metaflac::Tag as FlacTag;
use midly::num::{u15, u24, u28, u4};
use midly::{
    write_std, Format, Header, MetaMessage, MidiMessage, Timing, Track, TrackEvent, TrackEventKind,
};
use mp3lame_encoder::*;
use oggvorbismeta::*;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Read;
use std::io::{Cursor, Write};
use std::path::Path;
use vorbis_encoder::Encoder;

/// The number of channels.
const NUM_CHANNELS: usize = 2;
/// Conversion factor for f32 to i16.
const F32_TO_I16: f32 = 32767.5;
/// An ordered list of MP3 bit rates. We can't use `IndexedValues` because this enum isn't serializable.
pub const MP3_BIT_RATES: [Bitrate; 16] = [
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
/// An ordererd list of mp3 qualities. We can't use `IndexedValues` because this enum isn't serializable.
pub const MP3_QUALITIES: [Quality; 10] = [
    Quality::Worst,
    Quality::SecondWorst,
    Quality::Ok,
    Quality::Decent,
    Quality::Good,
    Quality::Nice,
    Quality::VeryNice,
    Quality::NearBest,
    Quality::SecondBest,
    Quality::Best,
];

/// This struct contains all export settings, as well as exporter functions.
/// This struct does *not* write samples to a buffer; that's handled in the `Synthesizer`'s export functions.
/// Rather, this receives a buffer of f32 data, and then decides what to do with it based on the user-defined export settings.
///
/// There are always two copies of the same `Exporter`: One lives in the Synthesizer thread, and one lives on the main thread.
/// The user can edit the main thread `Exporter`, which is then sent to the Synthesizer thread.
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct Exporter {
    /// The framerate.
    pub framerate: U64orF32,
    /// Export metadata.
    pub metadata: Metadata,
    /// If true, write copyright info.
    pub copyright: bool,
    /// The mp3 quality index.
    pub mp3_bit_rate: Index<usize>,
    /// The mp3 quality index.
    pub mp3_quality: Index<usize>,
    /// If true, export to multiple files.
    pub multi_file: bool,
    /// Multi-file suffix setting.
    pub multi_file_suffix: IndexedValues<MultiFileSuffix, 3>,
    /// The .ogg file quality index.
    pub ogg_quality: Index<usize>,
    /// The export type.
    pub export_type: IndexedValues<ExportType, 5>,
    /// Export settings for .mid files.
    pub mid_settings: IndexedValues<ExportSetting, 3>,
    /// Export settings for .wav files.
    pub wav_settings: IndexedValues<ExportSetting, 3>,
    /// Export settings for .mp3 files.
    pub mp3_settings: IndexedValues<ExportSetting, 12>,
    /// Export settings for .ogg files.
    pub ogg_settings: IndexedValues<ExportSetting, 11>,
    /// Export settings for .flac files.
    /// Use a default if the save file is pre-0.1.3
    #[serde(default = "default_flac_settings")]
    pub flac_settings: IndexedValues<ExportSetting, 10>,
}

impl Default for Exporter {
    fn default() -> Self {
        let export_type = IndexedValues::new(
            0,
            [
                ExportType::Wav,
                ExportType::Mid,
                ExportType::MP3,
                ExportType::Ogg,
                ExportType::Flac,
            ],
        );
        let mid_settings = IndexedValues::new(
            0,
            [
                ExportSetting::Title,
                ExportSetting::Artist,
                ExportSetting::Copyright,
            ],
        );
        let wav_settings = IndexedValues::new(
            0,
            [
                ExportSetting::Framerate,
                ExportSetting::MultiFile,
                ExportSetting::MultiFileSuffix,
            ],
        );
        let mp3_settings = IndexedValues::new(
            0,
            [
                ExportSetting::Framerate,
                ExportSetting::Mp3Quality,
                ExportSetting::Mp3BitRate,
                ExportSetting::Title,
                ExportSetting::Artist,
                ExportSetting::Copyright,
                ExportSetting::Album,
                ExportSetting::TrackNumber,
                ExportSetting::Genre,
                ExportSetting::Comment,
                ExportSetting::MultiFile,
                ExportSetting::MultiFileSuffix,
            ],
        );
        let ogg_settings = IndexedValues::new(
            0,
            [
                ExportSetting::Framerate,
                ExportSetting::OggQuality,
                ExportSetting::Title,
                ExportSetting::Artist,
                ExportSetting::Copyright,
                ExportSetting::Album,
                ExportSetting::TrackNumber,
                ExportSetting::Genre,
                ExportSetting::Comment,
                ExportSetting::MultiFile,
                ExportSetting::MultiFileSuffix,
            ],
        );
        let flac_settings = default_flac_settings();
        let multi_file_suffix = IndexedValues::new(
            0,
            [
                MultiFileSuffix::ChannelAndPreset,
                MultiFileSuffix::Preset,
                MultiFileSuffix::Channel,
            ],
        );
        Self {
            framerate: U64orF32::from(DEFAULT_FRAMERATE),
            export_type,
            mp3_bit_rate: Index::new(12, MP3_BIT_RATES.len()),
            mp3_quality: Index::new(9, MP3_QUALITIES.len()),
            ogg_quality: Index::new(9, 10),
            wav_settings,
            mid_settings,
            mp3_settings,
            ogg_settings,
            flac_settings,
            multi_file_suffix,
            metadata: Metadata::default(),
            copyright: false,
            multi_file: false,
        }
    }
}

impl Exporter {
    /// Export to a .mid file.
    /// - `path` Output to this path.
    /// - `music` This is what we're saving.
    /// - `synth_state` We need this for its present names.
    /// - `text` This is is used for metadata.
    /// - `export_settings` .mid export settings.
    pub fn mid(&self, path: &Path, music: &Music, time: &Time, synth_state: &SynthState) {
        // Set the name of the music.
        let mut meta_messages = vec![MetaMessage::Text(self.metadata.title.as_bytes())];
        let mut copyright = vec![];
        // Set the tempo.
        meta_messages.push(MetaMessage::Tempo(u24::from(
            (60000000 / time.bpm.get_u()) as u32,
        )));
        // Set the time signature.
        meta_messages.push(MetaMessage::TimeSignature(4, 2, 24, 8));
        // Send copyright.
        if self.copyright {
            if let Some(artist) = &self.metadata.artist {
                copyright.append(&mut self.get_copyright(artist).as_bytes().to_vec());
                meta_messages.push(MetaMessage::Copyright(&copyright));
            }
        }

        let mut tracks = vec![];
        let mut track_0 = Track::new();
        for (i, midi_track) in music.midi_tracks.iter().enumerate() {
            if let Some(program) = synth_state.programs.get(&midi_track.channel) {
                // Get track 0 or start a new track.
                let mut track = Vec::new();

                if i == 0 {
                    for meta_message in meta_messages.iter() {
                        track_0.push(TrackEvent {
                            delta: 0.into(),
                            kind: TrackEventKind::Meta(*meta_message),
                        })
                    }
                }

                let channel = u4::from(midi_track.channel);

                // Set the program name.
                track.push(TrackEvent {
                    delta: 0.into(),
                    kind: TrackEventKind::Meta(MetaMessage::ProgramName(
                        program.preset_name.as_bytes(),
                    )),
                });
                // Change the program.
                track.push(TrackEvent {
                    delta: 0.into(),
                    kind: TrackEventKind::Midi {
                        channel,
                        message: MidiMessage::ProgramChange {
                            program: program.preset.into(),
                        },
                    },
                });

                // Iterate through the notes.
                let mut notes = midi_track.notes.clone();
                // Sort the notes by start time.
                notes.sort_by(|a, b| a.start.cmp(&b.start));
                // Get the start and end time.
                let t0 = notes.iter().map(|n| n.start).min().unwrap();
                // The delta is the first note.
                let mut dt = t0;
                let t1 = notes.iter().map(|n| n.end).max().unwrap();
                // Iterate through all pulses.
                for t in t0..t1 {
                    // Get all note-on events.
                    for note in notes.iter().filter(|n| n.start == t) {
                        let delta = Self::get_delta_time(&mut dt);
                        track.push(TrackEvent {
                            delta,
                            kind: TrackEventKind::Midi {
                                channel,
                                message: MidiMessage::NoteOn {
                                    key: note.note.into(),
                                    vel: note.velocity.into(),
                                },
                            },
                        });
                    }
                    // Get all note-off events.
                    for note in notes.iter().filter(|n| n.end == t) {
                        let delta = Self::get_delta_time(&mut dt);
                        track.push(TrackEvent {
                            delta,
                            kind: TrackEventKind::Midi {
                                channel,
                                message: MidiMessage::NoteOff {
                                    key: note.note.into(),
                                    vel: note.velocity.into(),
                                },
                            },
                        });
                    }
                }
                // End the track.
                track.push(TrackEvent {
                    delta: 0.into(),
                    kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
                });
                // Add the track.
                tracks.push(track);
            }
        }
        // Create the header.
        let header = Header::new(Format::Parallel, Timing::Metrical(u15::from(PPQ_U as u16)));
        // Write the file.
        let mut buffer: Vec<u8> = vec![];
        if let Err(error) = write_std(&header, tracks.iter(), &mut buffer) {
            panic!("Error writing {:?} {:?}", path, error);
        }
        Self::write_file(path, &buffer);
    }

    /// Export to a .wav file.
    ///
    /// - `path` The output path.
    /// - `buffer` A buffer of wav data.
    pub(crate) fn wav(&self, path: &Path, buffer: &AudioBuffer) {
        // Get the spec.
        let spec = WavSpec {
            channels: NUM_CHANNELS as u16,
            sample_rate: self.framerate.get_u() as u32,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };
        // Write.
        let mut writer = WavWriter::create(path, spec).unwrap();
        let mut i16_writer = writer.get_i16_writer(buffer[0].len() as u32 * (NUM_CHANNELS as u32));
        for (l, r) in buffer[0].iter().zip(buffer[1].iter()) {
            i16_writer.write_sample(Self::to_i16(l));
            i16_writer.write_sample(Self::to_i16(r));
        }
        i16_writer.flush().unwrap();
        writer.finalize().unwrap();
    }

    /// Export to a .mp3 file.
    ///
    /// - `path` The output path.
    /// - `buffer` A buffer of wav data.
    pub(crate) fn mp3<'a, T: 'a>(&self, path: &Path, buffer: &'a [Vec<T>; NUM_CHANNELS])
    where
        mp3lame_encoder::DualPcm<'a, T>: mp3lame_encoder::EncoderInput,
    {
        // Create the encoder.
        let mut mp3_encoder = Builder::new().expect("Create LAME builder");
        mp3_encoder
            .set_num_channels(NUM_CHANNELS as u8)
            .expect("Set channels");
        mp3_encoder
            .set_sample_rate(self.framerate.get_u() as u32)
            .expect("Set sample rate");
        mp3_encoder
            .set_brate(MP3_BIT_RATES[self.mp3_bit_rate.get()])
            .expect("Set bitrate");
        mp3_encoder
            .set_quality(MP3_QUALITIES[self.mp3_quality.get()])
            .expect("Set quality");
        // Build the encoder.
        let mut mp3_encoder = mp3_encoder.build().expect("To initialize LAME encoder");
        // Get the input.
        let input = DualPcm {
            left: &buffer[0],
            right: &buffer[1],
        };
        // Get the output buffer.
        let mut mp3_out_buffer = Vec::with_capacity(max_required_buffer_size(buffer[0].len()));
        // Get the size.
        let encoded_size = mp3_encoder
            .encode(input, mp3_out_buffer.spare_capacity_mut())
            .expect("To encode");
        unsafe {
            mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
        }
        let encoded_size = mp3_encoder
            .flush::<FlushNoGap>(mp3_out_buffer.spare_capacity_mut())
            .expect("To flush");
        unsafe {
            mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
        }
        // Write the file.
        Self::write_file(path, &mp3_out_buffer);
        // Write the tag.
        let time = Local::now();
        let mut tag = Tag::new();
        tag.set_year(time.year());
        tag.set_title(&self.metadata.title);
        if let Some(artist) = &self.metadata.artist {
            tag.set_artist(artist);
        }
        if let Some(album) = &self.metadata.album {
            tag.set_album(album);
        }
        if let Some(genre) = &self.metadata.genre {
            tag.set_genre(genre);
        }
        if let Some(comment) = &self.metadata.comment {
            tag.set_genre(comment);
        }
        if let Some(track_number) = &self.metadata.track_number {
            tag.set_track(*track_number);
        }
        if let Err(error) = tag.write_to_path(path, Version::Id3v24) {
            panic!("Error writing ID3 tag to {:?}: {}", path, error);
        }
    }

    /// Export to an .ogg file.
    ///
    /// - `path` The output path.
    /// - `buffer` A buffer of wav data.
    pub(crate) fn ogg(&self, path: &Path, buffer: &AudioBuffer) {
        let mut samples = vec![];
        for (l, r) in buffer[0].iter().zip(buffer[1].iter()) {
            samples.push(Self::to_i16(l));
            samples.push(Self::to_i16(r));
        }
        let mut encoder = Encoder::new(
            NUM_CHANNELS as u32,
            self.framerate.get_u(),
            (self.ogg_quality.get() as f32 / 9.0) * 1.2 - 0.2,
        )
        .expect("Error creating .ogg file encoder.");
        let samples = encoder
            .encode(&samples)
            .expect("Error encoding .ogg samples.");
        // Get a cursor.
        let cursor = Cursor::new(&samples);
        // Write the comments.
        let mut comments = CommentHeader::new();
        comments.set_vendor("Ogg");
        comments.add_tag_single("title", &self.metadata.title);
        comments.add_tag_single("date", &Local::now().year().to_string());
        if let Some(artist) = &self.metadata.artist {
            comments.add_tag_single("artist", artist);
            if self.copyright {
                comments.add_tag_single("copyright", &self.get_copyright(artist));
            }
        }
        if let Some(album) = &self.metadata.album {
            comments.add_tag_single("album", album);
        }
        if let Some(genre) = &self.metadata.genre {
            comments.add_tag_single("genre", genre);
        }
        if let Some(track_number) = &self.metadata.track_number {
            comments.add_tag_single("tracknumber", &track_number.to_string());
        }
        if let Some(comment) = &self.metadata.genre {
            comments.add_tag_single("description", comment);
        }
        // Write the comments.
        let mut out = vec![];
        replace_comment_header(cursor, comments)
            .read_to_end(&mut out)
            .expect("Error reading cursor.");
        // Write the file.
        Self::write_file(path, &out);
    }

    /// Encode to flac.
    pub(crate) fn flac(&self, path: &Path, buffer: &AudioBuffer) {
        // Convert to i32.
        let mut samples = vec![];
        for (left, right) in buffer[0].iter().zip(buffer[1].iter()) {
            samples.push(Self::to_i32(left));
            samples.push(Self::to_i32(right));
        }
        let config = FlacEncoder::default();
        let source =
            MemSource::from_samples(&samples, NUM_CHANNELS, 16, self.framerate.get_u() as usize);
        match encode_with_fixed_block_size(&config, source, config.block_sizes[0]) {
            Ok(flac_stream) => {
                let mut sink = ByteSink::new();
                flac_stream.write(&mut sink).unwrap();
                // Write the file.
                Self::write_file(path, sink.as_slice());
                // Write the tag.
                let mut tag = FlacTag::read_from_path(path).unwrap();
                tag.set_vorbis("title", vec![self.metadata.title.clone()]);
                tag.set_vorbis("date", vec![Local::now().year().to_string()]);
                if let Some(artist) = &self.metadata.artist {
                    tag.set_vorbis("artist", vec![artist.clone()]);
                    if self.copyright {
                        tag.set_vorbis("copyright", vec![self.get_copyright(artist)]);
                    }
                }
                if let Some(album) = &self.metadata.album {
                    tag.set_vorbis("album", vec![album.clone()]);
                }
                if let Some(genre) = &self.metadata.genre {
                    tag.set_vorbis("genre", vec![genre.clone()]);
                }
                if let Some(track_number) = &self.metadata.track_number {
                    tag.set_vorbis("track_number", vec![track_number.to_string()]);
                }
                if let Some(comment) = &self.metadata.genre {
                    tag.set_vorbis("description", vec![comment.clone()]);
                }
                // Save the tag.
                tag.save().unwrap();
            }
            Err(error) => panic!("Error encoding flac: {:?}", error),
        }
    }

    /// Write samples to a file.
    fn write_file(path: &Path, samples: &[u8]) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(false)
            .truncate(true)
            .create(true)
            .open(path)
            .expect("Error opening file {:?}");
        file.write_all(samples)
            .expect("Failed to write samples to file.");
    }

    /// Converts a PPQ value into a MIDI time delta and resets `ppq` to zero.
    fn get_delta_time(ppq: &mut u64) -> u28 {
        // Get the dt.
        let dt = (*ppq as f32 / PPQ_F) as u32;
        // Reset the PPQ value.
        *ppq = 0;
        u28::from(dt)
    }

    /// Converts an f32 sample to an i16 sample.
    fn to_i16(sample: &f32) -> i16 {
        (sample * F32_TO_I16).floor() as i16
    }

    /// Converts an f32 sample to an i32 sample.
    fn to_i32(sample: &f32) -> i32 {
        (sample * F32_TO_I16).floor() as i32
    }

    /// Returns a copyright string.
    fn get_copyright(&self, artist: &str) -> String {
        format!("Copyright {} {}", Local::now().year(), artist)
    }
}

fn default_flac_settings() -> IndexedValues<ExportSetting, 10> {
    IndexedValues::new(
        0,
        [
            ExportSetting::Framerate,
            ExportSetting::Title,
            ExportSetting::Artist,
            ExportSetting::Copyright,
            ExportSetting::Album,
            ExportSetting::TrackNumber,
            ExportSetting::Genre,
            ExportSetting::Comment,
            ExportSetting::MultiFile,
            ExportSetting::MultiFileSuffix,
        ],
    )
}
