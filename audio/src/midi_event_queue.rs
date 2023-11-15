use super::timed_midi_event::TimedMidiEvent;
use oxisynth::MidiEvent;

/// A queue of timed MIDI events.
#[derive(Default)]
pub(crate) struct MidiEventQueue {
    /// The events. Assume that this is sorted.
    events: Vec<TimedMidiEvent>,
}

impl MidiEventQueue {
    /// Enqueue a new MIDI event.
    ///
    /// - `time` The start time of the event in number of samples.
    /// - `event` The MIDI event.
    pub(crate) fn enqueue(&mut self, time: u64, event: MidiEvent) {
        // Add the event.
        self.events.push(TimedMidiEvent { time, event });
    }

    pub(crate) fn get_next_time(&self) -> Option<u64> {
        if self.events.is_empty() {
            None
        } else {
            Some(self.events[0].time)
        }
    }

    /// Sort the list of events by start time.
    pub(crate) fn sort(&mut self) {
        self.events.sort_by(|a, b| a.time.cmp(&b.time))
    }

    /// Dequeue any events that start at `time`.
    pub(crate) fn dequeue(&mut self, time: u64) -> Vec<MidiEvent> {
        let midi_events = self
            .events
            .iter()
            .filter(|e| e.time == time)
            .map(|e| e.event)
            .collect::<Vec<MidiEvent>>();
        if !midi_events.is_empty() {
            self.events.retain(|e| e.time != time);
        }
        midi_events
    }
}
