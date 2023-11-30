use super::timed_event::TimedEvent;
use crate::{event_type::EventType, program::Program};
use common::{Effect, EffectType, Note, Time};

/// A queue of timed events.
#[derive(Default)]
pub(crate) struct EventQueue {
    /// The events. Assume that this is sorted.
    events: Vec<TimedEvent>,
}

impl EventQueue {
    /// Enqueue notes and events.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn enqueue(
        &mut self,
        channel: u8,
        program: &Program,
        notes: &[Note],
        effects: &[Effect],
        time: &Time,
        framerate: f32,
        end_time: &mut u64,
    ) {
        // Enqueue default program values.
        self.enqueue_event(
            0,
            EventType::Effect {
                channel,
                effect: EffectType::Chorus(program.chorus as u16),
            },
        );
        self.enqueue_event(
            0,
            EventType::Effect {
                channel,
                effect: EffectType::Pan(program.pan as i16),
            },
        );
        self.enqueue_event(
            0,
            EventType::Effect {
                channel,
                effect: EffectType::Reverb(program.reverb as u16),
            },
        );
        // Enqueue notes.
        for note in notes.iter() {
            // Note-on event.
            self.enqueue_event(
                time.ppq_to_samples(note.start, framerate),
                EventType::NoteOn {
                    channel,
                    key: note.note,
                    vel: note.velocity,
                },
            );
            // Note-off event.
            let note_off_time = time.ppq_to_samples(note.end, framerate);
            if *end_time < note_off_time {
                *end_time = note_off_time;
            }
            self.enqueue_event(
                note_off_time,
                EventType::NoteOff {
                    channel,
                    key: note.note,
                },
            );
        }
        // Enqueue effects.
        for effect in effects.iter() {
            self.enqueue_event(
                time.ppq_to_samples(effect.time, framerate),
                EventType::Effect {
                    channel,
                    effect: effect.effect,
                },
            );
        }
    }

    /// Enqueue a new MIDI event.
    ///
    /// - `time` The start time of the event in number of samples.
    /// - `event` The MIDI event.
    fn enqueue_event(&mut self, time: u64, event: EventType) {
        // Add the event.
        self.events.push(TimedEvent { time, event });
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
    pub(crate) fn dequeue(&mut self, time: u64) -> Vec<EventType> {
        let mut events = vec![];
        while !self.events.is_empty() && self.events[0].time == time {
            events.push(self.events.remove(0).event);
        }
        events
    }
}
