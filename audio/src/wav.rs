const HEADER_SIZE: usize = 44;
const F32_TO_I16: f32 = 32767.5;
const CHUNKSIZE: u32 = 16;
const FORMAT: u16 = 1;
const SAMPLE_RATE: u32 = 44100;
const CHANNELS: u16 = 2;
const BIT_DEPTH: u16 = 16;
const BYTE_RATE: u32 = SAMPLE_RATE * BIT_DEPTH as u32 / 8;
const BLOCK_ALIGN: u16 = BIT_DEPTH / 2;

/// Converts an f32 sample to an i16 sample.
pub(crate) fn to_i16(sample: f32) -> i16 {
    (sample * F32_TO_I16).floor() as i16
}

/// Returns a new wav header.
///
/// - `num_samples` The number of samples in the wav file.
///
/// Source: https://www.simonwenkel.com/notes/programming_languages/rust/writing-files-with-rust-wav-file-example.html
pub(crate) fn get_wav_header(num_samples: u64) -> [u8; HEADER_SIZE] {
    let size = num_samples as u32;
    let mut header = [0; HEADER_SIZE];
    let mut i = 0;
    // Header.
    insert_str(&mut i, &mut header, "RIFF");
    insert_u32(&mut i, &mut header, size + HEADER_SIZE as u32);
    insert_str(&mut i, &mut header, "WAVE");
    // Format.
    insert_str(&mut i, &mut header, "fmt ");
    insert_u32(&mut i, &mut header, CHUNKSIZE);
    insert_u16(&mut i, &mut header, FORMAT);
    insert_u16(&mut i, &mut header, CHANNELS);
    insert_u32(&mut i, &mut header, SAMPLE_RATE);
    insert_u32(&mut i, &mut header, BYTE_RATE);
    insert_u16(&mut i, &mut header, BLOCK_ALIGN);
    insert_u16(&mut i, &mut header, BIT_DEPTH);
    // Data.
    insert_str(&mut i, &mut header, "data");
    insert_u32(&mut i, &mut header, size);
    header
}

fn insert_str(i: &mut usize, header: &mut [u8], value: &str) {
    for byte in value.as_bytes() {
        header[*i] = *byte;
        *i += 1;
    }
}

fn insert_u16(i: &mut usize, header: &mut [u8], value: u16) {
    for byte in value.to_le_bytes() {
        header[*i] = byte;
        *i += 1;
    }
}

fn insert_u32(i: &mut usize, header: &mut [u8], value: u32) {
    for byte in value.to_le_bytes() {
        header[*i] = byte;
        *i += 1;
    }
}
