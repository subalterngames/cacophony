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
        self.events.sort()
    }

    /// Dequeue any events that start at `time`.
    pub(crate) fn dequeue(&mut self, time: u64) -> Vec<MidiEvent> {
        let mut midi_events = vec![];
        while !self.events.is_empty() && self.events[0].time == time {
            midi_events.push(self.events.remove(0).event);
        }
        midi_events
    }
}
