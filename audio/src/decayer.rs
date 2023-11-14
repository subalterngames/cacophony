use crate::SharedSynth;
use oxisynth::Synth;

/// Export this many bytes per decay chunk.
pub(crate) const DECAY_CHUNK_SIZE: usize = 2048;
///  Oxisynth usually doesn't zero out its audio. This is essentially an epsilon.
/// This is used to detect if the export is done.
const SILENCE: f32 = 1e-7;

/// Write audio samples during a decay.
pub(crate) struct Decayer {
    pub left: [f32; DECAY_CHUNK_SIZE],
    pub right: [f32; DECAY_CHUNK_SIZE],
    pub decaying: bool,
}

impl Default for Decayer {
    fn default() -> Self {
        Self {
            left: [0.0; DECAY_CHUNK_SIZE],
            right: [0.0; DECAY_CHUNK_SIZE],
            decaying: false,
        }
    }
}

impl Decayer {
    pub fn decay(&mut self, synth: &mut Synth, len: usize) {
        // Write to the decay chunks.
        synth.write((self.left[0..len].as_mut(), self.right[0..len].as_mut()));
        self.set_decaying(len);
    }

    pub fn decay_shared(&mut self, synth: &SharedSynth, len: usize) {
        for (left, right) in self.left[0..len]
            .iter_mut()
            .zip(self.right[0..len].as_mut())
        {
            let mut synth = synth.lock();
            (*left, *right) = synth.read_next();
        }
        self.set_decaying(len);
    }

    fn set_decaying(&mut self, len: usize) {
        self.decaying = self.left[0..len].iter().any(|s| s.abs() > SILENCE)
            || self.right[0..len].iter().any(|s| s.abs() > SILENCE);
    }
}
