use crate::{CommandsMessage, ExportState, Player, SynthState, TimeState};
use crossbeam_channel::{Receiver, Sender};

/// The connects used by an external function.
pub struct Conn {
    /// The state (as far as we know). This is received from the synthesizer.
    pub state: SynthState,
    /// The current export state, if any.
    pub export_state: Option<ExportState>,
    /// The audio player. This is here so we don't drop it.
    _player: Option<Player>,
    /// Send commands to the synthesizer.
    send_commands: Sender<CommandsMessage>,
    /// Receive the program state.
    recv: Receiver<SynthState>,
    /// Receive the export state.
    recv_export: Receiver<ExportState>,
    /// Receive the updated time.
    recv_time: Receiver<TimeState>,
}

impl Conn {
    pub(crate) fn new(
        player: Option<Player>,
        send_commands: Sender<CommandsMessage>,
        recv: Receiver<SynthState>,
        recv_export: Receiver<ExportState>,
        recv_time: Receiver<TimeState>,
    ) -> Self {
        Self {
            state: SynthState::default(),
            export_state: None,
            _player: player,
            send_commands,
            recv,
            recv_export,
            recv_time,
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
        if let Ok(time) = self.recv_time.try_recv() {
            self.state.time = time;
        }
        if let Ok(export_state) = self.recv_export.try_recv() {
            self.export_state = Some(export_state)
        }
    }

     /// Try to update the time.
     pub fn update_time(&mut self) {
        if let Ok(time) = self.recv_time.try_recv() {
            self.state.time = time;
        }
    }
}
