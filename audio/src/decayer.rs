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
        // If the decay chunks are totally silent then we're not decaying anymore.
        self.decaying = self.left.iter().any(|s| s.abs() > SILENCE)
            || self.right.iter().any(|s| s.abs() > SILENCE);
    }
}
