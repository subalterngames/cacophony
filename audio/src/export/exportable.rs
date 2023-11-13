use crate::midi_event_queue::MidiEventQueue;

pub(crate) struct Exportable {
    pub events: MidiEventQueue,
    pub total_samples: u64,
    pub suffix: Option<String>,
}
