use crate::event_queue::EventQueue;

pub(crate) struct Exportable {
    pub events: EventQueue,
    pub total_samples: u64,
    pub suffix: Option<String>,
}
