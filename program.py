from typing import List, Dict
import pygame.mixer
from cacophony.render.renderer import Renderer
from cacophony.render.panel.panel import Panel
from cacophony.render.panel.piano_roll import PianoRoll
from cacophony.render.panel.main_menu import MainMenu
from cacophony.render.panel.tracks_list import TracksList
from cacophony.render.panel.synthesizer_panel import SynthesizerPanel
from cacophony.render.panel.panel_type import PanelType
from cacophony.render.input_key import InputKey
from cacophony.render.globals import UI_AUDIO_GAIN
from cacophony.render.render_result import RenderResult
from cacophony.text_to_speech import TextToSpeech
from cacophony.synthesizer.clatter import Clatter
from cacophony.music.music import Music
from cacophony.util import tooltip


class Program:
    """
    Cacophony.
    """

    def __init__(self):
        """
        (no parameters)
        """

        # Create new empty music.
        self.music: Music = Music(bpm=60)
        self.renderer: Renderer = Renderer()
        main_menu: MainMenu = MainMenu()
        tracks_list: TracksList = TracksList(music=self.music)
        piano_roll: PianoRoll = PianoRoll(music=self.music, track_index=0, selected_note=0, time_0=0, note_0=60)
        synthesizer_panel: SynthesizerPanel = SynthesizerPanel(music=self.music, track_index=0)
        panels: List[Panel] = [main_menu, tracks_list, piano_roll, synthesizer_panel]
        self.panels: Dict[PanelType, Panel] = {panel.get_panel_type(): panel for panel in panels}
        self._panel_keys: List[PanelType] = list(self.panels.keys())
        self._panel_focus: int = 0
        self.app_help_text: str = Program.get_app_help_text()

    def run(self) -> None:
        """
        Run the program in a loop until the user quits.
        """

        renderer = Renderer()
        result = renderer.render([])
        while True:
            # Quit.
            if InputKey.quit_program in result.inputs_held:
                return
            # New file.
            elif InputKey.new_file in result.inputs_held:
                self.new_file()
            # Cycle between panels.
            elif InputKey.next_panel in result.inputs_pressed:
                self.cycle_panel_focus(increment=True)
            elif InputKey.previous_panel in result.inputs_pressed:
                self.cycle_panel_focus(increment=False)
            # Get help (seriously).
            if InputKey.app_help in result.inputs_pressed:
                TextToSpeech.say(self.app_help_text)
            elif InputKey.stop_tts in result.inputs_pressed:
                TextToSpeech.stop()
            # Render.
            result = self.render(result=result)

    def cycle_panel_focus(self, increment: bool) -> None:
        """
        Increment or decrement the panel focus.

        :param increment: If True, increment. If False, decrement.
        """

        # Re-initialize the panel that is about to lose focus.
        self.panels[self._panel_keys[self._panel_focus]].initialized = False
        # Cycle.
        if increment:
            self._panel_focus += 1
            if self._panel_focus >= len(self.panels):
                self._panel_focus = 0
        else:
            self._panel_focus -= 1
            if self._panel_focus < 0:
                self._panel_focus = len(self.panels) - 1
        # Re-initialize the panel that just gained focus.
        self.panels[self._panel_keys[self._panel_focus]].initialized = False
        # Plink!
        if UI_AUDIO_GAIN > 0:
            sound = pygame.mixer.Sound(Clatter.get_random())
            sound.set_volume(UI_AUDIO_GAIN)
            sound.play()

    def render(self, result: RenderResult) -> RenderResult:
        """
        Render the window. Render the focused panel.

        Check which other panels have been affected.

        Pass parameter-value pairs to the affected panels and render those panels.

        :param result: The `RenderResult` user input data from the previous frame.

        :return: A new `RenderResult`.
        """

        commands = []
        # Render the focused panel.
        commands.extend(self.panels[self._panel_keys[self._panel_focus]].render(result=result, focus=True))
        # Rerender all affected panels.
        for affected_panel in self.panels[self._panel_keys[self._panel_focus]].affected_panels:
            # Set attributes for the panels.
            for attr_key in self.panels[self._panel_keys[self._panel_focus]].affected_panels[affected_panel]:
                attr_value = self.panels[self._panel_keys[self._panel_focus]].affected_panels[affected_panel][attr_key]
                setattr(self.panels[affected_panel], attr_key, attr_value)
            # Mark the affected panel as uninitialized.
            self.panels[affected_panel].initialized = False
        # Rerender all uninitialized panels.
        for panel_type in self.panels:
            if not self.panels[panel_type].initialized:
                commands.extend(self.panels[panel_type].render(result=result, focus=False))
        # Render.
        return self.renderer.render(commands)

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

        self.music = Music(bpm=60)
        self.renderer = Renderer()
        main_menu = MainMenu()
        tracks_list = TracksList(music=self.music)
        piano_roll = PianoRoll(music=self.music, track_index=0, selected_note=0, time_0=0, note_0=60)
        synthesizer_panel = SynthesizerPanel(music=self.music, track_index=0)
        panels = [main_menu, tracks_list, piano_roll, synthesizer_panel]
        self.panels.clear()
        self.panels.update({panel.get_panel_type(): panel for panel in panels})
        self._panel_keys.clear()
        self._panel_keys.extend(list(self.panels.keys()))
        self._panel_focus = 0


if __name__ == "__main__":
    p = Program()
    p.run()
