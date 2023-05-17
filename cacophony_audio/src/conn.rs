use crate::{Command, CommandsMessage, ExportedAudio, Player, SynthState, TimeState};
use crossbeam_channel::{Receiver, Sender};

/// The connects used by an external function.
pub struct Conn {
    /// The state (as far as we know). This is received from the synthesizer.
    pub state: SynthState,
    /// The audio player. This is here so we don't drop it.
    pub(crate) _player: Option<Player>,
    /// Send commands to the synthesizer.
    pub(crate) send_commands: Sender<CommandsMessage>,
    /// Receive the program state.
    pub(crate) recv: Receiver<SynthState>,
    /// Receive exported audio.
    pub(crate) recv_exported_audio: Receiver<ExportedAudio>,
    /// Receive the updated time.
    pub(crate) recv_time: Receiver<TimeState>,
}

impl Conn {
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

    /// Send the note-on commands with an export command. Block until we export the audio.
    /// 
    /// - `commands` The commands that we'll send.
    pub fn export(&mut self, commands: CommandsMessage) -> ExportedAudio {
        // Export.
        self.send(vec![Command::Export { commands }]);
        // Block until we get audio.
        match self.recv_exported_audio.recv() {
            Ok(audio) => audio,
            Err(error) => panic!("Failed to receive exported audio! {}", error),
        }
    }

    /// Try to update the time.
    pub fn update_time(&mut self) {
        if let Ok(time) = self.recv_time.try_recv() {
            self.state.time = time;
        }
    }
}
