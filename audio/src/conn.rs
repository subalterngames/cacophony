use crate::{
    AudioMessage, Command, CommandsMessage, ExportState, Player, SharedMidiEventQueue, SharedSynth,
    SharedTimeState, SynthState, TimeState, types::SharedSample,
};
use common::State;
use crossbeam_channel::{Receiver, Sender};
use oxisynth::MidiEvent;

/// The connects used by an external function.
pub struct Conn {
    /// The state (as far as we know). This is received from the synthesizer.
    pub state: SynthState,
    /// The current export state, if any.
    pub export_state: Option<ExportState>,
    /// The playback framerate.
    framerate: f32,
    /// The audio player. This is here so we don't drop it.
    _player: Option<Player>,
    /// Send commands to the synthesizer.
    send_commands: Sender<CommandsMessage>,
    /// Receive the program state.
    recv: Receiver<SynthState>,
    /// Receive the export state.
    recv_export: Receiver<Option<ExportState>>,
    /// Receive an audio sample.
    recv_sample: Receiver<AudioMessage>,
    /// The most recent sample.
    pub sample: SharedSample,
    synth: SharedSynth,
    midi_event_queue: SharedMidiEventQueue,
    time_state: SharedTimeState,
}

impl Conn {
    pub(crate) fn new(
        player: Option<Player>,
        send_commands: Sender<CommandsMessage>,
        recv: Receiver<SynthState>,
        recv_export: Receiver<Option<ExportState>>,
        recv_time: Receiver<TimeState>,
        recv_sample: Receiver<AudioMessage>,
        synth: SharedSynth,
        midi_event_queue: SharedMidiEventQueue,
        time_state: SharedTimeState,
        sample: SharedSample
    ) -> Self {
        let framerate = match &player {
            Some(player) => player.framerate as f32,
            None => 0.0,
        };
        Self {
            state: SynthState::default(),
            export_state: None,
            _player: player,
            send_commands,
            recv,
            recv_export,
            recv_sample,
            framerate,
            sample,
            synth,
            midi_event_queue,
            time_state
        }
    }

    /// Try to send commands and receive a `SynthState`, which updates `self.state.
    ///
    /// - `commands` The commands that we'll send.
    pub fn send(&mut self, commands: CommandsMessage) {
        match self.send_commands.send(commands) {
            Ok(_) => (),
            Err(error) => panic!("Error sending commands: {}", error),
        }
        // Update the state.
        if let Ok(state) = self.recv.recv() {
            self.state = state;
        }
    }

    /// Call this once per frame.
    pub fn update(&mut self) {
        // Get the export state.
        if self.export_state.is_some() {
            self.send(vec![Command::SendExportState]);
            if let Ok(export_state) = self.recv_export.recv() {
                self.export_state = export_state;
            }
        }
    }

    /// Schedule MIDI events and start to play music.
    pub fn schedule_music(&mut self, state: &State) {
        // Get the start time.
        let start = state
            .time
            .ppq_to_samples(state.time.playback, self.framerate);

        // Set the playback framerate.
        let mut synth = self.synth.lock();
        synth.set_sample_rate(self.framerate);

        // Enqueue note events.
        let mut midi_event_queue = self.midi_event_queue.lock();
        for track in state.music.midi_tracks {
            for note in track.get_playback_notes(start) {
                // Note-on event.
                midi_event_queue.enqueue(
                    state.time.ppq_to_samples(note.start, self.framerate),
                    MidiEvent::NoteOn {
                        channel: track.channel,
                        key: note.note,
                        vel: note.velocity,
                    },
                );
                // Note-off event.
                midi_event_queue.enqueue(
                    state.time.ppq_to_samples(note.end, self.framerate),
                    MidiEvent::NoteOff {
                        channel: track.channel,
                        key: note.note,
                    },
                );
            }
        }
        // Sort the events by start time.
        midi_event_queue.sort();

        // Set time itself.
        let mut time_state = self.time_state.lock();
        time_state.music = true;
        time_state.time = Some(start);
    }
}
