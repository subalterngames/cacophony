#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ExportState {
    NotExporting,
    /// Writing samples to a wav buffer.
    WritingWav {
        total_samples: u64,
        exported_samples: u64,
    },
    /// Writing decay to a wav buffer while the audio decays.
    AppendingDecay,
    /// Converting the wav buffer to another file type and write to disk.
    WritingToDisk,
    /// Done exporting.
    Done,
}
