from typing import List
import pygame.mixer
from cacophony.render.renderer import Renderer
from cacophony.render.panel.panel import Panel
from cacophony.render.panel.piano_roll import PianoRoll
from cacophony.render.panel.main_menu import MainMenu
from cacophony.render.panel.tracks_list import TracksList
from cacophony.render.panel.synthesizer_panel import SynthesizerPanel
from cacophony.render.input_key import InputKey
from cacophony.render.globals import UI_AUDIO_GAIN
from cacophony.render.render_result import RenderResult
from cacophony.text_to_speech import TextToSpeech
from cacophony.synthesizer.clatter import Clatter
from cacophony.music.music import Music
from cacophony.util import tooltip


class Program:
    def __init__(self):
        # Create new empty music.
        self.music: Music = Music(bpm=60)
        self.renderer: Renderer = Renderer()
        self.main_menu: MainMenu = MainMenu()
        self.tracks_list: TracksList = TracksList(music=self.music)
        self.piano_roll: PianoRoll = PianoRoll(music=self.music, track_index=0, selected_note=0, time_0=0, note_0=60)
        self.synthesizer_panel: SynthesizerPanel = SynthesizerPanel(music=self.music, track_index=0)
        self.panels: List[Panel] = [self.main_menu, self.tracks_list, self.piano_roll, self.synthesizer_panel]
        self.panel_focus: int = 0
        self.app_help_text: str = Program.get_app_help_text()

    def run(self) -> None:
        renderer = Renderer()
        result = renderer.render([])
        while True:
            # Quit.
            if InputKey.quit_program in result.inputs_pressed:
                return
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
        self.panels[self.panel_focus].initialized = False
        # Cycle.
        if increment:
            self.panel_focus += 1
            if self.panel_focus >= len(self.panels):
                self.panel_focus = 0
        else:
            self.panel_focus -= 1
            if self.panel_focus < 0:
                self.panel_focus = len(self.panels) - 1
        # Re-initialize the panel that just gained focus.
        self.panels[self.panel_focus].initialized = False
        # Plink!
        if UI_AUDIO_GAIN > 0:
            sound = pygame.mixer.Sound(Clatter.get_random())
            sound.set_volume(UI_AUDIO_GAIN)
            sound.play()

    def render(self, result: RenderResult) -> RenderResult:
        commands = []
        # Render the main menu.
        commands.extend(self.main_menu.render(result=result, focus=self.panel_focus == 0))
        # Remember the track index.
        track_index = self.tracks_list.selection_index
        # Render the tracks list.
        commands.extend(self.tracks_list.render(result=result, focus=self.panel_focus == 1))
        # The selected track changed.
        if track_index != self.tracks_list.selection_index:
            self.piano_roll.track_index = self.tracks_list.selection_index
            self.piano_roll.initialized = False
            self.synthesizer_panel.track_index = self.tracks_list.selection_index
            self.synthesizer_panel.initialized = False
        # Render the piano roll.
        commands.extend(self.piano_roll.render(result=result, focus=self.panel_focus == 2))
        # Render the synthesizer panel.
        commands.extend(self.synthesizer_panel.render(result=result, focus=self.panel_focus == 3))
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


if __name__ == "__main__":
    p = Program()
    p.run()
