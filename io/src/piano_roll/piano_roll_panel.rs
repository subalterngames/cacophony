use super::*;
use crate::panel::*;
use crate::{combine_tracks_to_commands, select_track};
use common::config::parse_fractions;
use common::{Index, Note, PianoRollMode, SelectMode, U64orF32, PPQ_F};
use ini::Ini;

/// The piano roll.
/// This is divided into different "modes" for convenience, where each mode is actually a panel.
pub struct PianoRollPanel {
    /// The edit mode.
    edit: Edit,
    /// The select mode.
    select: Select,
    /// The time mode.
    time: Time,
    /// The view mode.
    view: View,
    /// The beats that we can potentially input as PPQ values.
    beats: Vec<u64>,
    /// The index of the current beat.
    beat: Index,
    /// A buffer of copied notes.
    copied_notes: Vec<Note>,
}

impl PianoRollPanel {
    pub fn new(beat: &u64, config: &Ini) -> Self {
        let edit = Edit::new(config);
        let select = Select {};
        let time = Time::new(config);
        let view = View::new(config);
        // Load the beats.
        let section = config.section(Some("PIANO_ROLL")).unwrap();
        let mut beats: Vec<u64> = parse_fractions(section, "beats")
            .iter()
            .map(|f| (*f * PPQ_F) as u64)
            .collect();
        // Is the input beat in the list?
        let beat_index = match beats.iter().position(|b| b == beat) {
            Some(position) => position,
            None => {
                beats.push(*beat);
                beats.len() - 1
            }
        };
        let beat = Index::new(beat_index, beats.len());
        Self {
            edit,
            select,
            time,
            view,
            beats,
            beat,
            copied_notes: vec![],
        }
    }

    /// Set the input beat.
    fn set_input_beat(&mut self, up: bool, state: &mut State) -> Option<Snapshot> {
        let s0 = state.clone();
        // Increment the beat.
        self.beat.increment(up);
        // Set the input beat.
        state.input.beat = U64orF32::from(self.beats[self.beat.get()]);
        Some(Snapshot::from_states(s0, state))
    }

    /// Set the piano roll mode.
    fn set_mode(mode: PianoRollMode, state: &mut State) -> Option<Snapshot> {
        let s0 = state.clone();
        state.piano_roll_mode = mode;
        Some(Snapshot::from_states(s0, state))
    }

    /// Returns the text-to-speech string that will be said if there is no valid track.
    fn tts_no_track(text: &Text) -> Vec<TtsString> {
        vec![TtsString::from(text.get("PIANO_ROLL_PANEL_TTS_NO_TRACK"))]
    }

    /// Returns the sub-panel corresponding to the current piano roll mode.
    fn get_sub_panel<'a>(&'a self, state: &State) -> &'a dyn PianoRollSubPanel {
        match state.piano_roll_mode {
            PianoRollMode::Edit => &self.edit,
            PianoRollMode::Select => &self.select,
            PianoRollMode::Time => &self.time,
            PianoRollMode::View => &self.view,
        }
    }

    /// Copy the selected notes to the copy buffer.
    fn copy_notes(&mut self, state: &State) {
        if let Some(notes) = state.select_mode.get_notes(&state.music) {
            self.copied_notes = notes.iter().map(|&n| *n).collect()
        }
    }

    /// Delete notes from the track.
    fn delete_notes(state: &mut State) -> Option<Snapshot> {
        // Clone the state.
        let s0 = state.clone();
        if let Some(indices) = state.select_mode.get_note_indices() {
            if let Some(track) = state.music.get_selected_track_mut() {
                // Remove the notes.
                track.notes = track
                    .notes
                    .iter()
                    .enumerate()
                    .filter(|n| !indices.contains(&n.0))
                    .map(|n| *n.1)
                    .collect();
                // Deselect.
                state.select_mode = match &state.select_mode {
                    SelectMode::Single(_) => SelectMode::Single(None),
                    SelectMode::Many(_) => SelectMode::Many(None),
                };
                // Return the undo state.
                return Some(Snapshot::from_states(s0, state));
            }
        }
        None
    }
}

impl Panel for PianoRollPanel {
    fn update(
        &mut self,
        state: &mut State,
        conn: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &mut Text,
        paths_state: &mut PathsState,
        exporter: &mut SharedExporter,
    ) -> Option<Snapshot> {
        // Select a track.
        if !state.view.single_track {
            if let Some(snapshot) = select_track(state, input) {
                return Some(snapshot);
            }
        }
        // Do nothing.
        if state.music.selected.is_none() {
            None
        }
        // Add notes.
        else if state.input.armed && !input.new_notes.is_empty() {
            // Clone the state.
            let s0 = state.clone();
            let track = state.music.get_selected_track_mut().unwrap();
            match conn.state.programs.get(&track.channel) {
                Some(_) => {
                    // Get the notes.
                    let notes: Vec<Note> = input
                        .new_notes
                        .iter()
                        .map(|n| Note {
                            note: n[1],
                            velocity: n[2],
                            start: state.time.cursor,
                            end: state.time.cursor + state.input.beat.get_u(),
                        })
                        .collect();
                    // Add the notes.
                    track.notes.extend(notes.iter().copied());
                    // Move the cursor.
                    state.time.cursor += state.input.beat.get_u();
                    Some(Snapshot::from_states(s0, state))
                }
                None => None,
            }
        }
        // Status TTS.
        else if input.happened(&InputEvent::StatusTTS) {
            let mut tts_strings = vec![];
            match state.music.get_selected_track() {
                Some(track) => match conn.state.programs.get(&track.channel) {
                    Some(_) => {
                        // The piano roll mode.
                        tts_strings.push(TtsString::from(text.get_with_values(
                            "PIANO_ROLL_PANEL_STATUS_TTS_PIANO_ROLL_MODE",
                            &[&text.get_piano_roll_mode(&state.piano_roll_mode)],
                        )));
                        match state.input.armed {
                            // The beat and the volume.
                            true => {
                                let beat = text.get_ppq_tts(&state.input.beat.get_u());
                                let v = state.input.volume.get().to_string();
                                let volume = if state.input.use_volume {
                                    v
                                } else {
                                    text.get_with_values(
                                        "PIANO_ROLL_PANEL_STATUS_TTS_VOLUME",
                                        &[&v],
                                    )
                                };
                                tts_strings.push(TtsString::from(text.get_with_values(
                                    "PIANO_ROLL_PANEL_STATUS_TTS_ARMED",
                                    &[&beat, &volume],
                                )));
                            }
                            // Not armed.
                            false => tts_strings.push(TtsString::from(
                                text.get("PIANO_ROLL_PANEL_STATUS_TTS_NOT_ARMED"),
                            )),
                        }
                        // How many tracks?
                        let tracks_key = if state.view.single_track {
                            "PIANO_ROLL_PANEL_STATUS_TTS_SINGLE_TRACK"
                        } else {
                            "PIANO_ROLL_PANEL_STATUS_TTS_MULTI_TRACK"
                        };
                        tts_strings.push(TtsString::from(text.get_with_values(
                            tracks_key,
                            &[&state.music.selected.unwrap().to_string()],
                        )));
                        // Panel-specific status.
                        tts_strings
                            .append(&mut self.get_sub_panel(state).get_status_tts(state, text));
                    }
                    None => tts_strings.append(&mut PianoRollPanel::tts_no_track(text)),
                },
                None => tts_strings.append(&mut PianoRollPanel::tts_no_track(text)),
            };
            tts.enqueue(tts_strings);
            None
        }
        // Input TTS.
        else if input.happened(&InputEvent::InputTTS) {
            let s = match state.music.get_selected_track() {
                Some(track) => match conn.state.programs.get(&track.channel) {
                    // Here we go...
                    Some(_) => {
                        let mut tts_strings = vec![text.get_tooltip(
                            "PIANO_ROLL_PANEL_INPUT_TTS_PLAY",
                            &[InputEvent::PlayStop],
                            input,
                        )];
                        // Armed state, beat, volume.
                        match state.input.armed {
                            true => {
                                tts_strings.push(text.get_tooltip(
                                    "PIANO_ROLL_PANEL_INPUT_TTS_ARMED",
                                    &[
                                        InputEvent::Arm,
                                        InputEvent::InputBeatLeft,
                                        InputEvent::InputBeatRight,
                                    ],
                                    input,
                                ));
                                match state.input.use_volume {
                                    true => tts_strings.push(text.get_tooltip(
                                        "PIANO_ROLL_PANEL_INPUT_TTS_DO_NOT_USE_VOLUME",
                                        &[
                                            InputEvent::DecreaseInputVolume,
                                            InputEvent::IncreaseInputVolume,
                                            InputEvent::ToggleInputVolume,
                                        ],
                                        input,
                                    )),
                                    false => tts_strings.push(text.get_tooltip(
                                        "PIANO_ROLL_PANEL_INPUT_TTS_USE_VOLUME",
                                        &[InputEvent::ToggleInputVolume],
                                        input,
                                    )),
                                }
                            }
                            false => tts_strings.push(text.get_tooltip(
                                "PIANO_ROLL_PANEL_INPUT_TTS_NOT_ARMED",
                                &[InputEvent::Arm],
                                input,
                            )),
                        }
                        // Toggle tracks.
                        let tracks_key = if state.view.single_track {
                            "PIANO_ROLL_PANEL_INPUT_TTS_MULTI_TRACK"
                        } else {
                            "PIANO_ROLL_PANEL_INPUT_TTS_SINGLE_TRACK"
                        };
                        tts_strings.push(text.get_tooltip(
                            tracks_key,
                            &[InputEvent::PianoRollToggleTracks],
                            input,
                        ));
                        // Multi-track scroll.
                        if !state.view.single_track {
                            tts_strings.push(text.get_tooltip(
                                "PIANO_ROLL_PANEL_INPUT_TTS_TRACK_SCROLL",
                                &[
                                    InputEvent::PianoRollPreviousTrack,
                                    InputEvent::PianoRollNextTrack,
                                ],
                                input,
                            ));
                        }
                        // Change the mode.
                        tts_strings.push(text.get_tooltip(
                            "PIANO_ROLL_PANEL_INPUT_TTS_MODES",
                            &[
                                InputEvent::PianoRollSetTime,
                                InputEvent::PianoRollSetView,
                                InputEvent::PianoRollSetSelect,
                                InputEvent::PianoRollSetEdit,
                            ],
                            input,
                        ));
                        // Cut, copy.
                        let selected_some = state.select_mode.get_note_indices().is_some();
                        if selected_some {
                            tts_strings.push(text.get_tooltip(
                                "PIANO_ROLL_PANEL_INPUT_TTS_COPY_CUT",
                                &[InputEvent::CopyNotes, InputEvent::CutNotes],
                                input,
                            ));
                        }
                        // Paste.
                        if !self.copied_notes.is_empty() {
                            tts_strings.push(text.get_tooltip(
                                "PIANO_ROLL_PANEL_INPUT_TTS_PASTE",
                                &[InputEvent::PasteNotes],
                                input,
                            ));
                        }
                        // Delete.
                        if selected_some {
                            tts_strings.push(text.get_tooltip(
                                "PIANO_ROLL_PANEL_INPUT_TTS_DELETE",
                                &[InputEvent::DeleteNotes],
                                input,
                            ));
                        }
                        // Sub-panel inputs.
                        tts_strings.append(
                            &mut self.get_sub_panel(state).get_input_tts(state, input, text),
                        );
                        tts_strings
                    }
                    None => PianoRollPanel::tts_no_track(text),
                },
                None => PianoRollPanel::tts_no_track(text),
            };
            tts.enqueue(s);
            None
        }
        // Play and stop music.
        else if input.happened(&InputEvent::PlayStop) {
            match conn.state.time.music {
                // Stop playing.
                true => conn.send(vec![Command::StopMusic]),
                false => {
                    conn.send(
                        combine_tracks_to_commands(state, conn.framerate, state.time.playback).0,
                    );
                }
            }
            None
        }
        // Copy notes.
        else if input.happened(&InputEvent::CopyNotes) {
            self.copy_notes(state);
            None
        }
        // Cut notes.
        else if input.happened(&InputEvent::CutNotes) {
            // Copy.
            self.copy_notes(state);
            // Delete.
            PianoRollPanel::delete_notes(state)
        }
        // Delete notes.
        else if input.happened(&InputEvent::DeleteNotes) {
            PianoRollPanel::delete_notes(state)
        }
        // Paste notes.
        else if input.happened(&InputEvent::PasteNotes) {
            if !self.copied_notes.is_empty() {
                // Clone the state.
                let s0 = state.clone();
                if let Some(track) = state.music.get_selected_track_mut() {
                    // Get the minimum start time.
                    let min_time = self
                        .copied_notes
                        .iter()
                        .min_by(|a, b| a.start.cmp(&b.start))
                        .unwrap()
                        .start;
                    // Adjust the start and end time.
                    let mut notes = self.copied_notes.to_vec();
                    notes.iter_mut().for_each(|n| {
                        let dt = n.end - n.start;
                        n.start = (n.start - min_time) + state.time.cursor;
                        n.end = n.start + dt;
                    });
                    // Add the notes.
                    track.notes.append(&mut notes);
                    // Return the undo state.
                    Some(Snapshot::from_states(s0, state))
                } else {
                    None
                }
            } else {
                None
            }
        }
        // Toggle arm.
        else if input.happened(&InputEvent::Arm) {
            let s0 = state.clone();
            state.input.armed = !state.input.armed;
            Some(Snapshot::from_states(s0, state))
        }
        // Toggle tracks view.
        else if input.happened(&InputEvent::PianoRollToggleTracks) {
            Some(Snapshot::from_state_value(
                |s| &mut s.view.single_track,
                !state.view.single_track,
                state,
            ))
        }
        // Set the input beat.
        else if input.happened(&InputEvent::InputBeatLeft) {
            self.set_input_beat(false, state)
        } else if input.happened(&InputEvent::InputBeatRight) {
            self.set_input_beat(true, state)
        }
        // Set the volume.
        else if input.happened(&InputEvent::ToggleInputVolume) {
            Some(Snapshot::from_state_value(
                |s| &mut s.input.use_volume,
                !state.input.use_volume,
                state,
            ))
        } else if input.happened(&InputEvent::DecreaseInputVolume) {
            Some(Snapshot::from_state(
                |s| s.input.volume.increment(false),
                state,
            ))
        } else if input.happened(&InputEvent::IncreaseInputVolume) {
            Some(Snapshot::from_state(
                |s| s.input.volume.increment(true),
                state,
            ))
        }
        // Set the mode.
        else if input.happened(&InputEvent::PianoRollSetEdit) {
            PianoRollPanel::set_mode(PianoRollMode::Edit, state)
        } else if input.happened(&InputEvent::PianoRollSetSelect) {
            PianoRollPanel::set_mode(PianoRollMode::Select, state)
        } else if input.happened(&InputEvent::PianoRollSetTime) {
            PianoRollPanel::set_mode(PianoRollMode::Time, state)
        } else if input.happened(&InputEvent::PianoRollSetView) {
            PianoRollPanel::set_mode(PianoRollMode::View, state)
        } else {
            // Sub-panel actions.
            let mode = state.piano_roll_mode;
            match mode {
                PianoRollMode::Edit => {
                    self.edit
                        .update(state, conn, input, tts, text, paths_state, exporter)
                }
                PianoRollMode::Select => {
                    self.select
                        .update(state, conn, input, tts, text, paths_state, exporter)
                }
                PianoRollMode::Time => {
                    self.time
                        .update(state, conn, input, tts, text, paths_state, exporter)
                }
                PianoRollMode::View => {
                    self.view
                        .update(state, conn, input, tts, text, paths_state, exporter)
                }
            }
        }
    }
}
