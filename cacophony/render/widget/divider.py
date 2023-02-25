from typing import List, Tuple
from overrides import final
from pygame import Rect
from cacophony.render.commands.command import Command
from cacophony.render.commands.text import Text
from cacophony.render.commands.line import Line
from cacophony.render.color import Color
from cacophony.render.globals import COLORS
from cacophony.render.widget.widget import Widget
from cacophony.render.render_result import RenderResult


class Divider(Widget):
    """
    A section divider.
    """

    _FOREGROUND_COLOR: Tuple[int, int, int] = COLORS[Color.border_no_focus]
    _BACKGROUND_COLOR: Tuple[int, int, int] = COLORS[Color.panel_background]

    def __init__(self, text: str, width: int):
        """
        :param text: The label text.
        :param width: The width of the divider in grid coordinates.
        """

        super().__init__()
        self._text: str = text
        self._width: int = width

    def blit(self, position: Tuple[int, int], panel_focus: bool, element_focus: bool, pivot: Tuple[float, float] = None,
             anchor: Tuple[float, float] = None, parent_rect: Rect = None) -> List[Command]:
        return [Line(position=position,
                     length=self._width,
                     color=Divider._FOREGROUND_COLOR,
                     vertical=False,
                     pivot=pivot,
                     anchor=anchor,
                     parent_rect=parent_rect),
                Text(text=self._text,
                     position=(position[0] + 1, position[1]),
                     text_color=Divider._FOREGROUND_COLOR,
                     background_color=Divider._BACKGROUND_COLOR,
                     pivot=pivot,
                     anchor=anchor,
                     parent_rect=parent_rect)]

    @final
    def get_size(self) -> Tuple[int, int]:
        return self._width, 1

    def do(self, result: RenderResult) -> bool:
        return False

    def get_help_text(self) -> str:
        return ""
