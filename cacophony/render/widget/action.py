from typing import List, Tuple, Callable
from overrides import final
from pygame import Rect
from cacophony.render.commands.command import Command
from cacophony.render.commands.text import Text
from cacophony.render.commands.border import Border
from cacophony.render.color import Color
from cacophony.render.globals import COLORS
from cacophony.render.widget.widget import Widget
from cacophony.render.render_result import RenderResult
from cacophony.render.input_key import InputKey
from cacophony.util import tooltip


class Action(Widget):
    """
    A label with a callback.
    """

    def __init__(self, text: str, width: int, tts: str, callback: Callable[[], None]):
        """
        :param text: The label text.
        :param width: The width of the widget.
        :param tts: Text-to-speech text.
        :param callback: A callback to invoke on select.
        """

        super().__init__()
        self._text: str = text
        self._size: Tuple[int, int] = (width, 3)
        self._tts: str = tts
        self._callback: Callable[[], None] = callback

    @final
    def blit(self, position: Tuple[int, int], panel_focus: bool, element_focus: bool, pivot: Tuple[float, float] = None,
             anchor: Tuple[float, float] = None, parent_rect: Rect = None) -> List[Command]:
        if panel_focus:
            if element_focus:
                background_color = Color.border_focus
                text_color = Color.parameter_value
            else:
                background_color = Color.panel_background
                text_color = Color.border_focus
        else:
            background_color = Color.panel_background
            text_color = Color.border_no_focus
        commands = []
        # Show the border.
        if element_focus:
            commands.append(Border(position=position,
                                   size=self._size,
                                   color=COLORS[text_color],
                                   pivot=pivot,
                                   anchor=anchor,
                                   parent_rect=parent_rect))
        # Show the text.
        commands.append(Text(text=self._text,
                             position=(position[0] + 1, position[1] + 1),
                             text_color=COLORS[text_color],
                             background_color=COLORS[background_color],
                             pivot=pivot,
                             anchor=anchor,
                             parent_rect=parent_rect))
        return commands

    @final
    def get_size(self) -> Tuple[int, int]:
        return self._size

    @final
    def do(self, result: RenderResult) -> bool:
        if InputKey.select in result.inputs_pressed:
            self._callback()
            return True
        return False

    @final
    def get_help_text(self) -> str:
        return tooltip(keys=[InputKey.select], predicate=self._tts)
