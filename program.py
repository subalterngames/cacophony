from typing import List, Dict
import pygame.mixer
from cacophony.render.renderer import Renderer
from cacophony.render.panel.panel import Panel
from cacophony.render.panel.piano_roll import PianoRoll
from cacophony.render.panel.main_menu import MainMenu
from cacophony.render.panel.tracks_list import TracksList
from cacophony.render.panel.synthesizer_panel import SynthesizerPanel
from cacophony.render.panel.new_track import NewTrack
from cacophony.render.panel.panel_type import PanelType
from cacophony.render.panel.open_file import OpenFile
from cacophony.render.input_key import InputKey
from cacophony.render.globals import UI_AUDIO_GAIN
from cacophony.text_to_speech import TextToSpeech
from cacophony.synthesizer.clatter import Clatter
from cacophony.music.music import Music
from cacophony.music.note import Note
from cacophony.util import tooltip, note_on
from cacophony.state import State
from cacophony.piano_roll_state import PianoRollState
from cacophony.open_file_state import OpenFileState


class Program:
    """
    Cacophony.
    """

    def __init__(self):
        """
        (no parameters)
        """

        self.renderer: Renderer = Renderer()
        self.panels: Dict[PanelType, Panel] = dict()
        self.app_help_text: str = Program.get_app_help_text()
        self.state: State = State(music=Music(bpm=60),
                                  track_index=0,
                                  focused_panel=PanelType.main_menu,
                                  open_file_state=OpenFileState(suffixes=[],
                                                                previous_focus=PanelType.main_menu),
                                  piano_roll_state=PianoRollState(time_0=0,
                                                                  note_0=60,
                                                                  selected_note_index=0))
        self.new_file()

    def run(self) -> None:
        """
        Run the program in a loop until the user quits.
        """

        renderer = Renderer()
        self.state.result = renderer.render([])
        while True:
            # Quit.
            if InputKey.quit_program in self.state.result.inputs_held:
                return
            # New file.
            elif InputKey.new_file in self.state.result.inputs_held:
                self.new_file()
            # Cycle between panels.
            elif InputKey.next_panel in self.state.result.inputs_pressed:
                self.cycle_panel_focus(increment=True)
            elif InputKey.previous_panel in self.state.result.inputs_pressed:
                self.cycle_panel_focus(increment=False)
            # Get help (seriously).
            if InputKey.app_help in self.state.result.inputs_pressed:
                TextToSpeech.say(self.app_help_text)
            elif InputKey.stop_tts in self.state.result.inputs_pressed:
                TextToSpeech.stop()
            # Render.
            commands = []
            # Render all active panels.
            for panel_type in self.state.active_panels:
                commands.extend(self.panels[panel_type].render(state=self.state,
                                                               focus=panel_type == self.state.focused_panel))
            # Render all dirty panels.
            dirty_panels = list(set(self.state.dirty_panels))
            for panel_type in dirty_panels:
                self.panels[panel_type].do_render = True
                commands.extend(self.panels[panel_type].render(state=self.state,
                                                               focus=panel_type == self.state.focused_panel))
            # Clear the dirty panels.
            self.state.dirty_panels.clear()
            # Render.
            self.state.result = self.renderer.render(commands)
            # Play any MIDI notes.
            self.play_midi_notes()

    def cycle_panel_focus(self, increment: bool) -> None:
        """
        Increment or decrement the panel focus.

        :param increment: If True, increment. If False, decrement.
        """

        # Re-initialize the panel that is about to lose focus.
        self.state.dirty_panels.append(self.state.focused_panel)
        active_panels = [panel for panel in self.panels if panel in self.state.active_panels]
        # Can't cycle.
        if len(active_panels) <= 1:
            return
        panel_focus = active_panels.index(self.state.focused_panel)
        # Cycle.
        if increment:
            panel_focus += 1
            if panel_focus >= len(active_panels):
                panel_focus = 0
        else:
            panel_focus -= 1
            if panel_focus < 0:
                panel_focus = len(active_panels) - 1
        # Set the focused panel.
        self.state.focused_panel = active_panels[panel_focus]
        self.state.dirty_panels.append(self.state.focused_panel)
        # Plink!
        if UI_AUDIO_GAIN > 0:
            sound = pygame.mixer.Sound(Clatter.get_random())
            sound.set_volume(UI_AUDIO_GAIN)
            sound.play()

    @staticmethod
    def get_app_help_text() -> str:
        """
        :return: The help text for the whole app.
        """

        text = "Hello world. I am Casey the Cacodemon. "
        tooltips = [tooltip(keys=[InputKey.next_panel, InputKey.previous_panel], predicate="cycle through panels.", boop="and"),
                    tooltip(keys=[InputKey.panel_help], predicate="ask me to tell you what the current panel does."),
                    tooltip(keys=[InputKey.widget_help], predicate="ask me to tell you what the current widget does."),
                    tooltip(keys=[InputKey.new_file], predicate="new file."),
                    tooltip(keys=[InputKey.open_file], predicate="open file."),
                    tooltip(keys=[InputKey.save_file], predicate="save."),
                    tooltip(keys=[InputKey.undo], predicate="undo."),
                    tooltip(keys=[InputKey.quit_program], predicate="quit."),
                    tooltip(keys=[InputKey.app_help], predicate="ask me to say this message again."),
                    tooltip(keys=[InputKey.stop_tts], predicate="tell me to stop talking.")]
        text += " ".join(tooltips)
        return text

    def new_file(self) -> None:
        """
        Create a new file.
        """

        # Create the panels.
        main_menu: MainMenu = MainMenu()
        tracks_list: TracksList = TracksList()
        piano_roll: PianoRoll = PianoRoll()
        synthesizer_panel: SynthesizerPanel = SynthesizerPanel()
        new_track: NewTrack = NewTrack()
        open_file: OpenFile = OpenFile()
        panels: List[Panel] = [main_menu, tracks_list, piano_roll, synthesizer_panel, new_track, open_file]
        self.panels.clear()
        self.panels.update({panel.get_panel_type(): panel for panel in panels})
        # Create a new renderer.
        self.renderer = Renderer()
        # Create a new state.
        self.state: State = State(music=Music(bpm=60),
                                  track_index=0,
                                  focused_panel=PanelType.main_menu,
                                  open_file_state=OpenFileState(suffixes=[],
                                                                previous_focus=PanelType.main_menu),
                                  piano_roll_state=PianoRollState(time_0=0,
                                                                  note_0=60,
                                                                  selected_note_index=0))
        # Set the active panels.
        self.state.active_panels.extend([PanelType.main_menu, PanelType.tracks_list, PanelType.piano_roll, PanelType.synthesizer_panel])
        self.state.dirty_panels.extend(self.state.active_panels)

    def play_midi_notes(self) -> None:
        """
        Play notes from a MIDI controller without recording them.
        """

        if self.state.track_index >= len(self.state.music.tracks):
            return
        # Play the notes.
        beat = self.state.music.tracks[self.state.track_index].synthesizer.beat.get().value
        for i in range(len(self.state.result.midi)):
            # Play the note.
            if note_on(midi_event=self.state.result.midi[i]):
                note = Note(self.state.result.midi[i][1], start=0, duration=beat, volume=self.state.result.midi[i][2])
                a = self.state.music.tracks[self.state.track_index].synthesizer.audio(note=note, bpm=self.state.music.bpm)
                sound = pygame.mixer.Sound(a)
                sound.play()


if __name__ == "__main__":
    p = Program()
    p.run()
