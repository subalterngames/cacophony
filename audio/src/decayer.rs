use crate::SharedSynth;
use oxisynth::Synth;

/// Export this many bytes per decay chunk.
const DECAY_CHUNK_SIZE: usize = 4096;
///  Oxisynth usually doesn't zero out its audio. This is essentially an epsilon.
/// This is used to detect if the export is done.
const SILENCE: f32 = 1e-7;

/// Write audio samples during a decay.
pub(crate) struct Decayer {
    pub buffer: [f32; DECAY_CHUNK_SIZE],
    buffer_1: [f32; DECAY_CHUNK_SIZE],
    pub decaying: bool,
}

impl Default for Decayer {
    fn default() -> Self {
        Self {
            buffer: [0.0; DECAY_CHUNK_SIZE],
            buffer_1: [0.0; DECAY_CHUNK_SIZE],
            decaying: false,
        }
    }
}

impl Decayer {
    pub fn decay_shared(&mut self, synth: &SharedSynth, len: usize) {
        for sample in self.buffer[0..Self::get_len(len)].chunks_mut(2) {
            let mut synth = synth.lock();
            synth.write(sample);
        }
        self.set_decaying(len);
    }

    pub fn decay_two_channels(
        &mut self,
        left: &mut Vec<f32>,
        right: &mut Vec<f32>,
        synth: &mut Synth,
    ) {
        // Write samples.
        synth.write((self.buffer.as_mut(), self.buffer_1.as_mut()));
        left.extend(self.buffer);
        right.extend(self.buffer_1);
        self.decaying = self.buffer.iter().any(|s| s.abs() > SILENCE)
            || self.buffer_1.iter().any(|s| s.abs() > SILENCE);
    }

    fn set_decaying(&mut self, len: usize) {
        self.decaying = self.buffer[0..Self::get_len(len)]
            .iter()
            .any(|s| s.abs() > SILENCE);
    }

    fn get_len(len: usize) -> usize {
        if len <= DECAY_CHUNK_SIZE {
            len
        } else {
            DECAY_CHUNK_SIZE
        }
    }
}
