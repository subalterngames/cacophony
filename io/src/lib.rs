use audio::Conn;
use common::hashbrown::HashMap;
use common::ini::Ini;
use common::{InputState, PanelType, Paths, State};
use input::{Input, InputEvent};
use text::{Text, TTS};
mod io_command;
mod music_panel;
mod open_file;
mod panel;
mod piano_roll;
mod tooltip;
mod tracks_panel;
mod undo_state;
use io_command::IOCommand;
pub(crate) use io_command::IOCommands;
use music_panel::MusicPanel;
use open_file::open_file_panel::OpenFilePanel;
use open_file::open_file_type::OpenFileType;
pub(crate) use panel::Panel;
use piano_roll::PianoRollPanel;
pub(crate) use tooltip::{get_tooltip, get_tooltip_with_values};
use tracks_panel::TracksPanel;
pub(crate) use undo_state::UndoRedoState;

/// The maximum size of the undo stack.
const MAX_UNDOS: usize = 100;

pub struct IO {
    /// A stack of states that can be popped to undo an action.
    undo: Vec<UndoRedoState>,
    /// A stack of states that can be popped to redo an action.
    redo: Vec<UndoRedoState>,
    /// Top-level text-to-speech lookups.
    tts: HashMap<InputEvent, String>,
    /// The music panel.
    music_panel: MusicPanel,
    /// The tracks panel.
    tracks_panel: TracksPanel,
    /// The open-file panel.
    open_file_panel: OpenFilePanel,
    /// The piano roll panel.
    piano_roll_panel: PianoRollPanel,
}

impl IO {
    pub fn new(config: &Ini, input: &Input, input_state: &InputState, text: &Text) -> Self {
        let mut tts = HashMap::new();
        // App TTS.
        let app = get_tooltip(
            "APP_TTS",
            &[
                InputEvent::PanelTTS,
                InputEvent::SubPanelTTS,
                InputEvent::AppTTS,
                InputEvent::FileTTS,
                InputEvent::ConfigTTS,
                InputEvent::Quit,
                InputEvent::NextPanel,
                InputEvent::PreviousPanel,
                InputEvent::Undo,
                InputEvent::Redo,
                InputEvent::StopTTS,
            ],
            input,
            text,
        );
        tts.insert(InputEvent::AppTTS, app);
        // File TTS.
        let file = get_tooltip(
            "FILE_TTS",
            &[
                InputEvent::NewFile,
                InputEvent::OpenFile,
                InputEvent::SaveFile,
                InputEvent::SaveFileAs,
                InputEvent::ExportFile,
            ],
            input,
            text,
        );
        tts.insert(InputEvent::FileTTS, file);
        // Config TTS.
        let config_tts = get_tooltip(
            "CONFIG_TTS",
            &[InputEvent::EditConfig, InputEvent::OverwriteConfig],
            input,
            text,
        );
        tts.insert(InputEvent::ConfigTTS, config_tts);
        let music_panel = MusicPanel::new(text);
        let tracks_panel = TracksPanel::new(input, text);
        let open_file_panel = OpenFilePanel::new(input, text);
        let piano_roll_panel = PianoRollPanel::new(&input_state.beat, config, text);
        Self {
            tts,
            music_panel,
            tracks_panel,
            open_file_panel,
            piano_roll_panel,
            redo: vec![],
            undo: vec![],
        }
    }

    pub fn update(
        &mut self,
        state: &mut State,
        conn: &mut Conn,
        input: &mut Input,
        tts: &mut TTS,
        text: &Text,
        paths: &Paths,
    ) -> bool {
        // Update the input state.
        input.update(state);

        // Quit.
        if input.happened(&InputEvent::Quit) {
            return true;
        }

        // Undo.
        if input.happened(&InputEvent::Undo) && !self.undo.is_empty() {
            // Get the first undo-redo state.
            let undo_redo = self.undo.remove(0);
            let redo = UndoRedoState::from((undo_redo.redo, &undo_redo.undo));
            // Assign the undo state to the current state.
            if let Some(s1) = undo_redo.undo.state {
                *state = s1;
            }
            // Send the commands.
            if let Some(commands) = undo_redo.undo.commands {
                conn.send(commands);
            }
            // Push to the redo stack.
            self.redo.push(redo);
        // Redo.
        } else if input.happened(&InputEvent::Redo) && !self.redo.is_empty() {
            // Get the first undo-redo state.
            let undo_redo = self.redo.remove(0);
            let undo = UndoRedoState::from((undo_redo.undo, &undo_redo.redo));
            // Assign the redo state to the current state.
            if let Some(s1) = undo_redo.redo.state {
                *state = s1;
            }
            // Send the commands.
            if let Some(commands) = undo_redo.redo.commands {
                conn.send(commands);
            }
            // Push to the undo stack.
            self.undo.push(undo);
        }

        // Cycle panels.
        if input.happened(&InputEvent::NextPanel) {
            let s0 = state.clone();
            state.focus.increment(true);
            self.push_state_to_undos(s0, state);
        } else if input.happened(&InputEvent::PreviousPanel) {
            let s0 = state.clone();
            state.focus.increment(false);
            self.push_state_to_undos(s0, state);
        }

        // App-level TTS.
        for tts_e in self.tts.iter() {
            if input.happened(tts_e.0) {
                tts.say(tts_e.1)
            }
        }
        // Stop talking.
        if input.happened(&InputEvent::StopTTS) {
            tts.stop();
        }

        // Listen to the focused panel.
        let resp = match state.panels[state.focus.get()] {
            PanelType::Music => self.music_panel.update(state, conn, input, tts, text),
            PanelType::Tracks => self.tracks_panel.update(state, conn, input, tts, text),
            PanelType::OpenFile => self.open_file_panel.update(state, conn, input, tts, text),
            PanelType::PianoRoll => self.piano_roll_panel.update(state, conn, input, tts, text),
            other => panic!("Not implemented: {:?}", other),
        };
        // Push an undo state generated by the focused panel.
        if let Some(undo) = resp {
            // Execute IO commands.
            if let Some(io_commands) = &undo.undo.io_commands {
                for command in io_commands {
                    match command {
                        IOCommand::EnableOpenFile(open_file_type) => match open_file_type {
                            OpenFileType::ReadSave => self.open_file_panel.read_save(paths, state),
                            OpenFileType::SoundFont => self.open_file_panel.soundfont(paths, state),
                            OpenFileType::WriteSave => {
                                self.open_file_panel.write_save(paths, state)
                            }
                        },
                        IOCommand::DisableOpenFile => {
                            self.open_file_panel.disable(state);
                        }
                    }
                }
            }
            // Push to the undo stack.
            if undo.undo.state.is_some() || undo.undo.commands.is_some() {
                self.push_undo(undo);
            }
        }
        // Try to update time itself.
        conn.update_time();

        // We're not done yet.
        false
    }

    /// Push this `State` to the undo stack and clear the redo stack.
    fn push_state_to_undos(&mut self, s0: State, s1: &State) {
        self.push_undo(UndoRedoState::from((s0, s1)));
    }

    /// Push this `UndoRedoState` to the undo stack and clear the redo stack.
    fn push_undo(&mut self, undo_redo: UndoRedoState) {
        self.undo.push(undo_redo);
        self.redo.clear();
        // Remove an undo if there are too many.
        if self.undo.len() > MAX_UNDOS {
            self.undo.remove(0);
        }
    }
}
