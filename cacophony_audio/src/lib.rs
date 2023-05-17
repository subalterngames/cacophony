mod command;
mod conn;
mod message;
mod player;
mod program;
mod synth_state;
mod synthesizer;
mod time_state;

pub mod time;

pub use crate::command::Command;
pub use crate::conn::Conn;
pub use crate::message::{AudioMessage, CommandsMessage, ExportedAudio};
pub(crate) use crate::time_state::TimeState;
use crate::program::Program;
pub use crate::synth_state::SynthState;
use crossbeam_channel::{bounded, unbounded};
use player::Player;
use std::thread::spawn;

/// Start the synthesizer and the audio player. Returns a `conn`.
pub fn connect() -> Conn {
    let (send_commands, recv_commands) = unbounded();
    let (send_state, recv_state) = bounded(1);
    let (send_audio, recv_audio) = bounded(1);
    let (send_time, recv_time) = bounded(1);
    let (send_exported_audio, recv_exported_audio) = unbounded();

    // Spawn the synthesizer thread.
    spawn(move || {
        synthesizer::Synthesizer::start(recv_commands, send_audio, send_state, send_exported_audio, send_time)
    });
    // Spawn the audio thread.
    let player = Player::new(recv_audio);
    // Get the conn.
    Conn {
        state: SynthState::default(),
        _player: player,
        send_commands,
        recv: recv_state,
        recv_exported_audio,
        recv_time,
    }
}

#[cfg(test)]
mod tests {
    use crate::{connect, Command, CommandsMessage};
    use std::thread::sleep;
    use std::time::Duration;

    const SF_PATH: &str = "tests/CT1MBGMRSV1.06.sf2";
    const CHANNEL: u8 = 0;
    const DURATION: u64 = 44100;
    const KEY: u8 = 60;
    const VELOCITY: u8 = 120;

    #[test]
    fn conn() {
        let conn = connect();
        assert!(conn._player.is_some());
    }

    #[test]
    fn sf() {
        // Make sure we can load the file.
        assert!(std::fs::File::open(SF_PATH).is_ok());
        let mut conn = connect();
        let commands = vec![Command::LoadSoundFont {
            path: SF_PATH.to_string(),
            channel: CHANNEL,
        }];
        // Make sure we can send commands.
        conn.send(commands);
        assert!(conn.state.programs.contains_key(&CHANNEL));
        let program = &conn.state.programs[&CHANNEL];
        assert_eq!(program.num_banks, 1);
        assert_eq!(program.bank_index, 0);
        assert_eq!(program.num_presets, 128);
        assert_eq!(program.preset_index, 0);
        assert_eq!(program.preset_name, "Piano 1");
    }

    #[test]
    fn audio() {
        let mut conn = connect();
        // Load the soundfont. set the program, and do a note-on.
        let path = SF_PATH.to_string();
        conn.send(vec![
            Command::LoadSoundFont {
                path: path.clone(),
                channel: CHANNEL,
            },
            Command::SetProgram {
                channel: CHANNEL,
                path,
                bank_index: 0,
                preset_index: 0,
            },
            Command::NoteOn {
                channel: CHANNEL,
                key: KEY,
                velocity: VELOCITY,
                duration: DURATION,
            },
        ]);
        // Listen!
        sleep(Duration::from_millis(500));
        // Schedule some events.
        let commands = get_commands();
        conn.send(commands);
        // Listen!
        sleep(Duration::from_secs(10));
        // Export.
        let exported_audio = conn.export(get_commands());
        assert!(exported_audio.len() > 0);
    }

    fn get_commands() -> CommandsMessage {
        let dt = DURATION / 4;
        let num: u8 = 10;
        (0..num)
            .map(|i| Command::NoteOnAt {
                channel: CHANNEL,
                key: KEY + i,
                velocity: VELOCITY,
                time: DURATION * 3 + dt * i as u64,
                duration: DURATION,
            })
            .collect::<CommandsMessage>()
    }
}
