use crate::SharedSynth;
use oxisynth::Synth;

/// Export this many bytes per decay chunk.
pub(crate) const DECAY_CHUNK_SIZE: usize = 4096;
///  Oxisynth usually doesn't zero out its audio. This is essentially an epsilon.
/// This is used to detect if the export is done.
const SILENCE: f32 = 1e-7;

/// Write audio samples during a decay.
pub(crate) struct Decayer {
    pub buffer: [f32; DECAY_CHUNK_SIZE],
    pub decaying: bool,
}

impl Default for Decayer {
    fn default() -> Self {
        Self {
            buffer: [0.0; DECAY_CHUNK_SIZE],
            decaying: false,
        }
    }
}

impl Decayer {
    pub fn decay_shared(&mut self, synth: &SharedSynth, len: usize) {
        for sample in self.buffer[0..len].chunks_mut(2) {
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
        len: usize,
    ) {
        let i = left.len();
        // Resize the output vectors.
        let new_len = i + len;
        left.resize(new_len, 0.0);
        right.resize(new_len, 0.0);
        // Write samples.
        synth.write((left[i..new_len].as_mut(), right[i..new_len].as_mut()));
        self.decaying = left[i..len].iter().any(|s| s.abs() > SILENCE)
            || right[i..len].iter().any(|s| s.abs() > SILENCE);
    }

    fn set_decaying(&mut self, len: usize) {
        self.decaying = self.buffer[0..len].iter().any(|s| s.abs() > SILENCE);
    }
}
