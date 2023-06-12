/// The state of audio that is being exported.
#[derive(Eq, PartialEq, Copy, Clone)]
pub struct ExportState {
    /// The number of samples that have been exported.
    pub exported: u64,
    /// The total number of samples.
    pub samples: u64,
}

impl ExportState {
    pub fn new(samples: u64) -> Self {
        Self {
            exported: 0,
            samples,
        }
    }
}
