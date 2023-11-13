use super::timed_midi_event::TimedMidiEvent;
use oxisynth::MidiEvent;

/// A queue of timed MIDI events.
#[derive(Default)]
pub(crate) struct MidiEventQueue {
    /// The events. Assume that this is sorted.
    events: Vec<TimedMidiEvent>
}

impl MidiEventQueue {
    /// Enqueue a new MIDI event. This will also sort the internal list of events.
    /// 
    /// - `time` The start time of the event in number of samples.
    /// - `event` The MIDI event.
    pub(crate) fn enqueue(&mut self, time: u64, event: MidiEvent) {
        // Add the event.
        self.events.push(TimedMidiEvent { time, event });
        // Sort the events.
        self.events.sort_by(|a, b| a.time.cmp(&b.time))
    }

    /// Dequeue any events that start at `time`.
    pub(crate) fn dequeue(&mut self, time: u64) -> Vec<MidiEvent> {
        let midi_events = self.events.iter().filter(|e| e.time == time).map(|e| e.event).collect::<Vec<MidiEvent>>();
        if !midi_events.is_empty() {
            self.events.retain(|e| e.time != time);
        }
        midi_events
    }
}