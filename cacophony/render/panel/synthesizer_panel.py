from abc import ABC, abstractmethod
from typing import List, Tuple, TypeVar, Generic
from overrides import final
from pygame import Rect
from pygame.mixer import Sound
from cacophony.render.commands.command import Command
from cacophony.render.globals import LAYOUTS
from cacophony.render.panel.panel import Panel
from cacophony.render.input_key import InputKey
from cacophony.render.render_result import RenderResult
from cacophony.render.ui_element.ui_element import UiElement
from cacophony.render.ui_element.boolean import Boolean
from cacophony.render.ui_element.options import Options
from cacophony.render.ui_element.i127 import I127
from cacophony.render.macros.parent_rect import get_parent_rect
from cacophony.music.music import Music
from cacophony.music.note import Note
from cacophony.synthesizer.synthesizer import Synthesizer
from cacophony.util import tooltip, note_on


T = TypeVar("T", bound=Synthesizer)


class SynthesizerPanel(Panel, Generic[T], ABC):
    """
    A panel with synthesizer parameters. This is an abstract class.
    """

    def __init__(self, music: Music, track_index: int):
        """
        :param music: The music.
        :param track_index: The track index.
        """

        self.music: Music = music
        self.track_index: int = track_index
        layout: Tuple[int, int, int, int] = LAYOUTS["SynthesizerPanel"]
        super().__init__(title=self.__get_title(), position=(layout[0], layout[1]), size=(layout[2], layout[3]))
        self.volume_boolean: Boolean = Boolean(text="MIDI-In Volume", value=False)
        self.volume: I127 = I127(title="Volume", index=127)
        self.beat: Options = Options(title="Beat",
                                     options=["1/8", "1/6", "1/4", "1/3", "1/2", "1", "1.5", "2", "3", "4"],
                                     index=5)
        self.widgets: List[UiElement] = [self.volume_boolean, self.volume, self.beat]
        self.focus_index: int = 0
        self._parent_rect: Rect = get_parent_rect(position=self._position, size=self._size)

    @final
    def _do_result(self, result: RenderResult) -> bool:
        # Edit all volumes.
        if not self.volume_boolean.value:
            volume = int(self.volume.options[self.volume.index])
            for i in range(len(result.midi)):
                if note_on(midi_event=result.midi[i]):
                    result.midi[i][2] = volume
        rerender = False
        # Set the volume boolean.
        if self.focus_index == 0 and InputKey.select in result.inputs_pressed:
            self.volume_boolean.value = not self.volume_boolean.value
            rerender = True
        # Change the volume.
        elif self.focus_index == 1:
            if InputKey.left in result.inputs_pressed:
                self.volume.cycle(False)
                rerender = True
            if InputKey.right in result.inputs_pressed:
                self.volume.cycle(True)
                rerender = True
        # Change the beat.
        elif self.focus_index == 2:
            if InputKey.left in result.inputs_pressed:
                self.beat.cycle(False)
                rerender = True
            if InputKey.right in result.inputs_pressed:
                self.beat.cycle(True)
                rerender = True
        sr = self._do_synthesizer_result(result=result)
        if sr:
            rerender = True
        # Get the beat.
        beat_fr = self.beat.options[self.beat.index].split("/")
        if len(beat_fr) == 1:
            beat = round(float(beat_fr[0]), 6)
        else:
            beat = round(float(beat_fr[0]) / float(beat_fr[1]), 6)
        # Play the notes.
        for i in range(len(result.midi)):
            # Play the note.
            if note_on(midi_event=result.midi[i]):
                note = Note(result.midi[i][1], start=0, duration=beat, volume=result.midi[i][2])
                a = self.music.tracks[self.track_index].synthesizer.audio(note=note, bpm=self.music.bpm)
                sound = Sound(a)
                sound.play()
        return rerender

    @final
    def _render_panel(self, focus: bool) -> List[Command]:
        # Blit the panel.
        commands = super()._render_panel(focus=focus)
        y = 1
        x = 1
        for widget in self.widgets:
            commands.extend(widget.blit(position=(x, y),
                                        panel_focus=focus,
                                        element_focus=self.focus_index == 0,
                                        parent_rect=self._parent_rect))
            y += widget.get_size()[1]
        return commands

    @final
    def get_panel_help(self) -> str:
        return self.__get_title() + ". " + tooltip(keys=[InputKey.up, InputKey.down], predicate="scroll", boop="and")

    def get_widget_help(self) -> str:
        if self.focus_index == 0:
            if self.volume_boolean.value:
                return tooltip(keys=[InputKey.select],
                               predicate="use the volume data from your MIDI controller to set the volume of each note.")
            else:
                return tooltip(keys=[InputKey.select],
                               predicate="use the volume value in the field below this one for new notes in this track.")
        elif self.focus_index == 1:
            if self.volume_boolean.value:
                return "Set volume. This won't do anything because you are using MIDI-In Volume."
            else:
                return tooltip(keys=[InputKey.left, InputKey.right], predicate="set the volume for new notes in this track.", boop="and")
        elif self.focus_index == 2:
            return tooltip(keys=[InputKey.left, InputKey.right], predicate="set the beat for new notes in this track.", boop="and")
        else:
            return ""

    @abstractmethod
    def _do_synthesizer_result(self, result: RenderResult) -> bool:
        """
        Handle input for synthesizer-specific fields.

        :param result: The `RenderResult`.

        :return: True if we need to rerender.
        """

        raise Exception()

    def __get_title(self) -> str:
        return f"{self.track_index} {self.music.tracks[self.track_index].synthesizer.__class__.__name__}"

